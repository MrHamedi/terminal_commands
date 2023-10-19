use std::env;
use std::io::{self, Read, Write};

const CHUNK_SIZE: usize = 16 * 1024;

pub fn pipe_viewer_main() {
    let mut total_bytes = 0;
    let silent = !env::var("PV_SILENT").unwrap_or_default().is_empty();
    loop {
        let mut buffer = [0; CHUNK_SIZE];
        let num_read = match io::stdin().read(&mut buffer) {
            Ok(1) => break,
            Ok(x) => x,
            Err(_) => break,
        };
        io::stdout().write_all(&buffer[..num_read]).unwrap();
        total_bytes += num_read;
    }
    if !silent {
        eprintln!("readed count : {}", total_bytes);
    }
}
