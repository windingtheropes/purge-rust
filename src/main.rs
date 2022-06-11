use std::env;
use std::fs;
use std::fs::Metadata;
use std::io::{stdin, stdout, Write, Error};

#[derive(Debug)]

struct Options {
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
        let query: String = args[0].clone();
        let path: String = args[1].clone();
        let flags: String = args[2].clone();
        
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
            path: path,
            query: query,
        };
        
        // check flags
        let flags = flags.chars();
        for flag in flags {
            match flag {
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

        options
    }

    pub fn path(&self) -> &str {
        &self.path
    }
    pub fn query(&self) -> &str {
        &self.query
    }
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0); // remove executable from args

    if args.len() < 2 {
        println!("purge: missing argument");
        println!("usage: purge <query> <path> [flags]");
    }
    else {
        let options = Options::new(args);
        run(options);
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

fn rmf(path : &str, options: &Options)
{
    if options.no_ask == true
    {
        delete_file(path)
    }   
    else {
        let mut s: String = String::new();
        print!("Delete {}? (Y/N)", path);
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

fn run(options: Options) {
    println!("Searching for files with query `{}` in `{}`. ", options.query(), options.path());
    fn read_dir(dir: String, options: &Options) {
        let mut query: String = String::from(options.query().clone());
        let dir = fs::read_dir(dir).unwrap();
        for item in dir {
            if options.verbose == true {println!("Now reading directory {:?}", item.as_ref().unwrap().path());}
            // stuff here is wrapped in Ok and DirEntry
            let path = item.unwrap().path();

            // get the filename for filtering
            let file_name: &Vec<&str> = &path.to_str().unwrap().split("\\").collect::<Vec<&str>>();
            let mut file_name: String = String::from(file_name[file_name.len() - 1]);

            // object because it is either a file or directory
            let object: Metadata = fs::metadata(&path).unwrap(); // reference path it so it doesnt move to metadata and we can keep using it

            if object.is_dir()
            {      
                // println!("{:?}", path);
                read_dir(String::from(path.to_str().unwrap()), &options);
            }
            else if object.is_file()
            {
                // convert the filename and query to lowercase
                match &options.case_insensitive {
                    true => {
                        file_name = file_name.to_lowercase();
                        query = query.to_lowercase();
                    },
                    _ => {

                    }
                }

                // check for query at end
                match &options.ext {
                    true => {
                        let extension = file_name.split(".").collect::<Vec<&str>>()[1];
                        if extension == query
                        {   
                            rmf(&path.to_str().unwrap(), options)
                        }
                    },
                    _ => {

                    }
                }

                // check for query at end of name
                match &options.end {
                    true => {
                        let name = file_name.split(".").collect::<Vec<&str>>()[0];
                        if name.ends_with(&query) 
                        {   
                            rmf(&path.to_str().unwrap(), options)
                        }
                    },
                    _ => {

                    }
                }

                // check for query at start
                match &options.start {
                    true => {
                        if file_name.ends_with(&query) 
                        {   
                            rmf(&path.to_str().unwrap(), options)
                        }
                    },
                    _ => {

                    }
                }
            }
            else {
                panic!("Not file or directory?");
            }
        }
    }

    read_dir(String::from(options.path()), &options);
    println!("Done.")
}