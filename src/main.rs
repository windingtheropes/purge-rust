mod libp;
use std::env;
use std::io::{stdin, stdout, Write, Error};
use std::fs::{Metadata, ReadDir};
use std::fs;

use libp::Options;

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

fn handle_delete(path : &str, options: &Options)
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
    fn read_dir(path: String, options: &Options, dir: ReadDir) {
        let mut query: String = String::from(options.query().clone());
        let d = fs::read_dir(&path).unwrap();

        for item in dir {
            if options.verbose == true {println!("Now reading directory {:?}", item.as_ref().unwrap().path());}
            // stuff here is wrapped in Ok and DirEntry
            let path = item.unwrap().path();

            // get the filename for filtering
            let file_name: &Vec<&str> = &path.to_str().unwrap().split("\\").collect::<Vec<&str>>();
            let mut file_name: String = String::from(file_name[file_name.len() - 1]);

            // object because it is either a file or directory
            let object = fs::metadata(&path); // reference path it so it doesnt move to metadata and we can keep using it
            // make sure there are no errors before continuing
            let object = match object {
                Ok(r) => r,
                Err(_) => {
                    if options.verbose {
                        println!("Failed to read directory {}", path.to_str().unwrap());
                    }
                    continue
                }
            };

            if object.is_dir()
            {      
                // check if the directory is accessible, BEFORE executing the function, so we can handle errors before running
                let dir = fs::read_dir(&path);
                // make sure there are no errors before continuing
                let dir = match dir {
                    Ok(r) => r,
                    Err(_) => {
                        if options.verbose {
                            println!("Failed to read directory {}", path.to_str().unwrap());
                        }
                        continue
                    },
                };

                read_dir(String::from(path.to_str().unwrap()), &options, dir);
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
                            handle_delete(&path.to_str().unwrap(), options)
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
                            handle_delete(&path.to_str().unwrap(), options)
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
                            handle_delete(&path.to_str().unwrap(), options)
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

    read_dir(String::from(options.path()), &options, fs::read_dir(options.path()).unwrap());
    println!("Done.")
}