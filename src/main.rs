
use std::io;

fn main() {
    loop {
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let cmd = input.trim();

                match cmd {
                    ".exit" => break,
                    ".hello" => println!("Hello!"),
                    _ => {
                        println!("Unrecognized command: {}", cmd);
                    }
                }
            }

            Err(error) => println!("error: {}", error),
        }
    }
}
