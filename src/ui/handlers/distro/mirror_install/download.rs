// SPDX-FileCopyrightText: Copyright (c) 2026 owu <wqh@live.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::path::{Path, PathBuf};
use std::time::Duration;
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::time::Instant;
use futures_util::StreamExt;
use reqwest::header::{RANGE, CONTENT_LENGTH, CONTENT_TYPE, ACCEPT_RANGES};
use tokio::fs::{File, OpenOptions};
use tokio::io::AsyncWriteExt;
use std::sync::Arc;
use tracing::{info, warn, debug};

use crate::api::models::MirrorSource;
use super::types::{DownloadError, DownloadProgress};

fn build_app_headers() -> reqwest::header::HeaderMap {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::USER_AGENT,
        reqwest::header::HeaderValue::from_static("WslDashboard/0.8.0"),
    );
    headers.insert(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("*/*"),
    );
    headers.insert(
        reqwest::header::ACCEPT_LANGUAGE,
        reqwest::header::HeaderValue::from_static("zh-CN,zh;q=0.9,en;q=0.8"),
    );
    headers
}

fn build_browser_headers() -> reqwest::header::HeaderMap {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::USER_AGENT,
        reqwest::header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"),
    );
    headers.insert(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8"),
    );
    headers.insert(
        reqwest::header::ACCEPT_LANGUAGE,
        reqwest::header::HeaderValue::from_static("zh-CN,zh;q=0.9,en;q=0.8"),
    );
    headers
}

fn build_client(headers: reqwest::header::HeaderMap) -> Result<reqwest::Client, reqwest::Error> {
    reqwest::Client::builder()
        .connect_timeout(Duration::from_secs(15))
        .default_headers(headers)
        .no_gzip()
        .no_brotli()
        .no_deflate()
        .build()
}

// Probe file size and Range support
// Returns (total_size, supports_range)
async fn probe_file_size_and_range(
    client: &reqwest::Client,
    url: &str,
) -> Result<(u64, bool), DownloadError> {
    let resp = client
        .request(reqwest::Method::HEAD, url)
        .send()
        .await
        .map_err(|e| DownloadError::NetworkError { mirror: url.to_string(), error: e.to_string() })?;

    let total_size = resp
        .headers()
        .get(CONTENT_LENGTH)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(0);

    let supports_range = resp
        .headers()
        .get(ACCEPT_RANGES)
        .map(|v| v.to_str().unwrap_or("").contains("bytes"))
        .unwrap_or(false);

    Ok((total_size, supports_range))
}

