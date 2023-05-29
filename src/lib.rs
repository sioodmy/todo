use colored::*;
use std::{
	fs::{ self, OpenOptions },
	io::{ BufReader, BufWriter, Write, Read},
	path::PathBuf,
	env::{ self, VarError as Var },
};


pub struct Todo {
	pub todo: Vec<String>,
	pub todo_path: PathBuf,
	pub todo_bak: PathBuf,
	pub no_backup: bool,
}


macro_rules! err {
	() => { concat!("Error@[", line!(), ',', column!(), "]: ") };
	($bytes: expr => $file: ident) => {
		let Ok(_) = $file.write_all($bytes) else { err!{ Unable to write data } };
	};
	($arguments: ident) => {
		if $arguments.is_empty() { err!{ "raw" takes in at least a single argument } }
	};
	($($addendum: tt)+) => { Err(concat!(err!(), stringify!($($addendum)+), '.'))? };
}


fn split(task: &str) -> Option<(char, String)> {
	let mut vectorised = task.chars();
	Some((vectorised.next()?, vectorised.collect()))
}

pub fn help() {
	println!(
r#"Usage: todo [COMMAND] [ARGUMENTS]
Todo is a super fast and simple tasks organizer written in rust
Available commands:
- add    [TASK...  ]: adds new task/s
- list   [         ]: lists all tasks
- done   [INDEX... ]: marks task as done
- rm     [INDEX... ]: removes a task
- reset  [         ]: deletes all tasks
- restore[         ]: restore recent backup
- sort   [         ]: sorts by status
- raw    [todo/done]: prints selection as plain text"#
	);
}


impl Todo {
	fn get_iter(&self) -> impl Iterator<Item = &String> {
		self
			.todo
			.iter()
	}

	pub fn new() -> Result<Self, String> {
		let todo_path = {
			let path = match env::var("TODO_PATH") {
				Ok(path) => PathBuf::from(path),
				Err(_) => {
					let home = match env::var("HOME") {
						Ok(home) => home,
						Err(Var::NotPresent	) => err!{ HOME environment variable was not found },
						Err(Var::NotUnicode(_)	) => err!{ HOME environment variabe contains some invalid unicode },
					};
					PathBuf::from(format!("{home}/.todo"))
				}
			};
			path
		};

		let todo_bak = PathBuf::from(
			match env::var("TODO_BAK_DIR") {
				Ok(t) => t,
				Err(_) => String::from("/tmp/todo.bak"),
			}
		);

		let no_backup = env::var("TODO_NOBACKUP").is_ok();

		let Ok(todofile) = OpenOptions::new()
			.write(true)
			.read(true)
			.create(true)
			.open(&todo_path) else { err!{ Could not open a todo_file } };

		// Creates a new buf reader
		let mut buf_reader = BufReader::new(&todofile);

		// Empty String ready to be filled with TODOs
		let mut contents = String::new();

		// Loads "contents" string with data
		let Ok(_) = buf_reader.read_to_string(&mut contents) else { err!{ Reading into the String buffer failed } };

		// Splits contents of the TODO file into a todo vector
		let todo = contents.to_string().lines().map(str::to_string).collect();

		// Returns todo
		Ok(
			Self {
				todo,
				todo_path,
				todo_bak,
				no_backup,
			}
		)
	}

	// Prints every todo saved
	pub fn list(&self) {
		self
			.get_iter()
			.enumerate()
			.filter_map(|(mut order, task)|
				{
					order += 1;
					if task.len() == 0 { None? };
					let rest = {
						let (completed, mut rest) = split(task)?;
						if completed == '1' {
							rest = rest
								.strikethrough()
								.to_string()
						}
						rest
					};
					Some((format!("{order}").bold(), rest))
				}
			)
			.for_each(|(order, text)| println!("{order} {text}"));
	}

	// This one is for yall, dmenu chads <3
	pub fn raw(&self, arg: &[String]) -> Result<(), String> {
		err!{ arg }
		// This loop will repeat itself for each task in TODO file
		self
			.get_iter()
			.filter_map(|task|
				{
					let (completed, rest) = split(task)?;
					Some(
						format!(
							"[{}] {rest}",
							match (completed, &*arg[0]) {
								('1', "done") => '*',
								('0', "todo") => ' ',
								_ => None?,
							}
						)
					)
				}
			)
			.for_each(|text| println!("{text}"));
		Ok(())
	}

