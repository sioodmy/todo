use todo::Config;
use todo::Todo;
use std::env;
use std::process;

fn main() {


    let config = match Config::new() {
        Ok(c) => c,
        Err(e) => { 
            eprintln!("Couldn't load config");
            process::exit(1);
        }
    };
    let todo = match Todo::new(config.workdir) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Couldn't create TODO instance: \n {} ", e);
            process::exit(1);
        }
    };


    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        for arg in args {
            match &arg[..] {
                    "list" | "l" | "ls" => todo.list(),
                    _ => println!("zjebaes"),
            }
        }
    }
    
}

