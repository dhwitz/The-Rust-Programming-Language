use std::io;
use std::collections::HashMap;
use std::io::Write;

fn main() {
    println!("Welcome to the employee database for Horowitz Enterprises!", );

    let mut database: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        print!("> ");
        io::stdout().flush().expect("flush");
        let mut response = String::new();

        io::stdin().read_line(&mut response)
            .expect("Line read error");
        
        let args = response.split_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        if args.len() == 0 {
            continue;
        }
        match args {
            ref x if x[0].to_lowercase() == "add" => {
                if x.len() != 5 {
                    println!("Invalid use of 'Add'. \nUsage: Add [First Name] [Last Name] \
                        to [Department]");
                    continue;
                }
                database.entry(x[4].to_string())
                    .or_insert(Vec::new()).push(format!("{}, {}", x[2], x[1]));
                
                if let Some(list) = database.get_mut(&x[4]) {
                    list.sort();
                }

                println!("{} {} was added to {}", x[1], x[2], x[4]);
            }
            ref x if x[0].to_lowercase() == "list" => {
                if x.len() == 1 {
                    println!("Invalid use of 'List'. \nUsage: List [department] or List all");
                } else if x[1] == "all" {
                    let mut names = Vec::new();
                    for key in database.keys() {
                        if let Some(list) = database.get(key) {
                            for item in list {
                                names.push(item);
                            }
                        }
                    }
                    names.sort();
                    for item in names {
                        println!("{}", item);
                    }
                } else {
                    let key = x[1].to_string();
                    if let Some(list) = database.get(&key) {
                        println!("Printing all names from the {} department.", &key);
                        for item in list {
                            println!("{}",item);
                        }
                    } else {
                        println!("{} is not a registered department", key);
                    }
                }
            }
            ref x if x[0].to_lowercase() == "exit" => break,
            ref x if x[0].to_lowercase() == "remove" => {
                if x.len() != 5 {
                    println!("Invalid use of 'Remove'. \nUsage: Remove [First Name] [Last Name] \
                    from [department]");
                    continue;
                }
                let name = format!("{}, {}", x[2], x[1]);
                if let Some(y) = database.get_mut(&x[4]) {
                    if let Some(z) = y.iter().position(|a| a[..] == name[..]) {
                        y.remove(z);
                        println!("{} {} was removed from {}", x[1], x[2], x[4])
                    } else {
                        println!("The name {} {} was not found in the {} department.", 
                            x[1], x[2], x[4]);
                    }
                } else {
                    println!("{} is not a registered department", x[4]);
                }
            }
            _ => println!("Invalid command.")
        }
    }
}
