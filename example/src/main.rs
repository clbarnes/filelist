use clap::Parser;
use std::ffi::OsString;
use filelist::{WalkDirOptions, expand_osstr, Split};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    recursive: bool,
    #[clap(short, long)]
    sorted: bool,
    #[clap(short = 'd', long)]
    in_delimiter: Option<OsString>,
    files: Vec<OsString>,
}

fn main() {
    let args = Args::parse();
    let delim: Split = args.in_delimiter.unwrap_or(OsString::from("\n")).try_into().expect("Delimiter must be single-byte character");
    let mut walk_opts = None;
    if args.recursive {
        walk_opts = Some(WalkDirOptions::default().sort(args.sorted));
    }
    let fpaths = expand_osstr(args.files, delim, walk_opts);
    for fpath in fpaths.into_iter() {
        println!("{}", fpath.into_os_string().into_string().unwrap());
    }
}
