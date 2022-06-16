#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

macro_rules! check {
	($tocheck: expr) => {
		match $tocheck {
			Ok(t) => t,
			Err(e) => return Err(e.to_string()),
		}
	};
}

macro_rules! arg {
	($name: expr) => {
		unsafe { $name }
	};
}

mod compiler;
mod parser;
mod scanner;
use compiler::*;
use parser::*;
use scanner::*;
use std::{fs, fs::File, io::prelude::*, path::Path, time::Instant};

pub static mut finaloutput: String = String::new();

pub static mut ENV_JITBIT: Option<String> = None;
pub static mut ENV_CONTINUE: bool = false;
pub static mut ENV_RAWSETGLOBALS: bool = false;
pub static mut ENV_DEBUGCOMMENTS: bool = false;


	// /// Use LuaJIT's bit library for bitwise operations
	// #[clap(short, long, value_name = "VAR NAME")]
	// jitbit: Option<String>,

	// /// Use tags and goto for continue
	// #[clap(short, long)]
	// r#continue: bool,

	// /// Use rawset to create globals
	// #[clap(short, long)]
	// rawsetglobals: bool,

	// /// Include debug comments in the output
	// #[clap(short, long)]
	// debugcomments: bool,

fn AddToOutput(string: &str) {
	unsafe { finaloutput += string }
}


fn CompileCode(code: String, name: String, scope: usize) -> Result<String, String> {
	let time = Instant::now();
	let tokens: Vec<Token> = ScanCode(code, name.clone())?;
	let ctokens = ParseTokens(tokens, name.clone())?;
	let code = CompileTokens(scope, ctokens);
	println!(
		"Compiled file \"{}\" in {} seconds!",
		name,
		time.elapsed().as_secs_f32()
	);
	Ok(code)
}