// Chunked concurrent download
// Splits the file into multiple chunks, downloads each to .chunk.N temp files, then merges
async fn download_file_chunked<F>(
    source: &MirrorSource,
    temp_file_path: &Path,
    custom_headers: &reqwest::header::HeaderMap,
    _browser_headers: &reqwest::header::HeaderMap,
    progress_callback: &F,
    total_size: u64,
) -> Result<(), DownloadError>
where
    F: Fn(DownloadProgress) + Send + Sync,
{
    const CHUNK_SIZE: u64 = 8 * 1024 * 1024; // 8MB per chunk
    const MAX_CONNECTIONS: usize = 4;         // max 4 concurrent connections

    let chunk_count = ((total_size + CHUNK_SIZE - 1) / CHUNK_SIZE).max(1) as usize;
    let connections = chunk_count.min(MAX_CONNECTIONS);
    // Chunk size per connection
    let per_conn_size = (total_size + connections as u64 - 1) / connections as u64;

    info!(
        "Chunked download: {} bytes, {} connections, ~{:.1}MB each",
        total_size,
        connections,
        per_conn_size as f64 / 1024.0 / 1024.0
    );

    let client = build_client(custom_headers.clone())
        .map_err(|e| DownloadError::NetworkError { mirror: source.mirror.clone(), error: e.to_string() })?;

    let total_downloaded = Arc::new(AtomicU64::new(0));
    let start_time = Instant::now();

    // Build download task for each connection
    let mut handles = Vec::with_capacity(connections);
    for i in 0..connections {
        let range_start = i as u64 * per_conn_size;
        let range_end = ((i as u64 + 1) * per_conn_size - 1).min(total_size - 1);
        if range_start > range_end { continue; }

        let chunk_path = temp_file_path.with_extension(format!(
            "{}.chunk.{}",
            temp_file_path.extension().unwrap_or_default().to_string_lossy(),
            i
        ));

        // Check existing partial download (resume support)
        let existing = if chunk_path.exists() {
            tokio::fs::metadata(&chunk_path).await.map(|m| m.len()).unwrap_or(0)
        } else {
            0
        };

        let actual_start = range_start + existing;
        total_downloaded.fetch_add(existing, Ordering::Relaxed);

        if actual_start > range_end {
            // This chunk is already fully downloaded
            handles.push(tokio::spawn(async move { Ok::<(), DownloadError>(()) }));
            continue;
        }

        let url = source.url.clone();
        let mirror = source.mirror.clone();
        let client = client.clone();
        let path = chunk_path.clone();
        let td = total_downloaded.clone();

        handles.push(tokio::spawn(async move {
            let req = client
                .get(&url)
                .header(RANGE, format!("bytes={}-{}", actual_start, range_end));

            let response = req.send().await.map_err(|e| DownloadError::NetworkError {
                mirror: mirror.clone(),
                error: e.to_string(),
            })?;

            let status = response.status();
            if !status.is_success() && status != reqwest::StatusCode::PARTIAL_CONTENT {
                return Err(DownloadError::NetworkError {
                    mirror: mirror.clone(),
                    error: format!("HTTP {}", status),
                });
            }

            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&path)
                .await
                .map_err(|e| DownloadError::FileError { mirror: mirror.clone(), error: e.to_string() })?;

            let mut stream = response.bytes_stream();
            while let Some(chunk) = stream.next().await {
                let chunk = chunk.map_err(|e| DownloadError::NetworkError {
                    mirror: mirror.clone(),
                    error: e.to_string(),
                })?;
                file.write_all(&chunk).await.map_err(|e| DownloadError::FileError {
                    mirror: mirror.clone(),
                    error: e.to_string(),
                })?;
                td.fetch_add(chunk.len() as u64, Ordering::Relaxed);
            }
            Ok(())
        }));
    }

    // Wait for all tasks in background, periodically report progress
    let handles_fut = tokio::spawn(futures_util::future::join_all(handles));
    // Ordered sampling queue: (timestamp_ms, cumulative_bytes), max 30 entries (~3s window)
    const SPEED_WINDOW_SIZE: usize = 30;
    let mut speed_window: std::collections::VecDeque<(u128, u64)> = std::collections::VecDeque::with_capacity(SPEED_WINDOW_SIZE);

    loop {
        tokio::time::sleep(Duration::from_millis(100)).await;

        let downloaded = total_downloaded.load(Ordering::Relaxed);
        let percent = if total_size > 0 {
            (downloaded as f32 / total_size as f32) * 100.0
        } else {
            0.0
        };

        // Record sampling point
        let ts_ms = start_time.elapsed().as_millis();
        speed_window.push_back((ts_ms, downloaded));
        if speed_window.len() > SPEED_WINDOW_SIZE {
            speed_window.pop_front();
        }

        // Calculate average speed using head and tail of window
        let speed = if speed_window.len() >= 2 {
            let (oldest_ts, oldest_bytes) = speed_window.front().unwrap();
            let (newest_ts, newest_bytes) = speed_window.back().unwrap();
            let dt_ms = newest_ts - oldest_ts;
            if dt_ms > 0 {
                let delta = newest_bytes.saturating_sub(*oldest_bytes);
                format!("{:.1} MB/s", (delta as f64 / 1024.0 / 1024.0) / (dt_ms as f64 / 1000.0))
            } else {
                "0.0 MB/s".to_string()
            }
        } else {
            "0.0 MB/s".to_string()
        };
        progress_callback(DownloadProgress::Downloading { percent, speed: &speed });

        if handles_fut.is_finished() {
            break;
        }
    }

    // Collect results
    let results = handles_fut.await.map_err(|e| DownloadError::NetworkError {
        mirror: source.mirror.clone(),
        error: format!("Join error: {}", e),
    })?;

    // Final progress
    let downloaded = total_downloaded.load(Ordering::Relaxed);
    let elapsed = start_time.elapsed().as_secs_f64();
    let speed = if elapsed > 0.0 {
        format!("{:.1} MB/s", (downloaded as f64 / 1024.0 / 1024.0) / elapsed)
    } else {
        "0.0 MB/s".to_string()
    };
    progress_callback(DownloadProgress::Downloading { percent: 100.0, speed: &speed });

    // Check for failed tasks
    for (i, result) in results.into_iter().enumerate() {
        match result {
            Ok(Ok(())) => {}
            Ok(Err(e)) => {
                // Clean up all temp chunk files
                for j in 0..connections {
                    let p = temp_file_path.with_extension(format!(
                        "{}.chunk.{}",
                        temp_file_path.extension().unwrap_or_default().to_string_lossy(),
                        j
                    ));
                    let _ = tokio::fs::remove_file(&p).await;
                }
                return Err(e);
            }
            Err(e) => {
                for j in 0..connections {
                    let p = temp_file_path.with_extension(format!(
                        "{}.chunk.{}",
                        temp_file_path.extension().unwrap_or_default().to_string_lossy(),
                        j
                    ));
                    let _ = tokio::fs::remove_file(&p).await;
                }
                return Err(DownloadError::NetworkError {
                    mirror: source.mirror.clone(),
                    error: format!("Task {} panicked: {}", i, e),
                });
            }
        }
    }

    // Merge all chunks into the target file
    let mut output = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(temp_file_path)
        .await
        .map_err(|e| DownloadError::FileError { mirror: source.mirror.clone(), error: e.to_string() })?;

    for i in 0..connections {
        let chunk_path = temp_file_path.with_extension(format!(
            "{}.chunk.{}",
            temp_file_path.extension().unwrap_or_default().to_string_lossy(),
            i
        ));
        let data = tokio::fs::read(&chunk_path).await
            .map_err(|e| DownloadError::FileError { mirror: source.mirror.clone(), error: e.to_string() })?;
        output.write_all(&data).await
            .map_err(|e| DownloadError::FileError { mirror: source.mirror.clone(), error: e.to_string() })?;
        let _ = tokio::fs::remove_file(&chunk_path).await;
    }

    info!("Chunked download completed: {} bytes in {:.1}s", total_size, elapsed);
    Ok(())
}

