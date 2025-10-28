/// PCAP file reader for extracting Cepton LiDAR data

use crate::cepton::{Point, RawPoint, StdvHeader};
use anyhow::{Context, Result};
use indicatif::ProgressBar;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

/// Scan PCAP file to count points per channel
pub fn scan_channels(pcap_path: &str, mode: crate::cepton::ParseMode) -> Result<HashMap<u8, usize>> {
    let mut file = File::open(pcap_path)
        .with_context(|| format!("Failed to open file: {}", pcap_path))?;

    // Read PCAP global header (24 bytes)
    let mut pcap_header = vec![0u8; 24];
    file.read_exact(&mut pcap_header)
        .context("Failed to read PCAP header")?;

    // Verify magic number
    let magic = u32::from_le_bytes([
        pcap_header[0],
        pcap_header[1],
        pcap_header[2],
        pcap_header[3],
    ]);

    if magic != 0xa1b2c3d4 && magic != 0xd4c3b2a1 {
        anyhow::bail!("Invalid PCAP file: bad magic number");
    }

    let mut channel_counts: HashMap<u8, usize> = HashMap::new();

    // Read all packets
    loop {
        // Read packet header (16 bytes)
        let mut pkt_header = vec![0u8; 16];
        match file.read_exact(&mut pkt_header) {
            Ok(_) => {}
            Err(_) => break, // End of file
        }

        let incl_len = u32::from_le_bytes([pkt_header[8], pkt_header[9], pkt_header[10], pkt_header[11]]);

        // Read packet data
        let mut packet_data = vec![0u8; incl_len as usize];
        file.read_exact(&mut packet_data)
            .context("Failed to read packet data")?;

        // Skip Ethernet (14) + IP (20) + UDP (8) headers = 42 bytes
        if packet_data.len() < 42 {
            continue;
        }

        let payload = &packet_data[42..];

        // Try to parse as STDV packet
        if let Some(header) = StdvHeader::parse(payload) {
            let point_data_start = 24; // After STDV header
            let point_size = match mode {
                crate::cepton::ParseMode::Normal => 10,
                crate::cepton::ParseMode::Debug => 17,
            };

            // Parse all points in this packet
            for i in 0..header.point_count {
                let offset = point_data_start + (i as usize * point_size);
                let required_size = match mode {
                    crate::cepton::ParseMode::Normal => 10,
                    crate::cepton::ParseMode::Debug => 17,
                };

                if offset + required_size <= payload.len() {
                    if let Some(raw_point) = RawPoint::parse_with_mode(&payload[offset..], mode) {
                        let channel = raw_point.channel();
                        *channel_counts.entry(channel).or_insert(0) += 1;
                    }
                }
            }
        }
    }

    Ok(channel_counts)
}

/// Extract points from selected channels
pub fn extract_points(
    pcap_path: &str,
    selected_channels: &[u8],
    channel_points: &mut HashMap<u8, Vec<Point>>,
    mode: crate::cepton::ParseMode,
    progress_bar: Option<&ProgressBar>,
) -> Result<()> {
    let mut file = File::open(pcap_path)
        .with_context(|| format!("Failed to open file: {}", pcap_path))?;

    // Skip PCAP global header (24 bytes)
    let mut pcap_header = vec![0u8; 24];
    file.read_exact(&mut pcap_header)
        .context("Failed to read PCAP header")?;

    // Read all packets
    loop {
        // Read packet header (16 bytes)
        let mut pkt_header = vec![0u8; 16];
        match file.read_exact(&mut pkt_header) {
            Ok(_) => {}
            Err(_) => break, // End of file
        }

        let incl_len = u32::from_le_bytes([pkt_header[8], pkt_header[9], pkt_header[10], pkt_header[11]]);

        // Read packet data
        let mut packet_data = vec![0u8; incl_len as usize];
        file.read_exact(&mut packet_data)
            .context("Failed to read packet data")?;

        // Skip Ethernet (14) + IP (20) + UDP (8) headers = 42 bytes
        if packet_data.len() < 42 {
            continue;
        }

        let payload = &packet_data[42..];

        // Try to parse as STDV packet
        if let Some(header) = StdvHeader::parse(payload) {
            let point_data_start = 24; // After STDV header
            let point_size = match mode {
                crate::cepton::ParseMode::Normal => 10,
                crate::cepton::ParseMode::Debug => 17,
            };

            // Parse all points in this packet
            for i in 0..header.point_count {
                let offset = point_data_start + (i as usize * point_size);
                let required_size = match mode {
                    crate::cepton::ParseMode::Normal => 10,
                    crate::cepton::ParseMode::Debug => 17,
                };

                if offset + required_size <= payload.len() {
                    if let Some(raw_point) = RawPoint::parse_with_mode(&payload[offset..], mode) {
                        let channel = raw_point.channel();

                        // Only extract if this channel is selected
                        if selected_channels.contains(&channel) {
                            let point = raw_point.to_meters();
                            if let Some(points) = channel_points.get_mut(&channel) {
                                points.push(point);
                            }
                        }

                        // Update progress bar
                        if let Some(pb) = progress_bar {
                            pb.inc(1);
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_scan_channels() {
        // This test would require a sample PCAP file
        // For now, just verify the function signature compiles
    }
}
