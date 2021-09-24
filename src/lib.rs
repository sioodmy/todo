use std::env;
use std::process;
use std::fs;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::fs::OpenOptions;

pub struct Config {
    pub workdir: String,
    pub editor: String,
}

impl Config {
    pub fn new () -> Result<Self,String> {
        //Reads EDITOR env variable 
        let editor = match env::var("EDITOR") {
            Ok(e) => e,
            Err(e) => String::from("vim")
        };
        //Reads working directory variable
        let workdir = match env::var("PWD") {
            Ok(w) => w,
            Err(e) => return Err(String::from("Couldn't get working directory")) 
        };

        Ok(Self { editor, workdir})

    }
}

pub struct Todo {
    pub todo: Vec<String>,
    pub todofile: fs::File,
}

impl Todo {
    pub fn new (workdir: String) -> Result<Self,String> {

        let filepath = format!("{}/TODO",workdir); 
//        let todofile = File::open(filepath).unwrap();
        let mut todofile = match OpenOptions::new()
            .write(true)
            .read(true)
            .append(true)
            .open("TODO") {
                Ok(t) => t,
                Err(e) => return Err(String::from("kurwa"))
            };
       let mut buf_reader = BufReader::new(&todofile);
       let mut contents = String::new();
       buf_reader.read_to_string(&mut contents).unwrap();
        let todo = contents.lines().map(str::to_string).collect();

        Ok(Self{todo,todofile})
    }

    pub fn list (&self) {
        for task in self.todo.iter() {
            println!("{}",task);
        }
    }
    
    pub fn add (&self, element: String) {
        println!("jd");
        writeln!(&self.todofile, "jd orka");
    }
}
