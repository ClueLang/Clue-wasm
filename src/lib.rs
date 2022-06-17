#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

macro_rules! check {
	($tocheck: expr) => {
		match $tocheck {
			Ok(t) => t,
			Err(e) => return e,
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
use wasm_bindgen::prelude::*;

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

#[wasm_bindgen]
pub fn CompileCode(code: String, name: String, scope: usize) -> String {
	let tokens: Vec<Token> = check!(ScanCode(code, name.clone()));
	let ctokens = check!(ParseTokens(tokens, name.clone()));
	CompileTokens(scope, ctokens)
}
