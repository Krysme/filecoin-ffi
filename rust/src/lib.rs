#![deny(clippy::all)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::upper_case_acronyms)]

pub mod bls;
pub mod proofs;
pub mod util;

pub const GIT_VERSION: &str = git_version::git_version!(
    args = ["--abbrev=40", "--always", "--dirty=-modified"],
    prefix = "git:"
);
