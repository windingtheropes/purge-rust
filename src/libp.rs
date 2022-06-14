// libp, lib purge version 1.0
// provides the Options type to run purge. added to a separate library for organisation

use std::fs;
use std::io::{stdin, stdout, Write, Error};
use std::fs::Metadata;
pub struct Options {
    // arguments
    path: String,
    query: String,

    // flags
    pub start: bool,
    pub end: bool,
    pub ext: bool,
    pub case_insensitive: bool,
    pub no_ask: bool,

    // options
    pub verbose: bool,

    // logging
    pub logging: bool,
    pub overwrite_logs: bool,
    pub verbose_logging: bool,
}

impl Options {
    pub fn new(args: Vec<String>) -> Options {
        // for i in &args {println!("{}", i)}
        let query: String = args[0].clone();
        let path: String = args[1].clone();
        
        let mut flags = args.clone();
        // this removes query and path, so flags can be separated in to as many arguments as they want
        flags.remove(0); // since this modifies the array automatically, this is index 0 of args
        flags.remove(0); // where this is index 1 of args because we removed 0 already but it's a new array so it is now first

        // check if path exists and is a directory
        let dir: Metadata = fs::metadata(&path).unwrap_or_else(|error| {
            if error.kind() == std::io::ErrorKind::NotFound {
                println!("Path not found.");
                std::process::exit(1);
            } else {
                panic!("{}", error);
            }
        });
        if !dir.is_dir() {
            println!("{} is not a directory.", path);
            std::process::exit(1);
        }

        // Define the default options
        let mut options = Options {
            start: false,
            end: false,
            ext: false,
            case_insensitive: false,
            no_ask: false,
            verbose: false,
            logging: false,
            overwrite_logs: false,
            verbose_logging: false,
            path,
            query,
        };

        // Run through flags array and get each set first
        for mut flag in flags {
            // Remove dashes
            flag = flag.replace("-", "");
            
            // Then read the set char by char
            let chars = flag.chars();
            for char in chars {
                match char {
                    's' => options.start = true,
                    'e' => options.end = true,
                    'x' => options.ext = true,
                    'c' => options.case_insensitive = true,
                    'y' => options.no_ask = true,
                    'v' => options.verbose = true,
                    'l' => options.logging = true,
                    'L' => options.verbose_logging = true,
                    'o' => options.overwrite_logs = true,
                    _ => {
                        println!("Unknown flag: {}", flag);
                        std::process::exit(1);
                    }
                }
            }
        }

        // Return the options
        options
    }

    // These values have been validated as they were in the beginning, so they should be kept this way as changing them could cause errors
    pub fn path(&self) -> &str {
        &self.path
    }
    pub fn query(&self) -> &str {
        &self.query
    }
}

fn delete_file(path: &str)
{
    let result: Result<(), Error> = fs::remove_file(path);
    match result {
        Ok(_) => println!("Deleted {}", path),
        Err(_) => println!("Error deleting {}", path),

    }
}

pub fn handle_delete(path : &str, options: &Options)
{
    if options.no_ask == true
    {
        delete_file(path)
    }   
    else {
        let mut s: String = String::new();
        print!("Delete {}? (Y/N) ", path);
        let _ = stdout().flush();
        stdin().read_line(&mut s).expect("needed a value");
        let s = s.as_str().trim().to_lowercase();
        let s = s.as_str();
        match s {
            "y" => {
                delete_file(path)
            },
            "n" => return println!("Not deleting {}", path),
            _ => return println!("Not deleting {}", path),
        };
    }
}