///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
use std::{
	fs::{ self, OpenOptions },
	fmt::{ self, Display },
	io::{ Read, Write },
	str::FromStr,
	ops::{ Deref, DerefMut },
	path::PathBuf,
	result,
};
use serde::{ Deserialize, Serialize };
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
mod errors {
	macro_rules! msg {
		($name: ident = $($tokens: tt)+) => { pub(super) const $name: &str = concat!($(stringify!($tokens), ' '),+); }
	}
	msg!{ PARSE = PARSE ERROR }
	msg!{ READ = READING FAILED }
	msg!{ OPEN = OPENING FAILED }
	msg!{ SAVE = SAVING FAILD }
	// TODO: more descriptive errors.
}
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
pub type Result<T> = result::Result<T, String>;
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
	board: Option<String>,
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
#[derive(Debug)]
#[derive(Clone)]
#[derive(Default)]
pub enum Command { // map of public functions intended to be used as commands.
	Add,
	Finish,
	List,
	Clear,
	Raw,
	#[default] Help,
}
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
pub trait Message {
	type Inner;

	fn or_error(self, text: impl Display) -> Result<Self::Inner>;
}
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
pub fn help() {
	println!(
		"Usage: todo [COMMAND] [ARGUMENTS..]\n\n\
		Todo is a super fast and simple tasks organizer written in rust\n\n\
		Available commands:\n\
		- add     <TASK-NAME> <DESCRIPTION?>  adds a task.\n\
		- finish  <TASK-NAME>                 marks a task as finished.\n\
		- list    <BOARD?>                    either list all or a specific board of tasks.\n\
		- clear                               clears all the finished task.\n\
		- raw                                 list all with a raw formatting.\n\
		- help                                print out this help prompt.\n\n\
		NOTE:\n\
		the question mark inside the angle-brackets means that that argument is optional."
	);
}
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
impl Todo {
	pub fn new(path: Option<String>) -> Result<Todo> {
		let path = path
			.map_or_else(
				|| // needed due to ambiguous type case.
					Result::Ok(
						fs::read_dir(".")
							.or_error(errors::READ)?
							.filter(|item|
								item
									.as_ref()
									.map(|item|
										item
											.file_type()
											.map_or(false, |item| item.is_file())
									)
									.unwrap_or_default()
							)
							.find(|file|
								file
									.as_ref()
									.map(|file|
										{
											let Ok(name) = file
												.file_name()
												.into_string() else { return false };
											let name = name.to_lowercase();
											name.starts_with("todo") && name.ends_with("toml")
										},
									)
									.unwrap_or_default()
							)
							.or_error(errors::OPEN)?
							.unwrap() /* unwrap safe */
							.path()
					),
				|text| Ok(PathBuf::from(text))
			)?;
		Ok(
			Todo {
				list: List::new(path.clone()).unwrap_or_default(),
				path
			}
		)
	}

	pub fn save(self) -> Result<()> {
		OpenOptions::new()
			.write(true)
			.truncate(true)
			.open(self.path)
			.or_error(errors::OPEN)?
			.write_all(
				toml::to_string_pretty(&self.list)
					.or_error(errors::PARSE)?
					.as_bytes()
			)
			.or_error(errors::SAVE)
	}
}

impl List {
	pub fn new(path: PathBuf) -> Result<Self> {
		let mut file = OpenOptions::new()
			.read(true)
			.open(path)
			.or_error(errors::OPEN)?;
		let buffer = &mut String::with_capacity(255);
		file
			.read_to_string(buffer)
			.or_error(errors::READ)?;
		toml::from_str(&buffer)
			.or_error(errors::PARSE)
	}

	pub fn finish_task(&mut self, identifier: String) {
		let Some(position) = self
			.tasks
			.iter()
			.position(|Task { name, .. }| *name == identifier) else { return };
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

	pub fn query(&self, query: Option<String>) {
		let select = |pool: &Vec<Task>|
			pool
				.iter()
				.filter(|task|
					{
						let Some(ref query) = query else { return true };
						task
							.board
							.as_ref()
							.map(|board| board == query)
							.unwrap_or_default()
					}
				)
				.for_each(|task| println!("{task}"));
		println!("TODO:");
		select(&self.tasks);
		println!("\nFINISHED:");
		select(&self.finished)
	}

	pub fn all_raw(&self) {
		let board_default = String::from("all");
		let print = |tasks: &Vec<Task>, finished: bool| {
			tasks
				.iter()
				.for_each(|task|
					println!(
						"{}_{}/{}{}",
						if finished { String::from("DONE") } else { String::from("TODO") },
						task
							.board
							.as_ref()
							.unwrap_or(&board_default),
						task.name,
						if task
							.description
							.is_some()
						{
							format!(
								":{}",
								task
									.description
									.as_ref()
									.unwrap() /* unwrap safe */
							)
						} else { String::new() }
					)
				)
		};
		print(&self.tasks, false);
		print(&self.finished, true);
	}
}
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
impl Display for Task {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {

		write!(
			formatter,
			"[{:>13}] * {}",
			self
				.board
				.as_ref()
				.unwrap_or(&String::from("all")),
			self.name
		)?;
		let Some(ref description) = self.description else { return Ok(()) };
		write!(formatter, ": \"{}\"", description)
	}
}
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T, E> Message for result::Result<T, E> {
	type Inner = T;

	fn or_error(self, text: impl Display) -> Result<T> { self.map_err(|_| format!("{text}")) }
}

impl<T> Message for Option<T> {
	type Inner = T;

	fn or_error(self, text: impl Display) -> Result<T> { self.ok_or(format!("{text}")) }
}
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
impl From<(String, Option<String>, Option<String>)> for Task {
	fn from((name, description, board): (String, Option<String>, Option<String>)) -> Task {
		Task { name, description, board }
	}
}
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
impl FromStr for Command {
	type Err = String;

	fn from_str(text: &str) -> Result<Self> {
		use Command::*;
		let text = text.to_lowercase();
		[("add", Add), ("finish", Finish), ("list", List), ("clear", Clear), ("raw", Raw), ("help", Help)]
			.into_iter()
			.find_map(|(command, variant)|
				(1..=command.len())
					.any(|upper| text.starts_with(&command[..upper]))
					.then_some(variant)
			)
			.or_error(errors::PARSE)
	}
}
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
impl Deref for Todo {
	type Target = List;

	fn deref(&self) -> &List { &self.list }
}
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
impl DerefMut for Todo {
	fn deref_mut(&mut self) -> &mut List { &mut self.list }
}
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
