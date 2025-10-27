/// Cepton STDV packet and point data structures

/// Represents a 3D point with XYZ coordinates in meters
#[derive(Debug, Clone)]
pub struct Point {
    pub x: f64,  // meters
    pub y: f64,  // meters
    pub z: f64,  // meters
}

/// STDV packet header (24 bytes)
#[derive(Debug)]
pub struct StdvHeader {
    pub signature: [u8; 4],      // "STDV"
    pub header_version: u8,
    pub header_size: u8,
    pub flags: u16,
    pub timestamp: u64,           // microseconds since sensor boot
    pub point_version: u8,
    pub point_size: u8,
    pub point_count: u16,         // max 144 points per packet
    pub sequence_id: u32,
}

impl StdvHeader {
    /// Parse STDV header from 24 bytes
    pub fn parse(data: &[u8]) -> Option<Self> {
        if data.len() < 24 {
            return None;
        }

        // Check signature
        if &data[0..4] != b"STDV" {
            return None;
        }

        let signature = [data[0], data[1], data[2], data[3]];
        let header_version = data[4];
        let header_size = data[5];
        let flags = u16::from_le_bytes([data[6], data[7]]);
        let timestamp = u64::from_le_bytes([
            data[8], data[9], data[10], data[11],
            data[12], data[13], data[14], data[15],
        ]);
        let point_version = data[16];
        let point_size = data[17];
        let point_count = u16::from_le_bytes([data[18], data[19]]);
        let sequence_id = u32::from_le_bytes([data[20], data[21], data[22], data[23]]);

        Some(StdvHeader {
            signature,
            header_version,
            header_size,
            flags,
            timestamp,
            point_version,
            point_size,
            point_count,
            sequence_id,
        })
    }
}

/// Raw point data structure (10 bytes minimum)
#[derive(Debug)]
pub struct RawPoint {
    pub x: i16,           // 0.5cm resolution
    pub y: u16,           // 0.5cm resolution
    pub z: i16,           // 0.5cm resolution
    pub reflectivity: u8, // 0-255
    pub timestamp: u8,    // microseconds (0-255)
    pub laser_id: u8,     // channel/laser ID (0-63)
    pub flags: u8,
}

impl RawPoint {
    /// Parse a single point from 10 bytes
    pub fn parse(data: &[u8]) -> Option<Self> {
        if data.len() < 10 {
            return None;
        }

        let x = i16::from_le_bytes([data[0], data[1]]);
        let y = u16::from_le_bytes([data[2], data[3]]);
        let z = i16::from_le_bytes([data[4], data[5]]);
        let reflectivity = data[6];
        let timestamp = data[7];
        let laser_id = data[8];
        let flags = data[9];

        Some(RawPoint {
            x,
            y,
            z,
            reflectivity,
            timestamp,
            laser_id,
            flags,
        })
    }

    /// Convert raw point to meters
    /// Cepton uses 0.5cm (0.005m) resolution for coordinates
    pub fn to_meters(&self) -> Point {
        const SCALE: f64 = 0.005; // 0.5cm = 0.005m

        Point {
            x: self.x as f64 * SCALE,
            y: self.y as f64 * SCALE,
            z: self.z as f64 * SCALE,
        }
    }

    /// Get the channel/laser ID
    pub fn channel(&self) -> u8 {
        self.laser_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stdv_header_parse() {
        let mut data = vec![0u8; 24];
        data[0..4].copy_from_slice(b"STDV");
        data[18] = 144; // point_count low byte

        let header = StdvHeader::parse(&data).unwrap();
        assert_eq!(&header.signature, b"STDV");
        assert_eq!(header.point_count, 144);
    }

    #[test]
    fn test_raw_point_parse() {
        // Test data: x=2560 (12.8m), y=144 (0.72m), z=-17788 (-88.94m)
        let data = [0x00, 0x0a, 0x90, 0x00, 0x84, 0xba, 0x74, 0x02, 0x05, 0x00];
        let point = RawPoint::parse(&data).unwrap();

        assert_eq!(point.x, 2560);
        assert_eq!(point.y, 144);
        assert_eq!(point.z, -17788);
        assert_eq!(point.laser_id, 5);

        let meters = point.to_meters();
        assert!((meters.x - 12.8).abs() < 0.01);
        assert!((meters.y - 0.72).abs() < 0.01);
    }
}
