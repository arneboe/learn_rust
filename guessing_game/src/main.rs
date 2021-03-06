extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Input Guess");

    let secret_num: i32 = rand::thread_rng().gen_range(0, 100);

    loop {
        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        //parse to int and shadow variable
        //trim also removes the '\n'
        //parse knows what type to return by type inference
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("secret_num {}", secret_num);
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("equal");
                break;
            }
        }
    }
}
