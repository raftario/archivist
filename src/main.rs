use clap::App;

fn main() {
    let _matches = App::new("The Archivist")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .subcommand(archivist_compress::sub_commands::compress())
        .get_matches();
}