async fn download_file<F>(
    source: &MirrorSource,
    temp_file_path: &Path,
    custom_headers: &reqwest::header::HeaderMap,
    browser_headers: &reqwest::header::HeaderMap,
    progress_callback: &F,
) -> Result<(), DownloadError>
where
    F: Fn(DownloadProgress) + Send + Sync,
{
    let client = build_client(custom_headers.clone())
        .map_err(|e| DownloadError::NetworkError { mirror: source.mirror.clone(), error: e.to_string() })?;

    // Try chunked concurrent download: probe file size and Range support
    const MIN_CHUNKED_SIZE: u64 = 10 * 1024 * 1024; // Enable chunked for files > 10MB
    match probe_file_size_and_range(&client, &source.url).await {
        Ok((total_size, true)) if total_size >= MIN_CHUNKED_SIZE => {
            info!("Mirror {} supports Range, size {}MB, using chunked download", source.mirror, total_size / 1024 / 1024);
            match download_file_chunked(source, temp_file_path, custom_headers, browser_headers, progress_callback, total_size).await {
                Ok(()) => return Ok(()),
                Err(e) => {
                    warn!("Chunked download failed for {}: {}, falling back to single-stream", source.mirror, e);
                    // Clean up any remaining chunk files
                    for j in 0..4 {
                        let p = temp_file_path.with_extension(format!(
                            "{}.chunk.{}",
                            temp_file_path.extension().unwrap_or_default().to_string_lossy(),
                            j
                        ));
                        let _ = tokio::fs::remove_file(&p).await;
                    }
                }
            }
        }
        Ok((total_size, supports_range)) => {
            debug!("Mirror {}: size={}MB, range={}, using single-stream", source.mirror, total_size / 1024 / 1024, supports_range);
        }
        Err(e) => {
            debug!("Probe failed for {}: {}, using single-stream", source.mirror, e);
        }
    }

    // Single-stream download (fallback logic)
    let mut start_byte = 0;
    if temp_file_path.exists() {
        if let Ok(metadata) = tokio::fs::metadata(temp_file_path).await {
            start_byte = metadata.len();
        }
    }

    let req = if start_byte > 0 {
        client.get(&source.url).header(RANGE, format!("bytes={}-", start_byte))
    } else {
        client.get(&source.url)
    };

    let response = req.send().await.map_err(|e| DownloadError::NetworkError { mirror: source.mirror.clone(), error: e.to_string() })?;

    let status = response.status();
    let is_server_rejection = (!status.is_success() && status != reqwest::StatusCode::PARTIAL_CONTENT)
        || response.headers().get(CONTENT_TYPE).map_or(false, |ct| {
            let s = ct.to_str().unwrap_or("");
            s.contains("text/html") || s.contains("application/json")
        })
        || response.headers().get(CONTENT_LENGTH).map_or(false, |cl| {
            cl.to_str().ok().and_then(|s| s.parse::<u64>().ok()).map_or(false, |len| len > 0 && len < 1_000_000)
        });

    if is_server_rejection {
        let reason = if !status.is_success() && status != reqwest::StatusCode::PARTIAL_CONTENT {
            format!("HTTP {}", status)
        } else if let Some(ct) = response.headers().get(CONTENT_TYPE) {
            format!("Content-Type: {}", ct.to_str().unwrap_or(""))
        } else {
            format!("Content-Length < 1MB")
        };

        let ua_is_app = custom_headers.get(reqwest::header::USER_AGENT)
            .and_then(|v| v.to_str().ok())
            .map_or(false, |s| s.starts_with("WslDashboard/"));

        if ua_is_app {
            warn!("Mirror {} rejected app UA ({}), retrying with browser UA", source.mirror, reason);
            return Box::pin(download_file(source, temp_file_path, browser_headers, browser_headers, progress_callback)).await;
        } else {
            warn!("Mirror {} rejected browser UA ({}): {}", source.mirror, reason, status);
            return Err(DownloadError::NetworkError { mirror: source.mirror.clone(), error: format!("HTTP {} ({})", status, reason) });
        }
    }

    let total_size = response
        .headers()
        .get(CONTENT_LENGTH)
        .and_then(|ct_len| ct_len.to_str().ok())
        .and_then(|ct_len| ct_len.parse::<u64>().ok())
        .unwrap_or(0) + start_byte;

    if total_size > 0 && total_size < 1_000_000 {
        let ua_is_app = custom_headers.get(reqwest::header::USER_AGENT)
            .and_then(|v| v.to_str().ok())
            .map_or(false, |s| s.starts_with("WslDashboard/"));

        if ua_is_app {
            warn!("Mirror {} Content-Length suspicious with app UA, retrying with browser UA", source.mirror);
            return Box::pin(download_file(source, temp_file_path, browser_headers, browser_headers, progress_callback)).await;
        }
        return Err(DownloadError::HumanVerification {
            mirror: source.mirror.clone(),
            reason: format!("Content-Length < 1MB: {} bytes", total_size),
        });
    }

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(temp_file_path)
        .await
        .map_err(|e| DownloadError::FileError { mirror: source.mirror.clone(), error: e.to_string() })?;

    let mut downloaded = start_byte;
    let mut stream = response.bytes_stream();
    let start_time = Instant::now();
    let mut last_percent = -1.0;

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| DownloadError::NetworkError { mirror: source.mirror.clone(), error: e.to_string() })?;
        file.write_all(&chunk).await.map_err(|e| DownloadError::FileError { mirror: source.mirror.clone(), error: e.to_string() })?;
        downloaded += chunk.len() as u64;

        if total_size > 0 {
            let percent = (downloaded as f32 / total_size as f32) * 100.0;
            if percent - last_percent >= 0.1 {
                last_percent = percent;
                let elapsed = start_time.elapsed().as_secs_f64();
                let speed = if elapsed > 0.0 {
                    format!("{:.1} MB/s", ((downloaded - start_byte) as f64 / 1024.0 / 1024.0) / elapsed)
                } else {
                    "0.0 MB/s".to_string()
                };
                progress_callback(DownloadProgress::Downloading { percent, speed: &speed });
            }
        }
    }

    // Final progress report
    if total_size > 0 {
        let elapsed = start_time.elapsed().as_secs_f64();
        let speed = if elapsed > 0.0 {
            format!("{:.1} MB/s", ((downloaded - start_byte) as f64 / 1024.0 / 1024.0) / elapsed)
        } else {
            "0.0 MB/s".to_string()
        };
        progress_callback(DownloadProgress::Downloading { percent: 100.0, speed: &speed });
    }

    Ok(())
}

