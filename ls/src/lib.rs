mod directory;
mod file;
mod helpers;
mod ls;

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
