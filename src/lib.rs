extern crate git2;
#[macro_use]
extern crate quick_error;

use git2::Repository;
use std::convert::AsRef;
use std::env;
use std::fs::{File, create_dir_all};
use std::io::{Write, Read, BufWriter};
use std::path::Path;

quick_error!
{
    #[derive(Debug)]
    pub enum Error
    {
        Io(err: std::io::Error) { from() }
        Git(err: git2::Error) { from() }
        MissingEnvVar { }
    }
}

fn content_differs<P: AsRef<Path>>(path: P, content: &str) -> Result<bool, Error>
{
    let mut f = try!(File::open(path));
    let mut current = String::new();
    try!(f.read_to_string(&mut current));

    Ok(current != content)
}

pub fn write <P: AsRef<Path>>(topdir: P) -> Result<(), Error>
{
    let path = try!(env::var_os("OUT_DIR").ok_or(Error::MissingEnvVar));
    let path : &Path = path.as_ref();

    try!(create_dir_all(path));

    let path = path.join("git-commit.rs");

    let repo = try!(Repository::discover(topdir));
    let oid = try!(repo.refname_to_id("HEAD"));

    let mut commit = repo.find_commit(oid).unwrap();
    let sumbytes = commit.summary_bytes().unwrap();
    let summary = std::str::from_utf8(&sumbytes).unwrap();

    let content = format!("static GIT_HASH: &'static str = \"{}\";\nstatic GIT_SUMMARY: &'static str = \"{}\";",
        oid, summary);

    let has_changes = if path.exists()
    {
        try!(content_differs(&path, &content))
    }
    else { true };

    if has_changes
    {
      let mut file = BufWriter::new(try!(File::create(&path)));
      try!(write!(file, "{}", content));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