	// Adds a new todo
	pub fn add(&self, args: &[String]) -> Result<(), String> {
		err!{ args }
		// Opens the TODO file with a permission to:
		let Ok(todofile) = OpenOptions::new()
			.create(true) // a) create the file if it does not exist
			.append(true) // b) append a line to it
			.open(&self.todo_path) else { err!{ Could not open a todo_file } };

		let mut buffer = BufWriter::new(todofile);

		for arg in args {
			if arg.trim().is_empty() { continue };
			err!{ format!("0{arg}\n").as_bytes() => buffer }
		}
		Ok(())
	}

	// Removes a task
	pub fn remove(&self, args: &[String]) -> Result<(), String> {
		err!{ args }
		// Opens the TODO file with a permission to:
		let Ok(todofile) = OpenOptions::new()
			.write(true) 
			.truncate(true)
			.open(&self.todo_path) else { err!{ Could not open the todo file } };

		let mut buffer = BufWriter::new(todofile);

		for task in self
			.get_iter()			
			.enumerate()
			.filter_map(|(mut index, task)|
				{
					index += 1;
					let (completed, _) = split(task)?;
					if args
						.iter()
						.any(|text| (text == "done" && completed == '1') || text == &format!("{index}"))
					{ None? };
					Some(format!("{task}\n"))
				}
			)
		{ err!{ task.as_bytes() => buffer } }
		Ok(())
	}

	fn remove_file(&self) -> Result<(), String> {
		let Ok(_) = fs::remove_file(&self.todo_path) else { err!{ Error whilst removing the todo_file } };
		Ok(())
	}

	// Clear todo by removing todo file
	pub fn reset(&self) -> Result<(), String> {
		if !self.no_backup { let Ok(_) = fs::copy(&self.todo_path, &self.todo_bak) else { err!{ Could not create a backup file } }; };
		self.remove_file()?;
		Ok(())
	}
	pub fn restore(&self) -> Result<(), String> {
		let Ok(_) = fs::copy(&self.todo_bak, &self.todo_path) else { err!{ Could not restore the backup } };
		Ok(())
	}

	// Sorts done tasks
	pub fn sort(&self) -> Result<(), String> {
		// Creates a new empty string
		let partition = self
			.todo
			.len() / 2;

		let (todo, done) = self
			.get_iter()
			.filter_map(|task| split(task))
			.fold((Vec::with_capacity(partition), Vec::with_capacity(partition)), |(mut todo, mut done), (completed, rest)|
				{
					match completed {
						'0' => todo.push(format!("{completed}{rest}")),
						'1' => done.push(format!("{completed}{rest}")),
						_ => (),
					}
					(todo, done)
				}
			);
		let newtodo = vec![todo, done]
			.into_iter()
			.flatten()
			.collect::<Vec<String>>()
			.join("\n");

		let Ok(mut todofile) = OpenOptions::new()
			.write(true) // a) write
			.truncate(true) // b) truncrate
			.open(&self.todo_path) else { err!{ Could not open the todo file } };

		// Writes contents of a newtodo variable into the TODO file
		err!{ newtodo.as_bytes() => todofile }
		Ok(())
	}

	pub fn done(&self, args: &[String]) -> Result<(), String> {
		err!{ args }

		// Opens the TODO file with a permission to overwrite it
		let Ok(mut todofile) = OpenOptions::new()
			.write(true)
			.open(&self.todo_path) else { err!{ Could not open the todofile } };

		for (completed, task) in self
			.get_iter()
			.enumerate()
			.filter_map(|(mut index, task)|
				{
					index += 1;
					let (completed, rest) = split(task)?;
					Some(
						(
							match (args.contains(&format!("{index}")), completed) {
								(true, '1') => '0',
								(true, '0') => '1',
								(_, other) => other,
							},
							rest
						)
					)
				}
			)
		{ err!{ format!("{completed}{task}\n").as_bytes() => todofile } }
		Ok(())
	}
}
