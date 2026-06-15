// SPDX-FileCopyrightText: Copyright (c) 2026 owu <wqh@live.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::time::Duration;
use tokio::time::Instant;
use futures_util::StreamExt;
use reqwest::header::RANGE;
use tracing::{info, warn, debug};

use crate::api::models::MirrorSource;

// Probe TCP connection latency (ms) as fallback when throughput probing fails
async fn probe_tcp_latency(domain: &str) -> u64 {
    let start = Instant::now();
    let addr = format!("{}:443", domain);

    match tokio::time::timeout(
        Duration::from_secs(3),
        tokio::net::TcpStream::connect(&addr)
    ).await {
        Ok(Ok(_)) => {
            let ms = start.elapsed().as_millis() as u64;
            debug!("TCP latency probe {}: {}ms", domain, ms);
            ms
        }
        _ => {
            debug!("TCP latency probe {}: unreachable", domain);
            u64::MAX
        }
    }
}

// Probe actual download throughput (bytes/sec) of a mirror
// Downloads the first 3MB via Range request to measure real transfer capability
// On timeout, tokio::time::timeout cancels the future; drop response/stream auto-closes connection
async fn probe_mirror_throughput(source: &MirrorSource) -> u64 {
    const PROBE_SIZE: u64 = 3 * 1024 * 1024; // 3MB
    const TIMEOUT: Duration = Duration::from_secs(6);

    let client = match reqwest::Client::builder()
        .connect_timeout(Duration::from_secs(5))
        .no_gzip()
        .no_brotli()
        .no_deflate()
        .build() {
            Ok(c) => c,
            Err(_) => return 0,
        };

    let req = client
        .get(&source.url)
        .header(RANGE, format!("bytes=0-{}", PROBE_SIZE - 1));

    let start = Instant::now();

    let result = tokio::time::timeout(TIMEOUT, async {
        let resp = req.send().await?;
        let status = resp.status();
        if !status.is_success() && status.as_u16() != 206 {
            return Err(resp.error_for_status().unwrap_err());
        }
        let mut total = 0u64;
        let mut stream = resp.bytes_stream();
        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            total += chunk.len() as u64;
            if total >= PROBE_SIZE {
                break;
            }
        }
        Ok::<_, reqwest::Error>(total)
    })
    .await;

    match result {
        Ok(Ok(bytes_downloaded)) if bytes_downloaded > 0 => {
            let elapsed_secs = start.elapsed().as_secs_f64();
            if elapsed_secs > 0.0 {
                let throughput = (bytes_downloaded as f64 / elapsed_secs) as u64;
                debug!(
                    "Mirror throughput probe {}: {} bytes in {:.2}s = {:.1} KB/s",
                    source.mirror,
                    bytes_downloaded,
                    elapsed_secs,
                    throughput as f64 / 1024.0
                );
                throughput
            } else {
                u64::MAX
            }
        }
        _ => {
            debug!("Mirror throughput probe {}: failed/timeout", source.mirror);
            0
        }
    }
}

// Extract domain from URL
fn extract_domain(url: &str) -> String {
    if let Ok(parsed) = reqwest::Url::parse(url) {
        parsed.host_str().unwrap_or("").to_string()
    } else {
        url.to_string()
    }
}

pub async fn select_fastest_mirrors(sources: &[MirrorSource]) -> Vec<MirrorSource> {
    // Layer 1: concurrent throughput probing for all mirrors
    let probes: Vec<_> = sources
        .iter()
        .map(|s| async {
            let throughput = probe_mirror_throughput(s).await;
            (s.clone(), throughput)
        })
        .collect();

    let mut results = futures_util::future::join_all(probes).await;

    // Check if any throughput probe succeeded
    let has_throughput = results.iter().any(|(_, t)| *t > 0);

    if has_throughput {
        // Sort by throughput descending, failed ones go to the end
        results.sort_by(|a, b| b.1.cmp(&a.1));
        let sorted: Vec<MirrorSource> = results.into_iter().map(|(s, _)| s).collect();
        if let Some(best) = sorted.first() {
            info!("Fastest mirror selected by throughput: {} ({})", best.mirror, best.url);
        }
        sorted
    } else {
        // Layer 2 (fallback): all throughput probes failed, degrade to TCP latency probing
        warn!("All throughput probes failed, falling back to TCP latency probing");
        let mut with_latency: Vec<_> = Vec::new();
        for s in sources {
            let domain = extract_domain(&s.url);
            let latency = probe_tcp_latency(&domain).await;
            with_latency.push((s.clone(), latency));
        }
        with_latency.sort_by_key(|(_, latency)| *latency);
        let sorted: Vec<MirrorSource> = with_latency.into_iter().map(|(s, _)| s).collect();
        if let Some(best) = sorted.first() {
            info!("Fastest mirror selected by TCP latency (fallback): {} ({})", best.mirror, best.url);
        }
        sorted
    }
}
