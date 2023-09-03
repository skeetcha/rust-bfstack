use std::io::{self, Read};
use std::io::Write;

fn interpret(input: &String, stack: &mut Vec<i64>) {
    let mut prog_loc = 0;
    let mut locs: Vec<usize> = Vec::new();

    while prog_loc < input.len() {
        match input.chars().nth(prog_loc).unwrap() {
            '>' => stack.push(0),
            '<' => {
                if stack.len() > 0 {
                    let _ = stack.pop().unwrap();
                } else {
                    println!("Stack is empty");
                }
            },
            '.' => {
                if stack.len() > 0 {
                    print!("{}", char::from(((*stack.last().unwrap()) % 255) as u8));
                } else {
                    println!("Stack is empty");
                }
            },
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
                if *stack.last().unwrap() == 0 {
                    let mut matches = 1;

                    while matches != 0 {
                        prog_loc += 1;

                        if prog_loc == input.len() {
                            break;
                        } else if input.chars().nth(prog_loc).unwrap() == '[' {
                            matches += 1;
                        } else if input.chars().nth(prog_loc).unwrap() == ']' {
                            matches -= 1;
                        }
                    }
                } else {
                    locs.push(prog_loc);
                }
            },
            ']' => {
                prog_loc = locs.pop().unwrap() - 1;
            },
            _ => ()
        };

        prog_loc += 1;
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
