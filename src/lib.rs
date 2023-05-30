use colored::*;
use itoa::Buffer;
use std::{ fs, env };
use std::io::{ Write, Read};
use std::path::PathBuf;


pub struct Todo {
	pub todo: Vec<String>,
	pub todo_path: PathBuf,
	pub todo_bak: Option<PathBuf>,
}


macro_rules! util {
	() => { concat!("Error@[", line!(), ',', column!(), "]: ") };
	($bytes: expr => $file: ident) => {
		let Ok(_) = $file.write_all($bytes) else { util!{ Unable to write data } };
	};
	($arguments: ident) => {
		if $arguments.is_empty() { util!{ "raw" takes in at least a single argument } }
	};
	($todo: expr; $($option: ident),+ $(,)?) => {
		{
			let Ok(file) = fs::OpenOptions::new()
				$(.$option(true))+
				.open(&$todo) else { util!{ Could not open a todo_file } };
			file
		}
	};
	($($addendum: tt)+) => { Err(concat!(util!(), stringify!($($addendum)+), '.'))? };
}


pub fn help() {
	println!(
		"Usage: todo [COMMAND] [ARGUMENTS]\n\
		Todo is a super fast and simple tasks organizer written in rust\n\
		Available commands:\n\
		- add    [TASK...            ]: adds new task/s\n\
		- list   [                   ]: lists all tasks\n\
		- done   [INDEX/NAME...      ]: marks task as done\n\
		- rm     [INDEX/NAME/done... ]: removes a task\n\
		- reset  [                   ]: deletes all tasks\n\
		- restore[                   ]: restore recent backup\n\
		- sort   [                   ]: sorts by status\n\
		- raw    [todo/done          ]: prints selection as plain text"
	);
}

fn split(task: &str) -> Option<(char, String)> {
	let mut vectorised = task.chars();
	Some((vectorised.next()?, vectorised.collect()))
}


impl Todo {
	fn get_iter(&self) -> impl Iterator<Item = &String> {
		self
			.todo
			.iter()
	}

	pub fn new() -> Result<Self, String> {
		let todo_path = match env::var("TODO_PATH") {
			Ok(path) => PathBuf::from(path),
			Err(_) => {
				let home = match env::var("HOME") {
					Ok(home) => home,
					Err(env::VarError::NotPresent	) => util!{ HOME environment variable was not found },
					Err(env::VarError::NotUnicode(_)) => util!{ HOME environment variabe contains some invalid unicode },
				};
				PathBuf::from(format!("{home}/.todo"))
			}
		};
		let todo_bak = match env::var("TODO_BAK_DIR") {
			Ok(path) => Some(
				{
					let path = PathBuf::from(path);
					if !path.exists() { PathBuf::from("/tmp/todo.bak") } else { path }
				}

			),
			Err(_) => None,
		};
		let mut todo_file = util!{ todo_path; write, read, create };
		let mut contents = String::new();
		let Ok(_) = todo_file.read_to_string(&mut contents) else { util!{ Reading into the String buffer failed } };
		let todo = contents.to_string().lines().map(str::to_string).collect();
		Ok(
			Self {
				todo,
				todo_path,
				todo_bak,
			}
		)
	}

	pub fn list(&self, buffer: &mut Buffer) -> Result<(), String> {
		let output = self
			.get_iter()
			.enumerate()
			.filter_map(|(mut order, task)|
				{
					order += 1;
					let (completed, mut rest) = split(task)?;
					if completed == '1' {
						rest = rest
							.strikethrough()
							.to_string()
					}
					Some(format!("{} {rest}", buffer.format(order).bold()))
				}
			)
			.collect::<Vec<String>>()
			.join("\n");
		println!("{output}");
		Ok(())
	}

	pub fn raw(&self, arguments: &[String]) -> Result<(), String> {
		util!{ arguments }
		let character = if arguments[0] == "done" { '1' } else { '0' };
		let output = self
			.get_iter()
			.filter_map(|task|
				{
					let (completed, rest) = split(task)?;
					if completed == character { return Some(rest) };
					None
				}
			)
			.collect::<Vec<String>>()
			.join("\n");
		println!("{output}");
		Ok(())
	}

	pub fn add(&self, arguments: &[String]) -> Result<(), String> {
		util!{ arguments }
		let mut todo_file = util!{ self.todo_path; create, append };
		let output = arguments
			.iter()
			.filter_map(|argument|
				{
					if argument
						.trim()
						.is_empty()
					{ None? };
					Some(format!("0{argument}\n"))
				}
			)
			.collect::<String>();
		util!{ output.as_bytes() => todo_file }
		Ok(())
	}

	pub fn remove(&self, arguments: &[String], buffer: &mut Buffer) -> Result<(), String> {
		util!{ arguments }
		let mut todo_file = util!{ self.todo_path; write, truncate };
		let output = self
			.get_iter()			
			.enumerate()
			.filter_map(|(mut index, task)|
				{
					index += 1;
					let (completed, rest) = split(task)?;
					if arguments
						.iter()
						.any(|argument| (argument == "done" && completed == '1') || argument == buffer.format(index) || argument == &rest)
					{ None? };
					Some(format!("{task}"))
				}
			)
			.collect::<Vec<String>>()
			.join("\n");
		util!{ output.as_bytes() => todo_file }
		Ok(())
	}

	fn remove_file(&self) -> Result<(), String> {
		let Ok(_) = fs::remove_file(&self.todo_path) else { util!{ Error whilst removing the todo_file } };
		Ok(())
	}

	pub fn reset(&self) -> Result<(), String> {
		if let Some(ref todo_bak) = self.todo_bak {
			let Ok(_) = fs::copy(&self.todo_path, todo_bak) else { util!{ Could not create a backup file } };			
		}
		self.remove_file()?;
		Ok(())
	}
	pub fn restore(&self) -> Result<(), String> {
		if let Some(ref todo_bak) = self.todo_bak {
			let Ok(_) = fs::copy(todo_bak, &self.todo_path) else { util!{ Could not restore the backup } };
		}
		Ok(())
	}

	pub fn sort(&self) -> Result<(), String> {
		let mut sorted_todo = self
			.todo
			.clone();
		sorted_todo.sort_unstable();
		let mut todo_file = util!{ self.todo_path; write, truncate };
		util!{
			sorted_todo
				.join("\n")
				.as_bytes() => todo_file
		}
		Ok(())
	}

	pub fn done(&self, arguments: &[String], buffer: &mut Buffer) -> Result<(), String> {
		util!{ arguments }
		let mut todo_file = util!{ self.todo_path; write };
		let mut position = String::with_capacity(50);
		let output = self
			.get_iter()
			.enumerate()
			.filter_map(|(mut index, task)|
				{
					index += 1;
					let (completed, rest) = split(task)?;
					position.replace_range(.., buffer.format(index));
					let completed = match (
						arguments
							.iter()
							.any(|argument| argument == &position || argument == &rest),
						completed,
					) {
						(true, '1') => '0',
						(true, '0') => '1',
						(_, other) => other,
					};
					Some(format!("{completed}{rest}"))
				}
			)
			.collect::<Vec<String>>()
			.join("\n");
		util!{ output.as_bytes() => todo_file }
		Ok(())
	}
}
