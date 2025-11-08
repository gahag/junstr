use std::path::PathBuf;

#[derive(Debug, clap::Parser)]
pub struct Cli {
	/// Operate at the given RFC6901 pointers in the json.
	#[arg(long, default_values_t = ["".to_string()])]
	pub at: Vec<String>,
	/// Input, "-" indicates stdin.
	#[arg(short, long = "in", default_value = "-")]
	pub input: PathBuf,
	/// Whether to pretty print the output.
	#[arg(short, long)]
	pub compact: bool,
}

