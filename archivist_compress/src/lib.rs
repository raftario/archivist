use std::io;
use std::io::Read;

fn _read_to_bytes<T: Read>(mut source: T) -> Result<Vec<u8>, io::Error> {
    let mut buffer = Vec::new();
    source.read_to_end(&mut buffer)?;

    Ok(buffer)
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
