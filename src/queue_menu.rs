use std::io::{self, Write};
use crate::queue::Queue; 

pub fn run_queue_menu() {
    let mut queue = Queue::new();

    loop {
        println!("\nQueue menu: ");
        println!("--------------------------------------------------------------------------------");
        println!("1. Enqueue");
        println!("2. Dequeue");
        println!("3. Peek");
        println!("4. Check if queue is empty");
        println!("5. Get the queue size");
        println!("6. -> Back to main menu");

        println!("\nChoose an option: ");
        io::stdout().flush().unwrap();

        let mut queue_choice = String::new();
        io::stdin().read_line(&mut queue_choice).expect("Failed to read line");
        let queue_choice = queue_choice.trim().parse::<u32>().unwrap_or(0);

        match queue_choice {
            1 => {
                println!("\nEnter value to enqueue: ");
                io::stdout().flush().unwrap();
                let mut push_value = String::new();
                io::stdin().read_line(&mut push_value).expect("Failed to read line");

                if let Ok(num) = push_value.trim().parse::<i32>() {
                    queue.enqueue(num);
                } else {
                    println!("Invalid input.");
                }
            }
            2 => {
                match queue.dequeue() {
                    Some(value) => println!("\n* Dequeue {} from the queue!", value),
                    None => println!("\n* Cannot dequeue! Queue is empty"),
                }
            }
            3 => {
                match queue.peek() {
                    Some(&value) => println!("\n* First value is {}", value),
                    None => println!("\n* Cannot peek! Queue is empty"),
                }
            }
            4 => {
                if queue.is_empty() {
                    println!("\n* Queue is empty");
                } else {
                    println!("\n* Queue is not empty");
                }
            }
            5 => {
                println!("\n* Queue size is {}", queue.size());
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