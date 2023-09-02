use std::io::{self, Read};
use std::io::Write;

fn interpret(input: &String, stack: &mut Vec<i64>) {
    let mut loop_loc = -1;

    for (prog_loc, op) in input.chars().enumerate() {
        match op {
            '>' => stack.push(0),
            '<' => {
                let _ = stack.pop().unwrap();
            },
            '.' => println!("{}", stack.last().unwrap()),
            ',' => {
                let mut buf: [u8; 1] = [b'\0'];
                io::stdin().read_exact(&mut buf).expect("failed to read character");
                stack.push(buf[0] as i64);
            },
            '+' => {
                *stack.last_mut().unwrap() += 1;
            },
            '-' => {
                *stack.last_mut().unwrap() -= 1;
            },
            '[' => {

            },
            ']' => {

            },
            _ => ()
        };
    }
}

fn main() {
    let mut stack: Vec<i64> = Vec::new();
    println!("Rust BFStack Interpreter v0.1\nWritten by Cass Unterholzner");
    let mut quit = false;

    while !quit {
        print!(">>> ");
        let _ = io::stdout().flush();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to readline");
        input = input.strip_suffix('\n').unwrap().to_string();

        if input == "quit" {
            quit = true;
        } else {
            interpret(&input, &mut stack);
        }
    }
}
