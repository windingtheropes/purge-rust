mod libp;
use std::env;
use std::fs::ReadDir;
use std::fs;

use libp::Options;
use libp::handle_delete;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0); // remove executable from args

    if args.len() < 2 {
        println!("purge: missing argument");
        println!("usage: purge <query> <path> [flags]");

        if(args.len() > 0 && args[0].to_lowercase() == "help")
        {
            println!("\nFlags:");
            println!("-e Search for query at end of filename");
            println!("-c Ignore capitalization");
            println!("-s Search for query at start of filename");
            println!("-x Search for file extension (ignores `.`)");
            println!("-y Don't ask before deleting found files");
            println!("-l Enable logging to purge.log.txt in the running directory");
            println!("-L Enable verbose logging");
            println!("-v Print more detailed progress information");
            println!("-o Overwrite logs, rather appending logs to the same file");
        }
        else { println!("\nrun `purge help` for a list of all flags"); }
    }
    else {
        let options = Options::new(args);
        run(options);
    }
}

fn run(options: Options) {
    println!("Searching for files with query `{}` in {}", options.query(), options.path());
    fn read_dir(root: String, dir: ReadDir, options: &Options) {
        if options.verbose == true { println!("Now reading directory {}", root); }

        let mut query: String = String::from(options.query().clone());

        for item in dir {

            // Unwrap item
            let item = match item {
                Ok(r) => r,
                Err(_) => continue,
            };

            let path = item.path();

            // get the filename for filtering
            let path_to_file = path.as_path().to_str().unwrap().split("/").collect::<Vec<&str>>();
            let mut file_name: String = String::from(path_to_file[path_to_file.len() - 1]); // get the last item in the array

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

                read_dir(String::from(path.to_str().unwrap()), dir, &options);
            }
            else if object.is_file()
            {
                // convert the filename and query to lowercase
                match &options.case_insensitive {
                    true => {
                        file_name = file_name.to_lowercase();
                        query = query.to_lowercase();
                    },
                    _ => {},
                }
                
            }
            else {
                panic!("Not file or directory?");
            }
        }
    }

    read_dir(String::from(options.path()), fs::read_dir(options.path()).unwrap(), &options);
    println!("Search complete. ")
}