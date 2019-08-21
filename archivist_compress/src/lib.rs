use std::fs::File;
use std::io;
use std::io::{Read, Write};

fn _read_file_to_bytes(path: &str) -> io::Result<Vec<u8>> {
    let mut buffer = Vec::new();
    let mut file = File::open(path)?;
    file.read_to_end(&mut buffer)?;

    Ok(buffer)
}

fn _write_bytes_to_file(path: &str, buffer: Vec<u8>) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(&buffer)?;
    file.flush()
}

pub mod bz2 {}

pub mod gz {}

pub mod xz {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
