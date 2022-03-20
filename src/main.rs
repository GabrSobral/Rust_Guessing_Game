use std::io::stdin;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    print!("\x1B[2J\x1B[1;1H");
    println!("Input your first guess");
    let _secret_number: u32 = rand::thread_rng().gen_range(1..101);

    loop {
        let mut guess: String = String::new();

        stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        print!("\x1B[2J\x1B[1;1H");
        println!("Your guess is {}", guess);

        match guess.cmp(&_secret_number) {
            Ordering::Less => println!("Too small!, try another."),
            Ordering::Greater => println!("Too big!, try another."),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
