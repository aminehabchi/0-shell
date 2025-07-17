mod directory;
mod file;
mod helpers;
mod ls;
mod file_helpers;
use crate::ls::Ls;

pub fn ls(args: &[&str]) {
    let ls: Ls = match Ls::new(&args) {
        Some(l) => l,
        None => {
            return;
        }
    };

    ls.print();
}
