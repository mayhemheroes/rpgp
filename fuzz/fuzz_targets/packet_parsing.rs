#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let packets = pgp::packet::PacketParser::new(data);
    for packet in packets {
        match match packet {
            Ok(p) => p,
            Err(_) => return,
        } {
            pgp::packet::Packet::CompressedData(d) => {
                let _ = d.decompress();
            }
            _ => {}
        }
    }
});
