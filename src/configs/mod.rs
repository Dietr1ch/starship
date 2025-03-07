pub mod aws;
pub mod battery;
pub mod character;
pub mod cmake;
pub mod cmd_duration;
pub mod conda;
pub mod crystal;
pub mod custom;
pub mod dart;
pub mod directory;
pub mod docker_context;
pub mod dotnet;
pub mod elixir;
pub mod elm;
pub mod env_var;
pub mod erlang;
pub mod gcloud;
pub mod git_branch;
pub mod git_commit;
pub mod git_state;
pub mod git_status;
pub mod go;
pub mod helm;
pub mod hg_branch;
pub mod hostname;
pub mod java;
pub mod jobs;
pub mod julia;
pub mod kubernetes;
pub mod memory_usage;
pub mod nim;
pub mod nix_shell;
pub mod nodejs;
pub mod ocaml;
pub mod package;
pub mod perl;
pub mod php;
pub mod purescript;
pub mod python;
pub mod ruby;
pub mod rust;
pub mod shlvl;
pub mod singularity;
mod starship_root;
pub mod swift;
pub mod terraform;
pub mod time;
pub mod username;
pub mod zig;

pub use starship_root::*;
