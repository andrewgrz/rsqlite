
use std::io;
use std::io::Write;

enum StatementType {
    Insert,
    Select,
}

fn prepare_statement(input: &str) -> Result<String, String> {
    Ok("Success!".to_string())
}

fn main() {
    loop {
        print!("db > ");
        let _ = io::stdout().flush();

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let cmd = input.trim();

                match cmd {
                    ".exit" => {
                        println!("Goodbye!");
                        break
                    },
                    ".hello" => println!("Hello!"),
                    _ => {
                        if cmd.starts_with('.') {
                            println!("Unrecognized meta command: {}", cmd);
                        }

                        match prepare_statement(cmd) {
                            Ok(res) => {
                                // execute_statement
                            },
                            Err(e) => println!("{}", e)
                        }
                    }
                }
            }

            Err(error) => println!("error: {}", error),
        }
    }
}