async fn validate_downloaded_file(path: &Path, expected_format: &str) -> Result<(), String> {
    use tokio::io::AsyncReadExt;
    let mut file = File::open(path).await.map_err(|e| e.to_string())?;

    match expected_format {
        "wsl" => {
            let mut header = [0u8; 2];
            if file.read_exact(&mut header).await.is_err() {
                return Err("Failed to read WSL header".to_string());
            }
            // A .wsl file is typically a gzipped tarball (1F 8B) or a zip/appx package (50 4B)
            if &header != &[0x1F, 0x8B] && &header != &[0x50, 0x4B] {
                return Err("Invalid WSL file header".to_string());
            }
        }
        "tar.xz" => {
            let mut header = [0u8; 6];
            if file.read_exact(&mut header).await.is_err() {
                return Err("Failed to read XZ header".to_string());
            }
            if &header != &[0xFD, 0x37, 0x7A, 0x58, 0x5A, 0x00] {
                return Err("Invalid XZ file header".to_string());
            }
        }
        "tar.gz" => {
            let mut header = [0u8; 2];
            if file.read_exact(&mut header).await.is_err() {
                return Err("Failed to read GZIP header".to_string());
            }
            if &header != &[0x1F, 0x8B] {
                return Err("Invalid GZIP file header".to_string());
            }
        }
        _ => {}
    }

    Ok(())
}

