use std::env;
use std::fs;
use std::fmt;
use std::convert::TryInto;
use uf2_parser::utils;

static UF2_MAX_DATALEN: u32 = 0x100;

struct Uf2Header {
    magic1: u32,
    magic2: u32,
    flags: u32,
    flash_addr: u32,
    data_len: u32,
    seq_no: u32,
    total_blocks: u32,
    board_id: u32,
    data: [u8; 0x1dc],
    final_magic: u32
}

impl fmt::Display for Uf2Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Magic1:\t\t\t{:08x}\n", self.magic1)?;
        write!(f, "Magic2:\t\t\t{:08x}\n", self.magic2)?;
        write!(f, "Flags:\t\t\t{:08x}\n", self.flags)?;
        write!(f, "Flash Address:\t\t{:08x}\n", self.flash_addr)?;
        write!(f, "Data Length:\t\t{:08x}\n", self.data_len)?;
        write!(f, "Sequence Number:\t{:08x}\n", self.seq_no)?;
        write!(f, "Total Blocks:\t\t{:08x}\n", self.total_blocks)?;
        write!(f, "Board ID (Can be zero):\t{:08x}\n", self.board_id)?;
        write!(f, "Final Magic:\t\t{:08x}\n", self.final_magic)?;
        write!(f, "Data:\n")?;
        write!(f, "{}\n", utils::hex_encode(&self.data))
    }
}

// If this were C I could just read the fucking data and cast it to Uf2Header
fn parse_uf2_block(data: &[u8]) -> Uf2Header {
    Uf2Header {
        magic1:         u32::from_le_bytes(data[0x00..0x04].try_into().unwrap()),
        magic2:         u32::from_le_bytes(data[0x04..0x08].try_into().unwrap()),
        flags:          u32::from_le_bytes(data[0x08..0x0c].try_into().unwrap()),
        flash_addr:     u32::from_le_bytes(data[0x0c..0x10].try_into().unwrap()),
        data_len:       u32::from_le_bytes(data[0x10..0x14].try_into().unwrap()),
        seq_no:         u32::from_le_bytes(data[0x14..0x18].try_into().unwrap()),
        total_blocks:   u32::from_le_bytes(data[0x18..0x1c].try_into().unwrap()),
        board_id:       u32::from_le_bytes(data[0x1c..0x20].try_into().unwrap()),
        data:           data[0x20..0x20 + 476].try_into().unwrap(),
        final_magic:    u32::from_le_bytes(data[0x20 + 476..0x20 + 476 + 4].try_into().unwrap()),
    }
}

fn uf2_get_flash_size(data: &[u8], filesize: usize, block_size: u32) -> u32 {
    let mut max_address: u32 = 0;
    for i in (0..filesize).step_by(block_size as usize) {
        let header = parse_uf2_block(&data[i as usize..]);
        if header.flash_addr > max_address { max_address = header.flash_addr }
    }
    return max_address + UF2_MAX_DATALEN;
}

fn main() {
    let argv: Vec<String> = env::args().collect();
    let input_file = &argv[1];
    let metadata = fs::metadata(input_file).expect("IO Error opening input file: ");
    let filesize = metadata.len();
    println!("File size: {}", metadata.len());
    let data = fs::read(input_file).expect("bad data read");
    let header_size = std::mem::size_of::<Uf2Header>();
    let flash_size = uf2_get_flash_size(&data, data.len(), 0x200);
    for i in (0..filesize).step_by(header_size) {
        let header = parse_uf2_block(&data[i as usize..]);
    }
}
