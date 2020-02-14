//! Substrate Node Template CLI library.
#![warn(missing_docs)]

mod chain_spec;
#[macro_use]
mod service;
mod cli;
mod command;

pub use sc_cli::{VersionInfo, error};

fn main() -> Result<(), error::Error> {
	let version = VersionInfo {
		name: "Substrate Permissioning Sample Node",
		commit: env!("VERGEN_SHA_SHORT"),
		version: env!("CARGO_PKG_VERSION"),
		executable_name: "permissioning-sample",
		author: "gautamdhameja",
		description: "Substrate Permissioning Sample",
		support_url: "support.anonymous.an",
		copyright_start_year: 2017,
	};

	command::run(version)
}
