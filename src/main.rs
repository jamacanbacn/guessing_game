//included standard io which we can use for user input
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
//generate random number using rand crate
    println!("Guess the number!");
    let mut secret_number = rand::thread_rng().gen_range(1,101);
    /*
    This is for debugging
    println!("The secret number is {}",&mut secret_number);
    */
   loop{
    println!("Please input your guess.");

    let mut guess = String::new();
//accept user input
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line.");

// Rust lets us shadow the previous guess since we can't compared i32 and string
    let guess: u32 = match guess.trim().parse(){
        //Parse returns OK on number , err on the rest - underscore is a catchall
        Ok(num) => num,
        Err(_) => {println!("YOU MUST ENTER A NUMBER!"); continue
        }
    };
        //.expect("Please type a number!");

//print user guess
    println!("You guessed: {}",&guess);

//match the guess with the secret number
    match guess.cmp(&mut secret_number){
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {println!("You win!");
        break;
    }
    }

 } //end of loop
//end of main
}