pub async fn download_with_fallback<F>(
    sources: &[MirrorSource],
    temp_file_path: &Path,
    progress_callback: F,
) -> Result<PathBuf, DownloadError>
where
    F: Fn(DownloadProgress) + Send + Sync,
{
    let app_headers = build_app_headers();
    let browser_headers = build_browser_headers();
    let mut errors = Vec::new();

    for source in sources {
        let mirror_name = &source.mirror;

        progress_callback(DownloadProgress::TryingMirror {
            mirror: mirror_name,
            url: &source.url,
        });

        match download_file(source, temp_file_path, &app_headers, &browser_headers, &progress_callback).await {
            Ok(_) => {
                if let Err(e) = validate_downloaded_file(temp_file_path, &source.format).await {
                    warn!("Mirror {} file validation failed: {}", mirror_name, e);
                    progress_callback(DownloadProgress::MirrorFileInvalid {
                        mirror: mirror_name,
                    });
                    errors.push(format!("{}: File error: {}", mirror_name, e));
                    let _ = tokio::fs::remove_file(temp_file_path).await;
                    continue;
                }
                info!("Download from mirror {} succeeded.", mirror_name);
                return Ok(temp_file_path.to_path_buf());
            }
            Err(e) => {
                let err_msg = e.to_string();
                warn!("Mirror {} failed: {}", mirror_name, err_msg);
                progress_callback(DownloadProgress::MirrorFailed {
                    mirror: mirror_name,
                    error: &err_msg,
                });
                errors.push(format!("{}: {}", mirror_name, err_msg));
                continue;
            }
        }
    }

    Err(DownloadError::AllMirrorsFailed { errors })
}
