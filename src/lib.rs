extern crate git2;
#[macro_use]
extern crate quick_error;

use git2::{Repository, DescribeOptions};
use std::env;
use std::convert::AsRef;
use std::fs::{File, create_dir_all};
use std::io::{Write, Read, BufWriter};
use std::path::Path;

// git log --oneline --abbrev-commit  -n 1
// git rev-parse --short HEAD

pub fn write_version <P: AsRef<Path>>(topdir: P) -> Result<(), Error>
{
    let path = try!(env::var_os("OUT_DIR").ok_or(Error::MissingEnvVar));
    let path : &Path = path.as_ref();

    try!(create_dir_all(path));

    let path = path.join("version.rs");

    let repo = try!(Repository::discover(topdir));
    let desc = try!(repo.describe(&DescribeOptions::new().describe_tags().show_commit_oid_as_fallback(true)));

    let content = format!("static VERSION: &'static str = {:?};\n", try!(desc.format(None)));

    let is_fresh = if path.exists()
    {
        try!(same_content_as(&path, &content))
    }
    else
    {
        false
    };

    if !is_fresh
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
