extern crate git2;
#[macro_use]
extern crate quick_error;

use git2::Repository;

fn main()
{
    let repo = Repository::discover("/Users/ceej/Dropbox/projects/rusty-things/git-latest-commit").unwrap();
    let oid_o = repo.refname_to_id("HEAD");

    let oid = match oid_o
    {
        Ok(v) => v,
        Err(_) => { std::process::exit(0); },
    };
    println!("sha: {:?}", oid);

    let mut commit = repo.find_commit(oid).unwrap();
    let sumbytes = commit.summary_bytes().unwrap();
    let summary = std::str::from_utf8(&sumbytes).unwrap();
    println!("mesg: {:?}", summary);
}
