extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    forTest();
}

fn forTest(){
    for x in 1..10{
        if (x % 2) == 0 { continue;}
        println!("x {}",x);
    }
}

fn testIf(val : i32){
    let y = if val < 10 { "less than 10" } else {"gt 10"};
    println!("the value is {}",y);
}

fn testTuple() {
    let a = (1, 2, 3);
    println!("test {}", a.1);
}

fn game() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}