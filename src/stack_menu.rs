use std::io::{self, Write};
use crate::stack::Stack; 

pub fn run_stack_menu() {
    let mut stack = Stack::new();

    loop {
        println!("\nStack menu: ");
        println!("--------------------------------------------------------------------------------");
        println!("1. Push");
        println!("2. Pop");
        println!("3. Peek");
        println!("4. Check if stack is empty");
        println!("5. Get the stack size");
        println!("6. -> Back to main menu");

        println!("\nChoose an option: ");
        io::stdout().flush().unwrap();

        let mut stack_choice = String::new();
        io::stdin().read_line(&mut stack_choice).expect("Failed to read line");
        let stack_choice = stack_choice.trim().parse::<u32>().unwrap_or(0);

        match stack_choice {
            1 => {
                println!("\nEnter value to push: ");
                io::stdout().flush().unwrap();
                let mut push_value = String::new();
                io::stdin().read_line(&mut push_value).expect("Failed to read line");

                if let Ok(num) = push_value.trim().parse::<i32>() {
                    stack.push(num);
                } else {
                    println!("Invalid input.");
                }
            }
            2 => {
                match stack.pop() {
                    Some(value) => println!("\n* Popped {} from the stack!", value),
                    None => println!("\n* Cannot pop! Stack is empty"),
                }
            }
            3 => {
                match stack.peek() {
                    Some(&value) => println!("\n* Top value is {}", value),
                    None => println!("\n* Cannot peek! Stack is empty"),
                }
            }
            4 => {
                if stack.is_empty() {
                    println!("\n* Stack is empty");
                } else {
                    println!("\n* Stack is not empty");
                }
            }
            5 => {
                println!("\n* Stack size is {}", stack.size());
            }
            6 => {
                println!("\n* Exiting to main menu");
                break;
            }
            _ => {
                println!("Invalid option");
            }
        }
    }
}