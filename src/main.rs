// main.rs
use rust_canbus_parser::parser::CanParser;
use std::collections::HashMap;

fn main() {
    let mut parser = CanParser::new();

    // Register known CAN IDs
    let known_ids = [
        (0x1234, "EngineSpeed".to_string()),
        (0x0567, "VehicleStatus".to_string()),
    ];
    parser.message_definitions = HashMap::from(known_ids);

    // Example: raw_data representing ID 0x1234 and 2 data bytes (0xDE, 0xAD).
    let raw_data = [0x12, 0x34, 0xDE, 0xAD];

    match parser.parse_message(&raw_data) {
        Ok(msg) => {
            println!(
                "Parsed CAN message with ID=0x{:X}, data={:X?}",
                msg.id, msg.data
            );
        }
        Err(e) => eprintln!("Error parsing CAN frame: {}", e),
    };
}
