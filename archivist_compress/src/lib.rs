// TODO: Merge duplicate code

#[macro_use]
extern crate clap;

pub mod logic;
pub mod sub_commands;

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

pub mod gz {
    use flate2::read::GzDecoder;
    use flate2::write::GzEncoder;
    use flate2::Compression;
    use std::fs::File;
    use std::io;

    pub fn compress(source: &str, dest: &str, level: u32) -> io::Result<u64> {
        if level > 9 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "invalid compression level",
            ));
        }
        let level = Compression::new(level);
        let dest_file = File::create(dest)?;
        let mut dest_compressor = GzEncoder::new(dest_file, level);
        let mut source_file = File::open(source)?;

        io::copy(&mut source_file, &mut dest_compressor)
    }

    pub fn decompress(source: &str, dest: &str) -> io::Result<u64> {
        let source_file = File::open(source)?;
        let mut source_decompressor = GzDecoder::new(source_file);
        let mut dest_file = File::create(dest)?;

        io::copy(&mut source_decompressor, &mut dest_file)
    }
}

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
    use std::fs;
    use std::path::PathBuf;

    fn out_path(path: &str) -> String {
        let mut out_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        out_path.pop();
        out_path.push("target");
        out_path.push(path);

        out_path
            .to_str()
            .expect("Can't convert path to string")
            .to_string()
    }

    fn lorem(path: &str) -> String {
        let contents = include_str!("../resources/tests/lorem.txt").to_owned();

        let dest_path = out_path(path);
        fs::write(&dest_path, contents).expect("Can't write to file");
        dest_path
    }

    mod bz2 {
        use crate::bz2;
        use crate::tests::{lorem, out_path};
        use std::fs;

        #[test]
        fn compress_decompress() {
            let source = lorem("lorem.b.txt");
            let source_contents = fs::read_to_string(&source).expect("can't read source");
            let dest = out_path("lorem.txt.bz2");
            bz2::compress(&source, &dest, 1).expect("compression failed");

            let source = dest;
            let dest = out_path("lorem.bz2.txt");
            bz2::decompress(&source, &dest).expect("decompression failed");
            let dest_contents = fs::read_to_string(&dest).expect("can't read dest");

            assert_eq!(source_contents, dest_contents);

            fs::remove_file(&source).expect("can't delete temporary file");
            fs::remove_file(&dest).expect("can't delete temporary file");
        }
    }

    mod gz {
        use crate::gz;
        use crate::tests::{lorem, out_path};
        use std::fs;

        #[test]
        fn compress_decompress() {
            let source = lorem("lorem.g.txt");
            let source_contents = fs::read_to_string(&source).expect("can't read source");
            let dest = out_path("lorem.txt.gz");
            gz::compress(&source, &dest, 1).expect("compression failed");

            let source = dest;
            let dest = out_path("lorem.gz.txt");
            gz::decompress(&source, &dest).expect("decompression failed");
            let dest_contents = fs::read_to_string(&dest).expect("can't read dest");

            assert_eq!(source_contents, dest_contents);

            fs::remove_file(&source).expect("can't delete temporary file");
            fs::remove_file(&dest).expect("can't delete temporary file");
        }
    }

    mod xz {
        use crate::tests::{lorem, out_path};
        use crate::xz;
        use std::fs;

        #[test]
        fn compress_decompress() {
            let source = lorem("lorem.x.txt");
            let source_contents = fs::read_to_string(&source).expect("can't read source");
            let dest = out_path("lorem.txt.xz");
            xz::compress(&source, &dest, 1).expect("compression failed");

            let source = dest;
            let dest = out_path("lorem.xz.txt");
            xz::decompress(&source, &dest).expect("decompression failed");
            let dest_contents = fs::read_to_string(&dest).expect("can't read dest");

            assert_eq!(source_contents, dest_contents);

            fs::remove_file(&source).expect("can't delete temporary file");
            fs::remove_file(&dest).expect("can't delete temporary file");
        }
    }
}
