use std::{fs, io::{self, Read}};

use clap::Parser;
use derive_more::{Display, Error, From};
use serde_json as json;

mod args;

type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
	let cli = args::Cli::parse();

	let mut json = cli.read_input()?;

	cli.junstr(&mut json);

	cli.print_output(&json);

	Ok(())
}

impl args::Cli {
	fn read_input(&self) -> Result<json::Value> {
		let input = if *self.input == *"-" {
			let mut input = Vec::with_capacity(512);
			io::stdin().read_to_end(&mut input)?;
			input
		} else {
			fs::read(&self.input)?
		};

		let json = json::from_slice(&input)?;

		Ok(json)
	}

	fn junstr(&self, data: &mut json::Value) {
		for at in &self.at {
			let Some(root) = data.pointer_mut(at) else { continue; };
			let mut stack = Vec::with_capacity(512);
			stack.push(root);

			while let Some(value) = stack.pop() {
				match value {
					json::Value::Null
						| json::Value::Bool(_)
						| json::Value::Number(_) => {}
					json::Value::Array(values) => stack.extend(values.iter_mut()),
					json::Value::Object(obj) => stack.extend(obj.values_mut()),
					json::Value::String(string) => {
						if let Ok(val) = json::from_str(string) {
							*value = val;
							stack.push(value);
						}
					},
				}
			}
		}
	}

	fn print_output(&self, json: &json::Value) {
		if self.compact {
			println!("{}", json);
		} else {
			println!("{:#}", json);
		}
	}
}


#[derive(derive_more::Debug, Display, Error, From)]
enum Error {
	#[debug("io error: {_0}")]
	#[display("io error: {_0}")]
	Io(io::Error),
	#[debug("json error: {_0}")]
	#[display("io error: {_0}")]
	Json(json::Error),
}
