use std::env;
use std::io::{self, Read, Write};

const CHUNK_SIZE_KB: usize = 16 * 1024;

fn main() {
    let silent = env::var("PV_SILENT").unwrap_or(String::new()).len() > 0;
    dbg!(silent);
    let mut total_bytes = 0;
    loop {
        let mut buffer = [0; CHUNK_SIZE_KB];
        let num_read = match io::stdin().read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break
        };
        dbg!(total_bytes += num_read);
        io::stdout().write_all(&buffer[..num_read]).unwrap();
    }

    if dbg!(!silent) {
        eprintln!("num_read: {}", total_bytes);
    }

}