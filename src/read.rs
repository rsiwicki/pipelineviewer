use crate::CHUNK_SIZE_KB;
use std::fs::File;
use std::io::{self, BufReader, Read, Result};
use std::sync::{Arc, Mutex};

pub fn read_loop(infile: &str, quit: Arc<Mutex<bool>>) -> Result<()> {
    let mut reader: Box<dyn Read> = if !infile.is_empty() {
        Box::new(BufReader::new(File::open(infile)?))
    } else {
        Box::new(BufReader::new(io::stdin()))
    };

    let mut buffer = [0; CHUNK_SIZE_KB];

    loop {
        let num_read = match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };

        //TODO: send buffer to the stats thread
        Vec::from(&buffer[..num_read]);
    }

    //TODO: send buffer empty to stats thread
    let mut quit = quit.lock().unwrap();
    *quit = true;
    Ok(())
}
