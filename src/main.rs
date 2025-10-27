mod cepton;
mod pcap_reader;
mod excel_exporter;

use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::io::{self, Write};
use std::path::Path;

fn main() -> Result<()> {
    println!("=======================================================");
    println!("  Cepton LiDAR PCAP to Excel Converter");
    println!("  Extract XYZ coordinates by channel");
    println!("=======================================================\n");

    // Get input file path (simple stdin read)
    print!("Enter PCAP file path [ch_28 (1).pcap]: ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let pcap_file = input.trim();

    let pcap_file = if pcap_file.is_empty() {
        "ch_28 (1).pcap"
    } else {
        pcap_file
    };

    println!("Using file: {}", pcap_file);

    if !Path::new(&pcap_file).exists() {
        anyhow::bail!("File not found: {}", pcap_file);
    }

    println!("\n[Step 1/4] Scanning PCAP file for channels...");
    println!("(This may take a moment for large files...)");

    // Scan file to get channel statistics
    let channel_stats = pcap_reader::scan_channels(&pcap_file)?;

    if channel_stats.is_empty() {
        anyhow::bail!("No valid STDV packets found in the file");
    }

    // Display channel statistics
    println!("\nFound {} channels:\n", channel_stats.len());
    let mut channels: Vec<_> = channel_stats.keys().cloned().collect();
    channels.sort();

    for channel in &channels {
        let count = channel_stats[channel];
        println!("  Channel {:2}: {:8} points", channel, count);
    }

    let total_points: usize = channel_stats.values().sum();
    println!("\n  Total:      {:8} points\n", total_points);

    // Let user select channels
    println!("[Step 2/4] Select channels to extract:");
    println!("  Options:");
    println!("    - Enter channel numbers separated by commas (e.g., 0,5,10)");
    println!("    - Enter 'all' to extract all channels");
    println!("    - Enter a range (e.g., 0-10)");
    print!("\nYour selection: ");
    io::stdout().flush()?;

    let mut selection = String::new();
    io::stdin().read_line(&mut selection)?;
    let selection = selection.trim().to_lowercase();

    let selected_channels: Vec<u8> = if selection == "all" {
        channels.clone()
    } else if selection.contains('-') && !selection.contains(',') {
        // Range selection (e.g., "0-10")
        let parts: Vec<&str> = selection.split('-').collect();
        if parts.len() == 2 {
            let start: u8 = parts[0].trim().parse()
                .map_err(|_| anyhow::anyhow!("Invalid range start"))?;
            let end: u8 = parts[1].trim().parse()
                .map_err(|_| anyhow::anyhow!("Invalid range end"))?;
            (start..=end).filter(|ch| channels.contains(ch)).collect()
        } else {
            anyhow::bail!("Invalid range format. Use: start-end (e.g., 0-10)");
        }
    } else {
        // Comma-separated selection
        selection
            .split(',')
            .filter_map(|s| s.trim().parse::<u8>().ok())
            .filter(|ch| channels.contains(ch))
            .collect()
    };

    if selected_channels.is_empty() {
        anyhow::bail!("No valid channels selected");
    }

    println!("\nSelected {} channel(s): {:?}", selected_channels.len(), selected_channels);

    // Calculate total points to extract
    let points_to_extract: usize = selected_channels.iter()
        .map(|ch| channel_stats[ch])
        .sum();

    println!("Total points to extract: {}", points_to_extract);

    // Extract points from selected channels
    println!("\n[Step 3/4] Extracting XYZ coordinates...");

    let mut channel_points: HashMap<u8, Vec<cepton::Point>> = HashMap::new();
    for &ch in &selected_channels {
        channel_points.insert(ch, Vec::new());
    }

    let pb = ProgressBar::new(total_points as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({percent}%) {msg}")?
            .progress_chars("=>-")
    );

    pcap_reader::extract_points(&pcap_file, &selected_channels, &mut channel_points, Some(&pb))?;

    pb.finish_with_message("Extraction complete!");

    // Export to Excel
    println!("\n[Step 4/4] Exporting to Excel...");

    let output_file = pcap_file.replace(".pcap", "_xyz.xlsx");
    excel_exporter::export_to_excel(&channel_points, &output_file)?;

    println!("\n✓ Export complete!");
    println!("\nOutput file: {}", output_file);

    // Summary
    println!("\n=======================================================");
    println!("Summary:");
    for &ch in &selected_channels {
        println!("  Channel {}: {} points extracted", ch, channel_points[&ch].len());
    }
    println!("=======================================================\n");

    println!("Press Enter to exit...");
    let mut _dummy = String::new();
    io::stdin().read_line(&mut _dummy)?;

    Ok(())
}
