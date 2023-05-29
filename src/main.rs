use std::env;
use todo_bin::{ help, Todo };
use itoa::Buffer;

fn main() {
	let mut args = env::args().skip(1);
	let todo = match Todo::new() {
		Ok(instance) => instance,
		Err(error) => { eprintln!("{error}"); return },
	};
	let mut buffer = Buffer::new();
	if let Some(command) = args.next() {
		let rest: Vec<String> = args.collect();
		if let Err(error) = match &*command {
			"list" => todo.list(&mut buffer),
			"add" => todo.add(&rest),
			"rm" => todo.remove(&rest, &mut buffer),
			"done" => todo.done(&rest, &mut buffer),
			"raw" => todo.raw(&rest),
			"sort" => todo.sort(),
			"reset" => todo.reset(),
			"restore" => todo.restore(),
			"help" | "--help" | "-h" | _ => Ok(help()),
		} { eprintln!("{error}") };
	} else { help(); return };
}
