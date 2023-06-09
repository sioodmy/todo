///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
use std::path::PathBuf;
use std::fs::{ self, OpenOptions };
use std::io::{ Read, Write };
use std::fmt::Display;
use std::result;
use serde::{ Deserialize, Serialize };
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
type Result<T> = result::Result<T, String>;
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Todo {
	pub list: List,
	pub path: PathBuf,
}

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Default)]
pub struct Task {
	name: String,
	description: Option<String>,
}

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Default)]
pub struct List {
	pub tasks: Vec<Task>,
	pub finished: Vec<Task>,
}
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
trait Message {
	type Inner;

	fn message(self, text: impl Display) -> Result<Self::Inner>;
}
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
pub fn help() {
	println!(
		"Usage: todo [COMMAND] [ARGUMENTS]\n\
		Todo is a super fast and simple tasks organizer written in rust\n\
		Available commands:\n\
		- add    [TASK...            ]: adds new task/s\n\
		- list   [                   ]: lists all tasks\n\
		- done   [INDEX/NAME...      ]: marks task as done\n\
		- reset  [                   ]: deletes all tasks\n\
		- restore[                   ]: restore recent backup\n\
		- sort   [                   ]: sorts by status\n\
		- raw    [todo/done          ]: prints selection as plain text"
	);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
impl Todo {
	pub fn new() -> Option<Todo> {
		let path = fs::read_dir(".")
			.ok()?
			.find(|item|
				item
					.as_ref()
					.map(|file|
						Some(
							file
								.file_type()
								.ok()?
								.is_file()
							&&
							file
								.file_name()
								.into_string()
								.ok()?
								.to_lowercase()
								.contains("todo")
						)
					)
					.unwrap_or_default()
					.unwrap_or_default()
			)
			.map(|file|
				file
					.unwrap() /* unwrap safe */
					.path()
			)?;
		Some(
			Todo {
				list: List::new(path.clone(), None).unwrap_or_default(),
				path
			}
		)
	}

	pub fn save(self) -> Result<()> {
		OpenOptions::new()
			.truncate(true)
			.create(true)
			.open(self.path)
			.message("FILE-ERROR")?
			.write_all(
				toml::to_string_pretty(&self.list)
					.message("TOML-PARSE-ERROR")?
					.as_bytes()
			)
			.message("SAVE-ERROR")
	}
}
impl Task {
	pub fn with_description(self, description: impl Display) -> Task {
		Task {
			name: self.name,
			description: Some(format!("{description}")),
		}
	}

	pub fn set_description(&mut self, new: impl Display) {
		let Some(ref mut description) = self.description else { return };
		*description = format!("{new}");
	}

	pub fn set_name(&mut self, new: impl Display) { self.name = format!("{new}") }
}

impl List {
	pub fn new(path: PathBuf, adapter: Option<fn(&mut OpenOptions) -> &mut OpenOptions>) -> Result<Self> {
		let mut file = adapter.unwrap_or(|options| options)(
			OpenOptions::new()
				.read(true)
		)
			.open(path)
			.message("FILE-ERROR")?;
		let buffer = &mut String::with_capacity(255);
		file
			.read_to_string(buffer)
			.message("READ-ERROR")?;
		toml::from_str(&buffer)
			.message("TOML-PARSE-ERROR")
	}

	pub fn finish_task(&mut self, identifier: &str) {
		let Some(position) = self
			.tasks
			.iter()
			.position(|Task { ref name, .. }| name == identifier) else { return };
		self
			.finished
			.push(
				self
					.tasks
					.remove(position) // probably safe
			);
	}

	pub fn add_task(&mut self, task: Task) {
		self
			.tasks
			.push(task)
	}

	pub fn clear_finished(&mut self) {
		self
			.finished
			.clear()
	}
}
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T, E> Message for result::Result<T, E> {
	type Inner = T;

	fn message(self, text: impl Display) -> Result<T> { self.map_err(|_| format!("{text}")) }
}
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
impl From<String> for Task {
	fn from(name: String) -> Task {
		Task { name, description: None }
	}
}
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
