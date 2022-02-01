pub use jwalk::Parallelism;
use jwalk::WalkDir;
use std::ffi::OsString;
use std::io::{stdin, BufRead, BufReader, Read};
use std::os::unix::ffi::OsStringExt;
use std::path::PathBuf;

pub struct WalkDirOptions {
    sort: bool,
    min_depth: usize,
    max_depth: usize,
    skip_hidden: bool,
    follow_links: bool,
    parallelism: Parallelism,
    // root_read_dir_state: C::ReadDirState,
    // process_read_dir: Option<Arc<ProcessReadDirFunction<C>>>,
}

impl Default for WalkDirOptions {
    fn default() -> Self {
        WalkDirOptions {
            sort: false,
            min_depth: 0,
            max_depth: ::std::usize::MAX,
            skip_hidden: true,
            follow_links: false,
            parallelism: Parallelism::RayonDefaultPool,
        }
    }
}

impl WalkDirOptions {
    pub fn sort(mut self, sort: bool) -> Self {
        self.sort = sort;
        self
    }

    pub fn min_depth(mut self, min_depth: usize) -> Self {
        self.min_depth = min_depth;
        self
    }

    pub fn max_depth(mut self, max_depth: usize) -> Self {
        self.max_depth = max_depth;
        self
    }

    pub fn skip_hidden(mut self, skip_hidden: bool) -> Self {
        self.skip_hidden = skip_hidden;
        self
    }

    pub fn follow_links(mut self, follow_links: bool) -> Self {
        self.follow_links = follow_links;
        self
    }

    pub fn parallelism(mut self, parallelism: Parallelism) -> Self {
        self.parallelism = parallelism;
        self
    }

    pub fn build(&self, path: PathBuf) -> WalkDir {
        WalkDir::new(path)
            .sort(self.sort)
            .min_depth(self.min_depth)
            .max_depth(self.max_depth)
            .skip_hidden(self.skip_hidden)
            .follow_links(self.follow_links)
            .parallelism(self.parallelism.clone())
    }
}

pub enum Argument<'a> {
    Reader(Box<&'a mut dyn Read>),
    Path(PathBuf),
}

impl<'a> Argument<'a> {
    pub fn expand(self, split: u8, walk_opts: &Option<WalkDirOptions>) -> Vec<PathBuf> {
        let mut out = Vec::default();
        match self {
            Argument::Reader(r) => {
                for b in BufReader::new(r).split(split) {
                    let arg = Self::Path(PathBuf::from(OsString::from_vec(b.unwrap())));
                    out.extend(arg.expand(split, walk_opts));
                }
            }
            Argument::Path(root) => {
                // todo: more error checking
                if root.is_file() {
                    out.push(root);
                } else if let Some(w) = walk_opts {
                    for p in w.build(root) {
                        let unwrapped = p.unwrap();

                        if unwrapped.file_type().is_file() {
                            out.push(unwrapped.path());
                        }
                    }
                }
            }
        }
        out
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Split {
    Newline,
    Null,
    Tab,
    Other(u8),
}

impl From<Split> for u8 {
    fn from(split: Split) -> Self {
        match split {
            Split::Newline => b'\n',
            Split::Null => b'\0',
            Split::Tab => b'\t',
            Split::Other(x) => x,
        }
    }
}

impl TryFrom<OsString> for Split {
    type Error = ();

    fn try_from(s: OsString) -> Result<Split, ()> {
        let as_str = s.to_str().ok_or(())?;
        match as_str {
            "\\n" => return Ok(Split::Newline),
            "\\t" => return Ok(Split::Tab),
            "\\0" => return Ok(Split::Null),
            _ => {}
        };
        if as_str.len() != 1 {
            return Err(());
        }
        match as_str.bytes().next().unwrap() {
            b'\n' => Ok(Split::Newline),
            b'\0' => Ok(Split::Null),
            b'\t' => Ok(Split::Tab),
            x => Ok(Split::Other(x)),
        }
    }
}

pub fn expand_arguments<T: Into<u8>>(
    args: Vec<Argument>,
    split: T,
    walk_opts: Option<WalkDirOptions>,
) -> Vec<PathBuf> {
    let s: u8 = split.into();
    args.into_iter()
        .map(|a| a.expand(s, &walk_opts))
        .flatten()
        .collect()
}

pub fn expand_osstr<T: Into<u8>>(
    strs: Vec<OsString>,
    split: T,
    walk_opts: Option<WalkDirOptions>,
) -> Vec<PathBuf> {
    let dash = OsString::from("-");
    let mut sin_idx: Option<usize> = None;
    let mut args = Vec::with_capacity(strs.len());
    for (idx, s) in strs.iter().enumerate() {
        if s == &dash {
            if sin_idx.is_none() {
                sin_idx = Some(idx);
            }
        } else {
            args.push(Argument::Path(PathBuf::from(s)))
        }
    }
    if let Some(sidx) = sin_idx {
        let mut sin = stdin();
        args.insert(sidx, Argument::Reader(Box::new(&mut sin)));
        expand_arguments(args, split, walk_opts)
    } else {
        expand_arguments(args, split, walk_opts)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
