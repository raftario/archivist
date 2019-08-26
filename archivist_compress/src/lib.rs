// TODO: Merge duplicate code

pub mod bz2 {
    use bzip2::read::{BzDecoder, BzEncoder};
    use bzip2::Compression;
    use std::fs::File;
    use std::io;

    pub fn compress(source: &str, dest: &str, level: u32) -> io::Result<u64> {
        let level = match level {
            0 => Compression::Fastest,
            1 => Compression::Default,
            2 => Compression::Best,
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "invalid compression level",
                ));
            }
        };
        let dest_file = File::create(dest)?;
        let mut dest_compressor = BzEncoder::new(dest_file, level);
        let mut source_file = File::open(source)?;

        io::copy(&mut source_file, &mut dest_compressor)
    }

    pub fn decompress(source: &str, dest: &str) -> io::Result<u64> {
        let source_file = File::open(source)?;
        let mut source_decompressor = BzDecoder::new(source_file);
        let mut dest_file = File::create(dest)?;

        io::copy(&mut source_decompressor, &mut dest_file)
    }
}

pub mod gz {}

pub mod xz {
    use std::fs::File;
    use std::io;
    use xz2::read::{XzDecoder, XzEncoder};

    pub fn compress(source: &str, dest: &str, level: u32) -> io::Result<u64> {
        if level > 9 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "invalid compression level",
            ));
        }
        let dest_file = File::create(dest)?;
        let mut dest_compressor = XzEncoder::new(dest_file, level);
        let mut source_file = File::open(source)?;

        io::copy(&mut source_file, &mut dest_compressor)
    }

    pub fn decompress(source: &str, dest: &str) -> io::Result<u64> {
        let source_file = File::open(source)?;
        let mut source_decompressor = XzDecoder::new(source_file);
        let mut dest_file = File::create(dest)?;

        io::copy(&mut source_decompressor, &mut dest_file)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
