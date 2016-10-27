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

fn main()
{
    let repo = try!(Repository::discover("."));
    // let shahead = try!(repo.revparse());
    let desc = try!(repo.describe(&DescribeOptions::new().describe_tags().show_commit_oid_as_fallback(true)));
    println!("sha: {:?}", desc);

    // let content = format!("static VERSION: &'static str = {:?};\n", try!(desc.format(None)));

    Ok(())
}
