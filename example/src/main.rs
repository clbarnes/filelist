use clap::Parser;
use filelist::{expand_osstr, Argument, Split, WalkDirOptions};
use std::ffi::OsString;
use std::fs::File;
use std::path::PathBuf;


#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    recursive: bool,
    #[clap(short, long)]
    sorted: bool,
    #[clap(short = 'd', long)]
    in_delimiter: Option<OsString>,
    #[clap(short, long)]
    from_file: Vec<PathBuf>,
    files: Vec<OsString>,
}

fn main() {
    let args = Args::parse();
    let delim: Split = args
        .in_delimiter
        .unwrap_or(OsString::from("\n"))
        .try_into()
        .expect("Could not parse delimiter");
    let mut walk_opts = None;
    if args.recursive {
        walk_opts = Some(WalkDirOptions::default().sort(args.sorted));
    }

    let mut fpaths = Vec::default();
    for fp in args.from_file.into_iter() {
        let mut f = File::open(fp).expect("Could not read file");
        fpaths.extend(Argument::Reader(Box::new(&mut f)).expand(delim.into(), &walk_opts));
    }
    fpaths.extend(expand_osstr(args.files, delim, walk_opts));

    for fpath in fpaths.into_iter() {
        println!(
            "{}",
            fpath
                .into_os_string()
                .into_string()
                .expect("Could not coerce path to string")
        );
    }
}
