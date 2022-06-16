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

fn CompileFile(path: &Path, name: String, scope: usize) -> Result<String, String> {
	let mut code: String = String::new();
	check!(check!(File::open(path)).read_to_string(&mut code));
	CompileCode(code, name, scope)
}

fn CompileFolder(path: &Path, rpath: String) -> Result<(), String> {
	for entry in check!(fs::read_dir(path)) {
		let entry = check!(entry);
		let name: String = entry
			.path()
			.file_name()
			.unwrap()
			.to_string_lossy()
			.into_owned();
		let filePathName: String = format!("{}/{}", path.display(), name);
		let filepath: &Path = Path::new(&filePathName);
		let rname = rpath.clone() + &name;
		if filepath.is_dir() {
			CompileFolder(filepath, rname + ".")?;
		} else if filePathName.ends_with(".clue") {
			let code = CompileFile(filepath, name, 2)?;
			let rname = rname.strip_suffix(".clue").unwrap();
			AddToOutput(&format!(
				"[\"{}\"] = function()\n{}\n\tend,\n\t",
				rname, code
			));
		}
	}
	Ok(())
}

fn main() -> Result<(), String> {
	unsafe {
		ENV_JITBIT = cli.jitbit;
		ENV_CONTINUE = cli.r#continue;
		ENV_RAWSETGLOBALS = cli.rawsetglobals;
		ENV_DEBUGCOMMENTS = cli.debugcomments;
	}
	if let Some(bit) = arg!(&ENV_JITBIT) {
		AddToOutput(&format!("local {} = require(\"bit\");\n", bit));
	}
	let codepath = cli.path.unwrap();
	let path: &Path = Path::new(&codepath);
	if path.is_dir() {
		AddToOutput(include_str!("base.lua"));
		CompileFolder(path, String::new())?;
		AddToOutput("\r}\nimport(\"main\")");
	} else if path.is_file() {
		let code = CompileFile(
			path,
			path.file_name().unwrap().to_string_lossy().into_owned(),
			0,
		)?;
		AddToOutput(&code);
	} else {
		return Err(String::from("The given path doesn't exist"));
	}
	Ok(())
}
