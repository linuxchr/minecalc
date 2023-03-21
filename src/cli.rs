use super::calc;
use super::filecalc;
use std::io;
use std::io::Error;
use std::io::Write;

pub fn handler() {
    loop {
        println!("Options:");
        println!("1. Parse a file");
        println!("2. Items to boxes");
        println!("3. Boxes to items");
        let uinput = match input("Action 1-3, q: ") {
            Ok(uinput) => uinput,
            Err(_) => {
                println!("Invalid Input");
                continue;
            }
        };
        match uinput.to_ascii_lowercase().as_str() {
            "q" => {
                break;
            }
            "1" => {
                match filecalc::parse_file(
                    match input("Input Path: ") {
                        Ok(uinput) => uinput,
                        Err(_) => {
                            println!("Invalid Input");
                            continue;
                        }
                    },
                    match input("Output Path: ") {
                        Ok(uinput) => uinput,
                        Err(_) => {
                            println!("Invalid Input");
                            continue;
                        }
                    },
                ) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("Error: {e}");
                    }
                }
            }
            "2" => match input("Items: ") {
                Ok(uinput) => match uinput.trim().parse::<f64>() {
                    Ok(uinput) => match input("Item name: ") {
                        Ok(name) => println!(
                            "{uinput} {1}(s) are {0} Boxes",
                            calc::boxes_from_items(uinput, &name),
                            &name
                        ),
                        Err(_) => {
                            println!("Invalid Input");
                            continue;
                        }
                    },
                    Err(_) => {
                        println!("No valid number");
                        continue;
                    }
                },
                Err(_) => {
                    println!("Invalid Input");
                    continue;
                }
            },
            "3" => match input("Boxes: ") {
                Ok(uinput) => match uinput.trim().parse::<f64>() {
                    Ok(uinput) => match input("Item name: ") {
                        Ok(name) => println!(
                            "{uinput} Boxes are {0} {1}(s)",
                            calc::item_from_boxes(uinput, &name),
                            &name
                        ),
                        Err(_) => {
                            println!("Invalid Input");
                            continue;
                        }
                    },
                    Err(_) => {
                        println!("No valid number");
                        continue;
                    }
                },
                Err(_) => {
                    println!("Invalid Input");
                    continue;
                }
            },

            _ => {
                continue;
            }
        }
    }
}

fn input(promt: &str) -> Result<String, Error> {
    let mut xinput: String = String::new();
    print!("{}", promt);
    io::stdout().flush()?;
    io::stdin().read_line(&mut xinput)?;
    xinput.pop();
    Ok(xinput)
}
