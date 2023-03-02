use clue_core::{
	compiler::Compiler,
	env::Options,
	parser::parse_tokens,
	preprocessor::{preprocess_code, preprocess_codes},
	scanner::scan_code,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn version() -> String {
	String::from(env!("CARGO_PKG_VERSION"))
}

#[wasm_bindgen]
pub fn compile_code(mut code: String) -> Result<String, String> {
	let options = Options::default();
	let filename = String::from("(clue wasm)");
	let code = unsafe { code.as_bytes_mut() };
	let (codes, variables, ..) = preprocess_code(code, 1, false, &filename, &options)?;
	let code = preprocess_codes(0, codes, &variables, &filename)?;
	let tokens = scan_code(code, &filename)?;
	let (ctokens, statics) = parse_tokens(tokens, &filename, &options)?;

	let code = Compiler::new(&options).compile_tokens(0, ctokens);
	let code = code + &statics;

	Ok(code)
}
