use std::env;
use todo_bin::*;
// use Command::*;

fn main() -> Result<()> {
	let mut arguments = env::args()
		.skip(1)
		.peekable(); 
	if arguments
		.peek()
		.is_none() { /* enter repl? */ help(); return Ok(()) };
	let command = arguments
		.next()
		.unwrap() /* unwrap safe */
		.parse::<Command>()?;
	match command {
		_ => { },
	}
	// SPEC:
	//	Adding & Removing
	//	Finishing (maybe also unfinishing(via undo command?)) (the same as Removing?)
	//	serialized via toml
	//	Descriptions
	//	Listing (displaying to Stdout)
	Ok(())
}
