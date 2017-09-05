
use std::io;
use std::io::Write;

enum StatementType {
    Insert,
    Select,
}

fn prepare_statement(input: &str) -> Result<StatementType, String> {
    if input.to_lowercase().starts_with("select") {
        return Ok(StatementType::Select)
    }
    if input.to_lowercase().starts_with("insert") {
        return Ok(StatementType::Insert)
    }
    Err(format!("Unrecognized keyword at start of: {}", input))
}

fn execute_statement(statement: StatementType) {
    match statement {
        StatementType::Insert => println!("Do an insert"),
        StatementType::Select => println!("Do a select statement")
    };
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
                            continue;
                        }

                        match prepare_statement(cmd) {
                            Ok(res) => {
                                execute_statement(res)
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
