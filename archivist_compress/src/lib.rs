pub mod sub_commands {
    use clap::{App, Arg, ArgGroup, SubCommand};

    pub fn compress<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("compress")
            .version(clap::crate_version!())
            .author(clap::crate_authors!())
            .about("Compresses single files using common compression algorithms")
            .aliases(&["c"])
            .display_order(1)
            .arg(
                Arg::with_name("SOURCE")
                    .help("Source file to compress")
                    .required(true)
                    .index(1)
            )
            .arg(
                Arg::with_name("DESTINATION")
                    .help("Destination file to write")
                    .index(2)
            )
            .arg(
                Arg::with_name("bz2")
                    .help("Use bzip2 compression with specified level (0: fastest, 1: default, 2: best)")
                    .long("bz2")
                    .aliases(&["bzip2"])
                    .value_name("LEVEL")
                    .takes_value(true)
                    .default_value("1")
            )
            .arg(
                Arg::with_name("gz")
                    .help("Use gzip compression with specified level (0: fastest to 9: best)")
                    .long("gz")
                    .aliases(&["gzip"])
                    .value_name("LEVEL")
                    .takes_value(true)
                    .default_value("6")
            )
            .arg(
                Arg::with_name("xz")
                    .help("Use lzma compression with specified level (0: fastest to 9: best)")
                    .long("xz")
                    .aliases(&["lzma"])
                    .value_name("LEVEL")
                    .takes_value(true)
                    .default_value("6")
            )
            .group(
                ArgGroup::with_name("algo")
                    .args(&["bz2", "gz", "xz"])
                    .required(true)
            )
    }
}

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

    mod gz {
        use crate::gz;
        use crate::tests::path_relative;
        use std::fs;

        #[test]
        fn compress_decompress() {
            let source = path_relative("lorem.txt");
            let source_contents = fs::read_to_string(&source).expect("can't read source");
            let dest = path_relative("lorem.txt.gz");
            gz::compress(&source, &dest, 1).expect("compression failed");

            let source = dest;
            let dest = path_relative("lorem.gz.txt");
            gz::decompress(&source, &dest).expect("decompression failed");
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
