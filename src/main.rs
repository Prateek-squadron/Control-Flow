use std::io;

fn main() {

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");

    let guess: u64 = guess.trim().parse().expect("Please type a number");

    if guess == 5 {
        print!("The guess is correct");

    }else if guess == 2{
            println!("The guess is 2");

    
    }

}
