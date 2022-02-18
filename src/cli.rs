use clap::{crate_authors, crate_version, Arg, Command};

pub fn cli() -> Command<'static> {
    Command::new(String::from(env!("CARGO_PKG_NAME")))
        .bin_name(String::from(env!("CARGO_PKG_NAME")))
        .version(crate_version!())
        .author(crate_authors!())
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::new("x1").help("coordinate 1 x").required(true))
        .arg(Arg::new("y1").help("coordinate 1 y").required(true))
        .arg(Arg::new("z1").help("coordinate 1 z").required(true))
        .arg(Arg::new("x2").help("coordinate 2 x").required(true))
        .arg(Arg::new("y2").help("coordinate 2 y").required(true))
        .arg(Arg::new("z2").help("coordinate 2 z").required(true))
}
