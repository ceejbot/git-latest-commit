extern crate git_latest_commit;

fn main()
{
    match git_latest_commit::write(".")
    {
        Ok(_) => { println!("wrote latest commit data to git-commit.rs"); },
        Err(e) => { println!("{:?}", e); },
    };
}
