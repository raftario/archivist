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
                .help("Use gzip compression with specified level (0: fastest to 9: best)")
                .long("gz")
                .short("g")
                .aliases(&["gzip"])
                .value_name("LEVEL")
                .takes_value(true)
                .default_value("6")
                .display_order(1),
        )
        .arg(
            Arg::with_name("xz")
                .help("Use lzma compression with specified level (0: fastest to 9: best)")
                .long("xz")
                .short("x")
                .aliases(&["lzma"])
                .value_name("LEVEL")
                .takes_value(true)
                .default_value("6")
                .display_order(2),
        )
        .arg(
            Arg::with_name("bz2")
                .help(
                    "Use bzip2 compression with specified level (0: fastest, 1: default, 2: best)",
                )
                .long("bz2")
                .short("b")
                .aliases(&["bzip2"])
                .value_name("LEVEL")
                .takes_value(true)
                .default_value("1")
                .display_order(3),
        )
        .group(
            ArgGroup::with_name("algo")
                .args(&["gz", "xz", "bz2"])
                .required(true),
        )
}

pub fn decompress<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("decompress")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
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
                .help("Use gzip compression with specified level (0: fastest to 9: best)")
                .long("gz")
                .short("g")
                .aliases(&["gzip"])
                .display_order(1),
        )
        .arg(
            Arg::with_name("xz")
                .help("Use lzma compression with specified level (0: fastest to 9: best)")
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
}
