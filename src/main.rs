use std::env;
use todo_bin::*;
use Command::*;

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
	if let New = command {
		Todo::create(arguments.next())?;
		return Ok(())
	}
	let mut instance = Todo::new(env::var("TODO").ok())?;
	match command {
		Add => instance.add_task(
			Task::from(
				(
					arguments
						.next()
						.or_error("NO TASK PROVIDED ")?,
					arguments.next(),
					arguments.next(),
				)
			)
		),
		Finish => instance
			.finish_task(
				arguments
					.next()
					.unwrap_or(String::from(' '))
			),
		List => instance.query(arguments.next()),
		Clear => instance.clear_finished(),
		Raw => instance.all_raw(),
		Help => help(),
		_ => unimplemented!(),
	}
	instance.save()?;
	Ok(())
}
