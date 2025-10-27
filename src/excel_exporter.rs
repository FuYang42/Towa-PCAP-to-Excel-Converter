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

        // Write headers
        worksheet.write_with_format(0, 0, "X (m)", &header_format)?;
        worksheet.write_with_format(0, 1, "Y (m)", &header_format)?;
        worksheet.write_with_format(0, 2, "Z (m)", &header_format)?;

        // Set column widths
        worksheet.set_column_width(0, 12)?;
        worksheet.set_column_width(1, 12)?;
        worksheet.set_column_width(2, 12)?;

        // Write data
        for (i, point) in points.iter().enumerate() {
            let row = (i + 1) as u32;
            worksheet.write_with_format(row, 0, point.x, &number_format)?;
            worksheet.write_with_format(row, 1, point.y, &number_format)?;
            worksheet.write_with_format(row, 2, point.z, &number_format)?;
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
                },
                Point {
                    x: 82.32,
                    y: -3.98,
                    z: 4.05,
                },
            ],
        );

        channel_points.insert(
            5,
            vec![Point {
                x: -1.05,
                y: 5.22,
                z: 0.32,
            }],
        );

        // Export to test file
        let result = export_to_excel(&channel_points, "test_output.xlsx");
        assert!(result.is_ok());

        // Clean up
        let _ = std::fs::remove_file("test_output.xlsx");
    }
}
