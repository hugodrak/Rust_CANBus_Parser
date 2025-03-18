// parser.rs
use crate::error::CanError;
use std::collections::HashMap;

/// A CAN message with an identifier and up to 8 bytes of data.
#[derive(Debug)]
pub struct CanMessage {
    pub id: u32,
    pub data: Vec<u8>,
}

/// The main struct responsible for parsing raw CAN frames.
pub struct CanParser {
    /// A mapping of known CAN IDs to a human-readable name or definition.
    pub message_definitions: HashMap<u32, String>,
}

impl CanParser {
    /// Creates a new `CanParser` with an empty definition map.
    pub fn new() -> Self {
        Self {
            message_definitions: HashMap::new(),
        }
    }

    /// Parses a raw CAN frame from a byte slice.
    ///
    /// # Arguments
    ///
    /// * `raw_data` - A byte slice containing a CAN frame. This is typically 2 bytes for ID
    ///   (example format here) followed by up to 8 bytes of data.
    ///
    /// # Returns
    ///
    /// A `CanMessage` if successful, or a `CanError` if the frame is invalid.
    pub fn parse_message(&self, raw_data: &[u8]) -> Result<CanMessage, CanError> {
        // For demonstration, let's assume the first 2 bytes represent the CAN ID in big-endian.
        if raw_data.len() < 2 {
            return Err(CanError::InvalidFrame(
                "Data length too short to extract an ID".to_string(),
            ));
        }

        // Example: combine first two bytes into a 16-bit ID.
        let id = ((raw_data[0] as u32) << 8) | (raw_data[1] as u32);

        // The rest of the bytes are the payload data. In real CAN, max is 8 bytes, but we can be flexible.
        let data = raw_data[2..].to_vec();

        // Optionally check if ID is known or not.
        if !self.message_definitions.contains_key(&id) {
            // We can choose to return an error or just proceed with an "unknown" ID
            // for demonstration, let's be strict:
            return Err(CanError::UnknownId(id));
        }

        Ok(CanMessage { id, data })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_parse() {
        let mut parser = CanParser::new();
        // Register a known ID for testing
        parser.message_definitions.insert(0x1234, "TestMessage".to_string());

        let raw_data = [0x12, 0x34, 0xAB, 0xCD];
        let message = parser.parse_message(&raw_data).unwrap();
        assert_eq!(message.id, 0x1234);
        assert_eq!(message.data, vec![0xAB, 0xCD]);
    }

    #[test]
    fn test_invalid_length() {
        let parser = CanParser::new();
        let raw_data = [0x12];
        let result = parser.parse_message(&raw_data);
        assert!(result.is_err());
        if let Err(CanError::InvalidFrame(msg)) = result {
            assert!(msg.contains("Data length too short"));
        } else {
            panic!("Expected InvalidFrame error.");
        }
    }

    #[test]
    fn test_unknown_id() {
        let parser = CanParser::new();
        let raw_data = [0x12, 0x34, 0xAA];
        let result = parser.parse_message(&raw_data);
        assert!(matches!(result, Err(CanError::UnknownId(0x1234))));
    }
}
