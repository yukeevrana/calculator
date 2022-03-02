use colored::Colorize;
use std::io;

fn main() {
    let mut calc = Calc::new();

    loop {
        let mut buffer = String::new();
        let stdin = io::stdin();

        stdin.read_line(&mut buffer).expect("Input error"); 

        match calc.take_message(String::from(buffer.trim_end())) {
            Ok(n) => { println!("{n}") },
            Err(error) => { 
                match error {
                    "exit" => break,
                    _ => println!("{error}")
                } 
            }
        }
    }
}

struct Calc {
    output_color: String,
    buffer: String
}

impl Calc {
    fn new() -> Self {
        Calc {
            output_color: String::from("default"),
            buffer: String::new()
        }
    }

    fn take_message(&mut self, message: String) -> Result<colored::ColoredString, &str> {
        self.buffer = message.clone();

        // TODO Move output from command match to color match. Change buffer in command match instead.
        match self.buffer.as_str() {
            "exit" => Err("exit"),
            "clear color" => { self.output_color = String::from("default"); Ok(String::from("Done").white()) },
            "blue" => { self.output_color = String::from("blue"); Ok(String::from("Done").blue()) },
            "green" => { self.output_color = String::from("green"); Ok(String::from("Done").green()) },
            "red" => { self.output_color = String::from("red"); Ok(String::from("Done").red()) },
            _ => {
                match self.output_color.as_str() {
                    "blue" => Ok(self.buffer.blue()),
                    "green" => Ok(self.buffer.green()),
                    "red" => Ok(self.buffer.red()),
                    _ => Ok(self.buffer.white())
                }
            }
        }
    }
}




