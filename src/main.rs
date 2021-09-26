use todo::*;
use std::env;
use std::process;

fn main() {

    let todo = match Todo::new() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Couldn't create TODO instance: {:?} ", e);
            process::exit(1);
        }
    };


    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let command = &args[1];   
        match &command[..] {
                    "list" => todo.list(),
                    "add" => todo.add(&args[2]),
                    "rm" => todo.remove(&args[2]),
                    "done" => todo.done(&args[2..]),
                    _ => ()
            }
    }  
    
}

