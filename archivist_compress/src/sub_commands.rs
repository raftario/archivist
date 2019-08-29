use clap::{App, Arg, ArgGroup, SubCommand};

pub fn compress<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("compress")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Compresses single files using common compression algorithms")
        .aliases(&["c"])
        .display_order(1)
        .arg(
            Arg::with_name("SOURCE")
                .help("Source file to compress")
                .required(true)
                .index(1)
                .display_order(1),
        )
        .arg(
            Arg::with_name("DESTINATION")
                .help("Destination file to write")
                .index(2)
                .display_order(2)
                .required_unless("algo"),
        )
        .arg(
            Arg::with_name("gz")
                .help("Use gzip compression (levels from 0 to 9, default is 6)")
                .long("gz")
                .short("g")
                .aliases(&["gzip"])
                .display_order(1),
        )
        .arg(
            Arg::with_name("xz")
                .help("Use lzma compression (levels from 0 to 9, default is 6)")
                .long("xz")
                .short("x")
                .aliases(&["lzma"])
                .display_order(2),
        )
        .arg(
            Arg::with_name("bz2")
                .help("Use bzip2 compression (levels from 0 to 2, default is 1)")
                .long("bz2")
                .short("b")
                .aliases(&["bzip2"])
                .display_order(3),
        )
        .group(ArgGroup::with_name("algo").args(&["gz", "xz", "bz2"]))
        .arg(
            Arg::with_name("level")
                .help("Specifies the compression level")
                .long("level")
                .short("l")
                .takes_value(true)
                .value_name("LEVEL")
                .display_order(4),
        )
        .arg(
            Arg::with_name("overwrite")
                .help("Overwrites the destination file if it already exists")
                .long("overwrite")
                .short("o")
                .display_order(5),
        )
}

pub fn decompress<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("decompress")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Decompresses single files using common decompression algorithms")
        .aliases(&["d"])
        .display_order(2)
        .arg(
            Arg::with_name("SOURCE")
                .help("Source file to decompress")
                .required(true)
                .index(1)
                .display_order(1),
        )
        .arg(
            Arg::with_name("DESTINATION")
                .help("Destination file to write")
                .index(2)
                .display_order(2),
        )
        .arg(
            Arg::with_name("gz")
                .help("Use gzip decompression")
                .long("gz")
                .short("g")
                .aliases(&["gzip"])
                .display_order(1),
        )
        .arg(
            Arg::with_name("xz")
                .help("Use lzma decompression")
                .long("xz")
                .short("x")
                .aliases(&["lzma"])
                .display_order(2),
        )
        .arg(
            Arg::with_name("bz2")
                .help("Use bzip2 decompression")
                .long("bz2")
                .short("b")
                .aliases(&["bzip2"])
                .display_order(3),
        )
        .group(ArgGroup::with_name("algo").args(&["gz", "xz", "bz2"]))
        .arg(
            Arg::with_name("overwrite")
                .help("Overwrites the destination file if it already exists")
                .long("overwrite")
                .short("o")
                .display_order(4),
        )
}
