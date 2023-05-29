use std::env;
use todo_bin::{ help, Todo };

fn main() {
	let mut args = env::args().skip(1);
	let todo = match Todo::new() {
		Ok(instance) => instance,
		Err(error) => { eprintln!("{error}"); return },
	};

	if let Some(command) = args.next() {
		let rest: Vec<String> = args.collect();
		if let Err(error) = match &*command {
			"list" => Ok(todo.list()),
			"add" => todo.add(&rest),
			"rm" => todo.remove(&rest),
			"done" => todo.done(&rest),
			"raw" => todo.raw(&rest),
			"sort" => todo.sort(),
			"reset" => todo.reset(),
			"restore" => todo.restore(),
			"help" | "--help" | "-h" | _ => Ok(help()),
		} { eprintln!("{error}") };
	} else {
		help();
		println!();
		todo.list();
		return
	};
}
