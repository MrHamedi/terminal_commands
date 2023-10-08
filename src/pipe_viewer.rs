
use std::io::{self, Read, Write};

const CHUNK_SIZE:usize = 16*1024;

pub fn pipe_viewer_main(){
    let mut total_bytes=0;
    loop{
        let mut buffer = [0; CHUNK_SIZE];
        let mut num_read = match io::stdin().read(buf:&mut buffer){
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };
        io::stdout().write_all(&mut buffer[..num_read]).unwrap();
        total_bytes=total_bytes+num_read;
    }
    eprintln!("readed count : {}", total_bytes);
}