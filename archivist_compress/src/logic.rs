use crate::{bz2, gz, xz};
use clap::ArgMatches;
use std::io;
use std::process;

fn e_nei() -> ! {
    eprintln!("Not enough information was provided to determine the desired operation.");
    process::exit(1)
}

fn e_unexpected(e: &str) -> ! {
    eprintln!(
        "{} (this shouldn't have happened, please file a bug report)",
        e
    );
    process::exit(1)
}

fn e_level() -> ! {
    eprintln!("The level must be an integer number.");
    process::exit(1)
}

fn e_compression(e: io::Error) -> ! {
    eprintln!("Compression error: {}", e);
    process::exit(1)
}

pub fn compress(matches: &ArgMatches) {
    let source = matches
        .value_of("SOURCE")
        .unwrap_or_else(|| e_unexpected("Source not specified"));
    let dest = matches.value_of("DESTINATION");
    let (mut is_gz, mut is_xz, mut is_bz2) = (
        matches.is_present("gz"),
        matches.is_present("xz"),
        matches.is_present("bz2"),
    );
    let algo = is_gz || is_xz || is_bz2;
    let mut level = matches.value_of("level");

    let mut ext: Option<String> = None;

    match (dest, algo) {
        (None, false) => e_nei(),
        // Destination is specified but not algorithm, try to get algorithm from extension
        (Some(dest), false) => {
            let ext = dest
                .split('.')
                .collect::<Vec<&str>>()
                .last()
                .unwrap_or_else(|| e_nei())
                .to_owned();
            match ext {
                "gz" => is_gz = true,
                "xz" => is_xz = true,
                "bz2" => is_bz2 = true,
                _ => e_nei(),
            };
        }
        // Algorithm is specified but not destination, determine file name from algorithm
        (None, true) => {
            ext = match (is_gz, is_xz, is_bz2) {
                (true, false, false) => Some(".gz".to_owned()),
                (false, true, false) => Some(".xz".to_owned()),
                (false, false, true) => Some(".bz2".to_owned()),
                _ => e_unexpected("Invalid algorithm"),
            };
        }
        (Some(_), true) => (),
    };

    // Use default level if unspecified
    if level.is_none() {
        level = Some(match (is_gz, is_xz, is_bz2) {
            (true, false, false) => "6",
            (false, true, false) => "6",
            (false, false, true) => "1",
            _ => e_unexpected("Invalid algorithm"),
        });
    }

    // These values should always be Some(&str) at this point
    let level: u32 = level
        .unwrap_or_else(|| e_unexpected("'level' is None"))
        .parse()
        .unwrap_or_else(|_| e_level());
    // If there's no destination, extension should be specified
    #[allow(clippy::or_fun_call)]
    let dest = dest
        .unwrap_or(format!("{}{}", source, ext.unwrap_or_else(|| "".to_owned())).as_ref())
        .to_owned();

    match (is_gz, is_xz, is_bz2) {
        (true, false, false) => gz::compress(source, &dest, level),
        (false, true, false) => xz::compress(source, &dest, level),
        (false, false, true) => bz2::compress(source, &dest, level),
        _ => e_unexpected("Invalid algorithm"),
    }
    .unwrap_or_else(|e| e_compression(e));
}

pub fn decompress(matches: &ArgMatches) {
    let source = matches
        .value_of("SOURCE")
        .unwrap_or_else(|| e_unexpected("Source not specified"));
    let mut dest = matches.value_of("DESTINATION");
    let (mut is_gz, mut is_xz, mut is_bz2) = (
        matches.is_present("gz"),
        matches.is_present("xz"),
        matches.is_present("bz2"),
    );
    let mut algo = is_gz || is_xz || is_bz2;

    if !algo {
        match source.split('.').collect::<Vec<&str>>().last() {
            Some(&"gz") => {
                is_gz = true;
                if dest.is_none() {
                    let len = source.len() - 3;
                    dest = Some(&source[..len]);
                };
            }
            Some(&"xz") => {
                is_xz = true;
                if dest.is_none() {
                    let len = source.len() - 3;
                    dest = Some(&source[..len]);
                };
            }
            Some(&"bz2") => {
                is_bz2 = true;
                if dest.is_none() {
                    let len = source.len() - 4;
                    dest = Some(&source[..len]);
                };
            }
            _ => (),
        };
        algo = is_gz || is_xz || is_bz2;
        if !algo {
            e_nei();
        };
    }

    // This values should always be Some(&str) at this point
    let dest = dest.unwrap_or_else(|| e_unexpected("'dest' is None"));

    match (is_gz, is_xz, is_bz2) {
        (true, false, false) => gz::decompress(source, dest),
        (false, true, false) => xz::decompress(source, dest),
        (false, false, true) => bz2::decompress(source, dest),
        _ => e_unexpected("Invalid algorithm"),
    }
    .unwrap_or_else(|e| e_compression(e));
}
