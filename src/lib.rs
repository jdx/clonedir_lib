extern crate reflink;

use reflink::reflink_or_copy;
use std::convert::AsRef;
use std::fs;
use std::io;
use std::path::Path;

/// Clones a directory with reflink
/// https://docs.rs/reflink/*/reflink/index.html
/// Some file systems implement COW (copy on write) functionality in order to speed up file copies. On a high level, the new file does not actually get copied, but shares the same on-disk data with the source file. As soon as one of the files is modified, the actual copying is done by the underlying OS.
///
/// # Examples
///
/// ```
/// use std::fs;
/// use std::io::prelude::*;
///
/// fn main() {
///     clonedir_lib::clonedir("src", "tmp/src").expect("error downloading");
///     let contents = fs::read_to_string("tmp/src/lib.rs").expect("error reading file");
///     assert!(contents.starts_with("extern"));
/// }
/// ```
pub fn clonedir<A: AsRef<Path>, B: AsRef<Path>>(from: A, to: B) -> io::Result<()> {
    let from = from.as_ref();
    let to = to.as_ref();
    println!("cloning dir {:?} to {:?}", &from, &to);
    for f in fs::read_dir(from)?.into_iter().map(|f| f.unwrap().path()) {
        fs::create_dir_all(&to)?;
        if f.is_dir() {
            clonedir(&f, &to)?;
        } else if f.is_file() {
            println!("cloning {:?} to {:?}", &f, &to);
            reflink_or_copy(&f, to.join(f.file_name().unwrap()))?;
        }
    }

    Ok(())
}
