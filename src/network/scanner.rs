// SPDX-FileCopyrightText: Copyright (c) 2026 owu <wqh@live.com>
// SPDX-License-Identifier: GPL-3.0-only

use super::models::{NetworkInterface};

// Get the list of Windows network interfaces (Internal IPs recommended)
pub fn get_physical_interfaces() -> Vec<NetworkInterface> {
    let mut interfaces = Vec::new();

    // Since we retrieve the network interface list in Rust, we can use the if-addrs crate.
    // If this dependency is not included, it needs to be added to Cargo.toml, or return empty for now.
    // Placeholder for now, dependency will be added via Cargo later.
    if let Ok(ifaces) = if_addrs::get_if_addrs() {
        for iface in ifaces {
            let ip = iface.ip();
            let is_loopback = ip.is_loopback();
            let is_v4 = ip.is_ipv4();
            
            if is_v4 {
                let name_lower = iface.name.to_lowercase();
                let is_virtual = name_lower.contains("wsl") 
                    || name_lower.contains("virtual")
                    || name_lower.contains("veth")
                    || name_lower.contains("vmware")
                    || name_lower.contains("hyper-v")
                    || name_lower.contains("tailscale")
                    || name_lower.contains("zerotier")
                    || name_lower.contains("wireguard")
                    || name_lower.contains("tunnel")
                    || name_lower.contains("openvpn");

                interfaces.push(NetworkInterface {
                    name: iface.name,
                    ip_address: ip.to_string(),
                    is_loopback,
                    is_virtual,
                });
            }
        }
    }

    interfaces
}
