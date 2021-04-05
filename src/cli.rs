use clap::{crate_authors, crate_version, App, Arg};

pub fn app() -> App<'static> {
    App::new(String::from(env!("CARGO_PKG_NAME")))
        .bin_name(String::from(env!("CARGO_PKG_NAME")))
        .version(crate_version!())
        .author(crate_authors!())
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::new("x1").about("coordinate 1 x").required(true))
        .arg(Arg::new("y1").about("coordinate 1 y").required(true))
        .arg(Arg::new("z1").about("coordinate 1 z").required(true))
        .arg(Arg::new("x2").about("coordinate 2 x").required(true))
        .arg(Arg::new("y2").about("coordinate 2 y").required(true))
        .arg(Arg::new("z2").about("coordinate 2 z").required(true))
}
