# filelist

Expanding lists of paths in rust.

Command line tools which take (potentially large numbers of) files may be interested in 2-3 types of arguments:

1. An individual file path
    - Due to shell limitations, passing very large numbers of file paths this way may degrade performance or break the shell
2. An existing list of file paths (e.g. a text file of newline-separated paths, or standard input)
3. Sometimes, a directory path, where child files should be included (recursively)
    - This can be achieved by piping out the output of a tool like `fd` / `find` into option 2, but sometimes it's convenient to do it directly

This is a utility which takes a mixture of those 3 options and converts them into a `Vec<PathBuf>`.

Currently unix-only, and only handles UTF-8 paths.
