///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
use std::{
	fs::{ self, File, OpenOptions },
	fmt::{ self, Display },
	io::{ Read, Write },
	str::FromStr,
	ops::{ Deref, DerefMut },
	path::PathBuf,
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
	msg!{ SAVE = SAVING FAILED }
	msg!{ FIND = FILE NOT FOUND }
	// TODO: more descriptive errors.
}
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
pub type End<T> = Result<T, String>;
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
	group: String,
	purpose: Option<String>,
}

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Default)]
pub struct List {
	todo: Vec<Task>,
	done: Vec<Task>,
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
	New,
	#[default] Help,
}
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
pub trait Message {
	type Inner;

	fn or_error(self, text: impl Display) -> End<Self::Inner>;
}
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
pub fn help() {
	println!(
		"Usage: todo [COMMAND] [ARGUMENTS..]\n\n\
		Todo is a super fast and simple tasks organizer written in rust\n\n\
		Available commands:\n\
		- add     <TASK-NAME> <GROUP?> <PURPOSE?>  adds a task.\n\
		- finish  <TASK-NAME> <GROUP?>             marks a task as finished.\n\
		- list    <GROUP?>                         either list all or a specific board of tasks.\n\
		- clear                                    clears all the finished task.\n\
		- raw                                      list all with a raw formatting.\n\
		- new     <FILE-NAME?>                     create a todo file in the current directory.\n\
		- help                                     print out this help prompt.\n\n\
		NOTE:\n\
		the question mark inside the angle-brackets means that that argument is optional."
	);
}

fn ref_map<T, U, E, F>(resulting: &Result<T, E>, map: impl FnOnce(&T) -> Result<U, F>) -> Option<U> {
	resulting
		.as_ref()
		.ok()
		.map(|item| map(item).ok())
		.flatten()
}

fn is_query(group: &String, query: &Option<String>) -> bool {
	query
		.as_ref()
		.map_or(true, |query| query == group)
}
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
impl Todo {
	pub fn new(path: Option<String>) -> End<Todo> {
		let error = || -> End<PathBuf> {
			Ok(
				fs::read_dir(".")
					.or_error(errors::READ)?
					.filter(|item|
						ref_map(
							&item,
							|item| item.file_type(),
						)
							.map_or(false, |item| item.is_file())
					)
					.find(|file|
						ref_map(
							&file,
							|file|
								file
									.file_name()
									.into_string()
						)
							.map(|name| name.to_lowercase())
							.is_some_and(|name| name.contains("todo") && name.ends_with("toml"))
					)
					.or_error(errors::FIND)?
					.unwrap() /* unwrap safe */
					.path()
		
			)
		};
		let path = path.map_or_else(error, |text| Ok(PathBuf::from(text)))?;
		Ok(
			Todo {
				list: List::new(path.clone()).unwrap_or_default(),
				path
			}
		)
	}

	pub fn create(path: Option<String>) -> End<()> {
		OpenOptions::new()
			.write(true)
			.create_new(true)
			.open(path.unwrap_or(String::from("todo.toml")))
			.or_error(errors::OPEN)?;
		Ok(())
	}

	pub fn save(self) -> End<()> {
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
	pub fn new(path: PathBuf) -> End<Self> {
		let mut file = File::open(path).or_error(errors::OPEN)?;
		let buffer = &mut String::with_capacity(255);
		file
			.read_to_string(buffer)
			.or_error(errors::READ)?;
		toml::from_str(&buffer).or_error(errors::PARSE)
	}

	pub fn finish_task(&mut self, identifier: String, query: Option<String>) {
		let Some(position) = self
			.todo
			.iter()
			.position(|Task { name, group, .. }| *name == identifier && is_query(&group, &query)) else { return };
		self
			.done
			.push(
				self
					.todo
					.remove(position) // probably safe
			);
	}

	pub fn add_task(&mut self, task: Task) {
		self
			.todo
			.push(task)
	}

	pub fn clear_finished(&mut self) {
		self
			.done
			.clear()
	}

	pub fn query(&self, query: Option<String>) {
		let select = |pool: &Vec<Task>|
			pool
				.iter()
				.filter(|task| is_query(&task.group, &query))
				.for_each(|task| println!("{task}"));
		let Self { ref todo, ref done } = self;
		if !todo.is_empty() {
			println!("TODO:");
			select(todo)
		}
		if !todo.is_empty() && !done.is_empty() { println!() }
		if !done.is_empty() {
			println!("DONE:");
			select(done)
		}
	}

	pub fn all_raw(&self) {
		let print = |tasks: &Vec<Task>, finished: bool|
			for task in tasks.iter() {
				let state = if finished { "DONE" } else { "TODO" };
				let description = task
					.purpose
					.as_ref()
					.map(|task| format!(":{task}"))
					.unwrap_or_default();
				println!("{state}@{}/{}{description}", task.group, task.name,)
			};
		print(&self.todo, false);
		print(&self.done, true);
	}
}
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
impl Display for Task {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {

		write!(
			formatter,
			"[{:>13}] * {}",
			self.group,
			self.name,
		)?;
		let Some(ref description) = self.purpose else { return Ok(()) };
		write!(formatter, ": \"{}\"", description)
	}
}
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T, E> Message for Result<T, E> {
	type Inner = T;

	fn or_error(self, text: impl Display) -> End<T> { self.map_err(|_| format!("{text}")) }
}

impl<T> Message for Option<T> {
	type Inner = T;

	fn or_error(self, text: impl Display) -> End<T> { self.ok_or(format!("{text}")) }
}
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
impl From<(String, Option<String>, Option<String>)> for Task {
	fn from((name, group, purpose): (String, Option<String>, Option<String>)) -> Task {
		Task { name, group: group.unwrap_or(String::from("global")), purpose }
	}
}
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
impl FromStr for Command {
	type Err = String;

	fn from_str(text: &str) -> End<Self> {
		use Command::*;
		let text = text.to_lowercase();
		[Add, Finish, List, Clear, Raw, New, Help]
			.into_iter()
			.map(|variant| (format!("{variant:?}").to_lowercase(), variant))
			.find_map(|(reference, variant)|
				(1..=reference.len())
					.any(|upper| text == &reference[..upper])
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
