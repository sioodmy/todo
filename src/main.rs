use std::env;

fn main() {
	let arguments: Vec<String> = env::args()
		.skip(1)
		.collect(); 
	if arguments.is_empty() { /* enter repl */ return };
	// SPEC:
	//	Adding & Removing
	//	Finishing (maybe also unfinishing(via undo command?)) (the same as Removing?)
	//	serialized via toml
	//	Descriptions
	//	Listing (displaying to Stdout)
}
