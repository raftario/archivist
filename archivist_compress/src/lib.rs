// TODO: Merge duplicate code

pub mod bz2 {
    use bzip2::read::BzDecoder;
    use bzip2::write::BzEncoder;
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
    use xz2::read::XzDecoder;
    use xz2::write::XzEncoder;

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
    use std::path::PathBuf;

    fn path_relative(relative_path: &str) -> String {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("src");
        path.push("tests");
        let relative_path = PathBuf::from(relative_path);
        path.push(relative_path);

        path.to_str().expect("can't parse path").to_string()
    }

    mod bz2 {
        use crate::bz2;
        use crate::tests::path_relative;
        use std::fs;

        #[test]
        fn compress_decompress() {
            let source = path_relative("lorem.txt");
            let source_contents = fs::read_to_string(&source).expect("can't read source");
            let dest = path_relative("lorem.txt.bz2");
            bz2::compress(&source, &dest, 1).expect("compression failed");

            let source = dest;
            let dest = path_relative("lorem.bz2.txt");
            bz2::decompress(&source, &dest).expect("decompression failed");
            let dest_contents = fs::read_to_string(&dest).expect("can't read dest");

            assert_eq!(source_contents, dest_contents);

            fs::remove_file(&source).expect("can't delete temporary file");
            fs::remove_file(&dest).expect("can't delete temporary file");
        }
    }

    mod xz {
        use crate::tests::path_relative;
        use crate::xz;
        use std::fs;

        #[test]
        fn compress_decompress() {
            let source = path_relative("lorem.txt");
            let source_contents = fs::read_to_string(&source).expect("can't read source");
            let dest = path_relative("lorem.txt.xz");
            xz::compress(&source, &dest, 1).expect("compression failed");

            let source = dest;
            let dest = path_relative("lorem.xz.txt");
            xz::decompress(&source, &dest).expect("decompression failed");
            let dest_contents = fs::read_to_string(&dest).expect("can't read dest");

            assert_eq!(source_contents, dest_contents);

            fs::remove_file(&source).expect("can't delete temporary file");
            fs::remove_file(&dest).expect("can't delete temporary file");
        }
    }
}
