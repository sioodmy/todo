use std::io::{BufReader, Write};
use std::io::prelude::*;
use std::fs::OpenOptions;
use colored::*;

pub struct Todo {
    pub todo: Vec<String>,
}

impl Todo {
    pub fn new () -> Result<Self,String> {

        let todofile = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open("TODO")
            .expect("Couldn't open the todofile");

        // Creates a new buf reader
        let mut buf_reader = BufReader::new(&todofile);

        // Empty String ready to be filled with TODOs
        let mut contents = String::new();

        // Loads "contents" string with data
        buf_reader.read_to_string(&mut contents).unwrap();

        // Splits contents of TODO file into a todo vector
        let todo = contents.to_string().lines().map(str::to_string).collect();
        
        // Returns todo
        Ok(Self{todo})
    }


    // Prints every todo
    pub fn list (&self) {
        let mut i = 1;
        
        // This loop will repeat itself for each taks in TODO file
        for task in self.todo.iter() {
           
            if task.len() > 5 {
            // Converts virgin default number into a chad BOLD string
            let number = i.to_string().bold();

            // Saves the symbol of current task
            let symbol = &task[..4];
            // Saves a task without a symbol
            let task = &task[4..];

            // Checks if the current task is completed or not...
            if symbol == "[*] " {
                // DONE
                
                //If the task is completed, then it prints it with a strikethrough 
                println!("{} {}",number, task.strikethrough()); 
            } else if symbol == "[ ] " {
                // NOT DONE

                //If the task is not completed yet, then it will print it as it is
                println!("{} {}",number , task);
            }

                // Increases the i variable by 1
            i = i+1;
            } 
        }
    }
   
    // Adds a new todo
    pub fn add (&self, args: &[String]) {
        
        // Opens the TODO file with a permission to:
        let mut todofile = OpenOptions::new()
            .create(true) // a) create the file if it does not exist 
            .append(true) // b) append a line to it
            .open("TODO")
            .expect("Couldn't open the todofile");

        let mut newtodo = String::new();
        
        for arg in args {
            let line = format!("[ ] {}\n", arg);
            newtodo.push_str(&line);
        }
        
        // Appends a new task/s to the file
        writeln!(todofile,"{}", newtodo).unwrap();
    }

    // Removes a task
    pub fn remove (&self, args: &[String]) {
        

        // Creates a new empty string
        let mut newtodo = String::new();

   
        
        for (pos, line) in self.todo.iter().enumerate() {
            if args.contains(&(pos+1).to_string()) {
                continue;
            }
           
            let line = format!("{}\n", line);
            newtodo.push_str(&line[..]);
        }
        
        // Opens the TODO file with a permission to:
        let mut todofile = OpenOptions::new()
            .write(true) // a) write
            .truncate(true) // b) truncrate
            .open("TODO")
            .expect("Couldn't open the todo file");
        
        // Writes contents of a newtodo variable into the TODO file 
        todofile.write_all(newtodo.as_bytes())
            .expect("Error while trying to save the todofile");
//        write!(&self.todofile, "{}", newtodo).unwrap();
    }


    pub fn done (&self, args: &[String]) {
        

        // Creates a new empty string
        let mut newtodo = String::new();

   
        for (pos, line) in self.todo.iter().enumerate() {
            if line.len() > 5 {
                    if  args.contains(&(pos+1).to_string()){

                    if &line[..4] == "[ ] "{
                        let line = format!("[*] {}\n", &line[4..]);
                        newtodo.push_str(&line[..]);
                    } else if &line[..4] == "[*] " {
                        let line = format!("[ ] {}\n", &line[4..]);
                        newtodo.push_str(&line[..]);
                    }
        
                } else {
                    if &line[..4] == "[ ] " || &line[..4] == "[*] " {
                        let line = format!("{}\n", line);
                        newtodo.push_str(&line[..]);
                    }
                } 
            }
        }
        
        // Opens the TODO file with a permission to overwrite it
        let mut f = OpenOptions::new()
            .write(true) 
            .open("TODO")
            .expect("Couldn't open the todofile");
        
        // Writes contents of a newtodo variable into the TODO file 
        f.write_all(newtodo.as_bytes()).expect("Error while trying to save the todofile");
//        write!(&self.todofile, "{}", newtodo).unwrap();
    }

}


pub fn help() {
    println!(
"Usage: todo [COMMAND] [ARGUMENTS]
Todo is a super fast and simple tasks organizer written in rust
Example: todo list
Available commands:
    - new [TASK/s] 
        adds new task/s
        Example: todo add \"buy carrots\"
    - list
        lists all tasks
        Example: todo list
    - done [INDEX]
        marks task as done
        Example: todo done 2 3 (marks second and third tasks as completed)
    - rm [INDEX] 
        removes a task
        Example: todo rm 4 
");
}
