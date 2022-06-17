mod libp;
use std::env;
use std::fs;
use std::fs::ReadDir;

use libp::handle_delete;
use libp::Options;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0); // remove executable from args

    if args.len() < 2 {
        println!("purge: missing argument");
        println!("usage: purge <query> <path> [flags]");

        if args.len() > 0 && args[0].to_lowercase() == "help" {
            println!("\nFlags:");
            println!("-e Search for query at end of filename");
            println!("-c Ignore capitalization");
            println!("-s Search for query at start of filename");
            println!("-x Search for file extension");
            println!("-y Don't ask before deleting found files");
            println!("-v Print more detailed progress information");
        } else {
            println!("\nrun `purge help` for a list of all flags");
        }
    } else {
        let options = Options::new(args);
        run(options);
    }
}

fn run(options: Options) {
    println!(
        "Searching for files with query `{}` in {}",
        options.query(),
        options.path()
    );
    fn read_dir(root: String, dir: ReadDir, options: &Options) {
        if options.verbose == true {
            println!("Now reading directory {}", root);
        }

        let mut query: String = String::from(options.query().clone());

        for item in dir {
            // Unwrap item
            let item = match item {
                Ok(r) => r,
                Err(_) => continue,
            };

            let path = item.path();

            // get the filename for filtering
            let path_to_file = path.as_path().to_str().unwrap();

            let path_to_file = if cfg!(windows) {
                let i = path_to_file.split("\\").collect::<Vec<&str>>();
                i
            } else {
                let i = path_to_file.split("/").collect::<Vec<&str>>();
                i
            };

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
                    continue;
                }
            };

            if object.is_dir() {
                // check if the directory is accessible, BEFORE executing the function, so we can handle errors before running
                let dir = fs::read_dir(&path);
                // make sure there are no errors before continuing
                let dir = match dir {
                    Ok(r) => r,
                    Err(_) => {
                        if options.verbose {
                            println!("Failed to read directory {}", path.to_str().unwrap());
                        }
                        continue;
                    }
                };

                read_dir(String::from(path.to_str().unwrap()), dir, &options);
            } else if object.is_file() {
                // convert the filename and query to lowercase
                match &options.case_insensitive {
                    true => {
                        file_name = file_name.to_lowercase();
                        query = query.to_lowercase();
                    }
                    _ => {}
                }

                let parts = file_name.split(".").collect::<Vec<&str>>();
                // if multi ext is on, all items after the first are ext, otherwise only the last one is
                let extension = if options.multi_ext {
                    let mut extension = parts.clone();
                    extension.remove(0);
                    extension
                } else {
                    let extension = vec![parts.last().unwrap().to_owned()];
                    extension
                };

                // configure file name
                let mut name = parts.clone();
                // if multi ext is on, only the first item is the name, if it's off, only the last item is the extension
                let name = if options.multi_ext {
                    let mut n = name.clone();
                    n[0].to_string()
                } else {
                    let mut n = name.clone();
                    n.remove(n.len() - 1);
                    let r = n.join(".");
                    r
                };

                let mut extension_parts = parts.clone();
                extension_parts.remove(0); // now this is just the parts after the first dot.

                // check at end of filename
                if options.end == true {
                    if name.ends_with(&query) {
                        if options.verbose == true {
                            println!("Found {}", file_name)
                        }
                        handle_delete(&String::from(&file_name), options)
                    }
                }

                // check at start of filename
                if options.start == true {
                    if name.starts_with(&query) {
                        if options.verbose == true {
                            println!("Found {}", file_name)
                        }
                        handle_delete(&String::from(&file_name), options)
                    }
                }

                // file extension
                if options.ext == true {
                    if query.starts_with(".") {
                        query.remove(0);
                    }
                    if options.multi_ext {
                        let extension = extension.join(".");
                        if extension == query {
                            if options.verbose == true {
                                println!("Found {}", file_name)
                            }
                            handle_delete(&String::from(&file_name), options)
                        }
                    } else {
                        let extension = extension.join(".");
                        if extension == query {
                            if options.verbose == true {
                                println!("Found {}", file_name)
                            }
                            handle_delete(&String::from(&file_name), options)
                        }
                    }
                }
            } else {
                panic!("Not file or directory?");
            }
        }
    }

    read_dir(
        String::from(options.path()),
        fs::read_dir(options.path()).unwrap(),
        &options,
    );
    println!("Done")
}
