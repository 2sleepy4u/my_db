use std::io;
use std::io::Write;

const DB_SYSTEM_NAME: &str = "my_db";
const INSERT_CMD: &str = "insert";
const QUERY_CMD: &str = "query";
const TABLE_CMD: &str = "table";

fn main() -> io::Result<()> {
    let mut exit = false;
    let stdin = io::stdin();
    let mut input: String = String::new();

    while !exit {
        print!("{}> ", DB_SYSTEM_NAME);
        io::stdout().flush().unwrap();
        stdin.read_line(&mut input);
        
        let command = input.trim().split(" ").collect::<Vec<&str>>()[0];

        match command {
            "exit" => exit = true,
            TABLE_CMD => println!("{}", input),
            INSERT_CMD => println!("{}", input),
            QUERY_CMD =>  println!("{}", input),
            &_ => println!("Unrecognized command: {}", input.trim())
        }

        input = String::new();
    }

    Ok(())
}

