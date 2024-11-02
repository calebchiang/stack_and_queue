use std::io::{self, Write};

mod stack;
mod queue;
mod stack_menu;
mod queue_menu;

fn main() {
    println!("--------------------------------------------------------------------------------");
    println!("\nWelcome to the Stack and Queue Program!");
    println!("This program allows you to interact with a fully functioning stack or queue...");
    println!("\n--------------------------------------------------------------------------------");
    
    loop {
        println!("\nMain Menu");
        println!("\n1.) Stack");
        println!("2.) Queue");
        println!("3.) Exit Program");
        println!("\nChoose data structure to interact with: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input = input.trim().parse::<u32>().unwrap_or(0);
        match input {
            1 => {
                stack_menu::run_stack_menu();
            }
            2 => {
                queue_menu::run_queue_menu();
            }
            3 => {
                println!("Goodbye...");
                break;
            }
            _ => { println!("Invalid input") }
        }
    }
}
