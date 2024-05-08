use rand::Rng;
use std::{cmp::Ordering, io::stdin};

fn main() {
    let number = rand::thread_rng().gen_range(1..=1000);

    loop {
        println!("insert a number: ");
        
        let mut guess: String = String::new();
        stdin().read_line(&mut guess).expect("Failed to read line");

        
        let guess:i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number");
                continue;
            },
        };



        match guess.cmp(&number) {
            Ordering::Less => println!("To low"),
            Ordering::Greater => println!("Yu high mate"),
            Ordering::Equal => {
                println!("You did it!!!11!1!11  the number was indeed {number}");
                break;
            },
        }
    }
    
}
