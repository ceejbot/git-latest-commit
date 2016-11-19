# git-latest-commit

[![crate](https://img.shields.io/crates/v/git-latest-commit.svg)](https://crates.io/crates/git-latest-commit)

Makes a file available to a Rust project at build time containing the latest commit's sha and summary as static vars.

Shamelessly adapted from [cstorey/git-build-version](https://github.com/cstorey/git-build-version).

Invoke this tool in your package build step. In `Cargo.toml`, add it as a build dep:

```toml
[package]
name = "something-gratuitously-mentioning-rust"
build = "build.rs"

[build-dependencies]
git-latest-commit = "0.1.3"
```

Then, in `build.rs` at the top level of your project:

```rs
extern crate git_latest_commit;

const PACKAGE_TOP_DIR : &'static str = ".";

fn main()
{
    git_latest_commit::write(PACKAGE_TOP_DIR).expect("Exporting git commit info.");
}
```

## License

ISC
