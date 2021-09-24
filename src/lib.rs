use std::env;
use std::process;
use std::fs;

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
    pub todofile: String,
}

impl Todo {
    pub fn new (workdir: String) -> Result<Self,String> {
        let todofile = format!("{}/TODO",workdir);
        let todo = match fs::read_to_string(&todofile){
            Ok(t) => t,
            Err(e) => return Err(String::from("Couldnt read file"))
        };
        let todo = todo.lines().map(str::to_string).collect();

        Ok(Self{todo,todofile})
    }

    pub fn list (&self) {
        for task in self.todo.iter() {
            println!("{}",task);
        }
    }
    
    pub fn add (&self, element: String) {
        println!("jd"); 
    }
}
