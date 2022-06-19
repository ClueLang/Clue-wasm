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
use wasm_bindgen::prelude::*;

pub static mut finaloutput: String = String::new();

pub static mut ENV_JITBIT: Option<String> = None;
pub static mut ENV_CONTINUE: bool = false;
pub static mut ENV_RAWSETGLOBALS: bool = false;
pub static mut ENV_DEBUGCOMMENTS: bool = false;

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_namespace = console)]
	fn log(s: &str);
}
#[wasm_bindgen]
pub fn CompileCode(code: String, name: String, scope: usize) -> Result<String, String> {
	log(&format!("{:?}", code));
	let tokens: Vec<Token> = ScanCode(code, name.clone())?;
	let ctokens = ParseTokens(tokens, name.clone())?;
	let code = CompileTokens(scope, ctokens);
	Ok(code)
}
