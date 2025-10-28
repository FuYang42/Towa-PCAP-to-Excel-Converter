/// Excel export functionality - creates multiple sheets for different channels

use crate::cepton::Point;
use anyhow::{Context, Result};
use rust_xlsxwriter::{Format, Workbook};
use std::collections::HashMap;

/// Export channel points to Excel with multiple sheets (one per channel)
pub fn export_to_excel(channel_points: &HashMap<u8, Vec<Point>>, output_path: &str) -> Result<()> {
    let mut workbook = Workbook::new();

    // Create format for headers
    let header_format = Format::new()
        .set_bold()
        .set_background_color(rust_xlsxwriter::Color::RGB(0x4472C4))
        .set_font_color(rust_xlsxwriter::Color::White);

    // Create format for numbers (4 decimal places)
    let number_format = Format::new().set_num_format("0.0000");

    // Sort channels for consistent ordering
    let mut channels: Vec<_> = channel_points.keys().collect();
    channels.sort();

    for &channel in channels {
        let points = &channel_points[&channel];

        if points.is_empty() {
            continue;
        }

        // Create worksheet for this channel
        let sheet_name = format!("Channel_{}", channel);
        let worksheet = workbook.add_worksheet();
        worksheet.set_name(&sheet_name)?;

        // Check if we have debug data
        let has_debug_data = points.first().and_then(|p| p.distance).is_some();

        // Write headers
        let mut col = 0;
        worksheet.write_with_format(0, col, "X (m)", &header_format)?;
        col += 1;
        worksheet.write_with_format(0, col, "Y (m)", &header_format)?;
        col += 1;
        worksheet.write_with_format(0, col, "Z (m)", &header_format)?;
        col += 1;
        worksheet.write_with_format(0, col, "Reflectivity", &header_format)?;
        col += 1;
        worksheet.write_with_format(0, col, "Flags", &header_format)?;
        col += 1;

        if has_debug_data {
            worksheet.write_with_format(0, col, "Distance", &header_format)?;
            col += 1;
            worksheet.write_with_format(0, col, "Intensity", &header_format)?;
            col += 1;
            worksheet.write_with_format(0, col, "Power Level", &header_format)?;
        }

        // Set column widths
        worksheet.set_column_width(0, 12)?;
        worksheet.set_column_width(1, 12)?;
        worksheet.set_column_width(2, 12)?;
        worksheet.set_column_width(3, 14)?;
        worksheet.set_column_width(4, 10)?;
        if has_debug_data {
            worksheet.set_column_width(5, 12)?;
            worksheet.set_column_width(6, 12)?;
            worksheet.set_column_width(7, 12)?;
        }

        // Write data
        for (i, point) in points.iter().enumerate() {
            let row = (i + 1) as u32;
            let mut col = 0;

            worksheet.write_with_format(row, col, point.x, &number_format)?;
            col += 1;
            worksheet.write_with_format(row, col, point.y, &number_format)?;
            col += 1;
            worksheet.write_with_format(row, col, point.z, &number_format)?;
            col += 1;
            worksheet.write_number(row, col, point.reflectivity as f64)?;
            col += 1;
            worksheet.write_number(row, col, point.flags as f64)?;
            col += 1;

            // Write debug fields if present
            if has_debug_data {
                if let Some(distance) = point.distance {
                    worksheet.write_number(row, col, distance as f64)?;
                }
                col += 1;
                if let Some(intensity) = point.intensity {
                    worksheet.write_number(row, col, intensity as f64)?;
                }
                col += 1;
                if let Some(power_level) = point.power_level {
                    worksheet.write_number(row, col, power_level as f64)?;
                }
            }
        }

        // Freeze first row (headers)
        worksheet.set_freeze_panes(1, 0)?;
    }

    // Save workbook
    workbook
        .save(output_path)
        .with_context(|| format!("Failed to save Excel file: {}", output_path))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_export_to_excel() {
        let mut channel_points: HashMap<u8, Vec<Point>> = HashMap::new();

        // Add some test points
        channel_points.insert(
            0,
            vec![
                Point {
                    x: 12.8,
                    y: 0.72,
                    z: -88.94,
                    reflectivity: 128,
                    flags: 0,
                    distance: None,
                    intensity: None,
                    power_level: None,
                },
                Point {
                    x: 82.32,
                    y: -3.98,
                    z: 4.05,
                    reflectivity: 255,
                    flags: 1,
                    distance: None,
                    intensity: None,
                    power_level: None,
                },
            ],
        );

        channel_points.insert(
            5,
            vec![Point {
                x: -1.05,
                y: 5.22,
                z: 0.32,
                reflectivity: 64,
                flags: 0,
                distance: None,
                intensity: None,
                power_level: None,
            }],
        );

        // Export to test file
        let result = export_to_excel(&channel_points, "test_output.xlsx");
        assert!(result.is_ok());

        // Clean up
        let _ = std::fs::remove_file("test_output.xlsx");
    }
}
