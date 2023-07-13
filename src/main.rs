use std::collections::VecDeque;
use std::fs;
use std::io;

const LENGTH: u32 = 65535;

fn read_file() -> String {
    let mut filename = String::new();
    println!("Enter file name: ");
    io::stdin().read_line(&mut filename).unwrap();
    let filename = filename.trim();
    let file_contents = fs::read_to_string(filename).expect("unable to read file");
    return file_contents.trim().to_string();
}

fn interpret(parsed_file: String) -> () {
    let mut memory: [u8; 30000] = [0; 30000];
    let mut pointer = 0;
    let mut counter = 0;
    let parsed_file = parsed_file
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<Vec<char>>();
    let mut stack: VecDeque<usize> = VecDeque::new();

    while counter < parsed_file.len() {
        if parsed_file[counter] == '>' {
            if pointer == LENGTH as usize {
                pointer = 0;
            } else {
                pointer += 1;
            }
        } else if parsed_file[counter] == '<' {
            if pointer == 0 {
                pointer = LENGTH as usize - 1;
            } else {
                pointer -= 1;
            }
        } else if parsed_file[counter] == '+' {
            memory[pointer] = memory[pointer].wrapping_add(1);
        } else if parsed_file[counter] == '-' {
            memory[pointer] = memory[pointer].wrapping_sub(1);
        } else if parsed_file[counter] == '.' {
            print!("{}", memory[pointer] as char);
        } else if parsed_file[counter] == ',' {
            let mut key = String::new();
            io::stdin().read_line(&mut key).unwrap();
            memory[pointer] = key.chars().nth(0).unwrap() as u8;
        } else if parsed_file[counter] == '[' {
            if memory[pointer] == 0 {
                let mut nested = 0;
                while counter < parsed_file.len() - 1 {
                    counter += 1;
                    if parsed_file[counter] == '[' {
                        nested += 1;
                    } else if parsed_file[counter] == ']' {
                        if nested == 0 {
                            break;
                        } else {
                            nested -= 1;
                        }
                    }
                }
            } else {
                stack.push_front(counter);
            }
        } else if parsed_file[counter] == ']' {
            if memory[pointer] != 0 {
                if let Some(jump_counter) = stack.front() {
                    counter = *jump_counter;
                }
            } else {
                stack.pop_front();
            }
        }
        counter += 1;
    }
}

fn main() {
    let parsed_file = read_file();
    interpret(parsed_file);
}
