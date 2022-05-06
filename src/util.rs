pub fn print_hex(data: &[u8], cols: usize) {
    for (i, byte) in data.iter().enumerate() {
        eprint!("{:02x}", byte);
        if i % cols == cols-1 { eprint!("\n") }
        else { eprint!(" ") }
    }
    if data.len() % cols != 0 { eprintln!() }
} 