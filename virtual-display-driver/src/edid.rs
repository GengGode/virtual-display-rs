use bytemuck::{Pod, Zeroable};

static MONITOR_EDID: &[u8] = &[
    0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x0D, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0x21, 0x01, 0x03, 0x80, 0x32, 0x1F, 0x78, 0x07, 0xEE, 0x95, 0xA3, 0x54, 0x4C, 0x99, 0x26,
    0x0F, 0x50, 0x54, 0x00, 0x00, 0x00, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
    0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x02, 0x3A, 0x80, 0x18, 0x71, 0x38, 0x2D, 0x40, 0x58, 0x2C,
    0x45, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1E, 0x00, 0x00, 0x00, 0xFD, 0x00, 0x17, 0xF0, 0x0F,
    0xFF, 0x0F, 0x00, 0x0A, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x00, 0x00, 0x00, 0xFC, 0x00, 0x56,
    0x69, 0x72, 0x74, 0x75, 0x44, 0x69, 0x73, 0x70, 0x6C, 0x61, 0x79, 0x2B, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
#[allow(clippy::module_name_repetitions)]
pub struct EdidBlob {
    header: [u8; 8],
    manufacturer_id: [u8; 2],
    product_code: u16,
    serial_number: u32,
    manufacture_week: u8,
    manufacture_year: u8,
    version: u8,
    revision: u8,
}

pub fn generate_edid_with(serial: u32) -> Vec<u8> {
    // change serial number in the header
    let header_bytes = &MONITOR_EDID[..std::mem::size_of::<EdidBlob>()];
    let mut header = bytemuck::pod_read_unaligned::<EdidBlob>(header_bytes);
    header.serial_number = serial;
    let header = bytemuck::bytes_of(&header);

    // slice of monitor edid minus header
    let data = &MONITOR_EDID[std::mem::size_of::<EdidBlob>()..];

    // splice together header and the rest of the EDID
    let mut edid: Vec<u8> = header.iter().copied().chain(data.iter().copied()).collect();
    // regenerate checksum
    gen_checksum(&mut edid);

    edid
}

pub fn get_edid_serial(edid: &[u8]) -> u32 {
    let header_bytes = &edid[0..std::mem::size_of::<EdidBlob>()];
    let header = bytemuck::pod_read_unaligned::<EdidBlob>(header_bytes);
    header.serial_number
}

fn gen_checksum(data: &mut [u8]) {
    // important, this is the bare minimum length
    assert!(data.len() >= 128);

    // slice to the entire data minus the last checksum byte
    let edid_data = &data[..=126];

    // do checksum calculation
    let sum: u32 = edid_data.iter().copied().map(u32::from).sum();
    let checksum = u8::try_from(256 - (sum % 256)).unwrap();

    // update last byte with new checksum
    data[127] = checksum;
}
