use std::io; //use "use" to get type

fn main() { //main() means no parameter 
    println!("Guess the number!");
    println!("Please input your guess.");

    /* In rust, variable is immutable default. (like const)
     * If you want to make your variable mutable, use "mut" keyword.
     */

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess) 
        /* & means reference. By default, reference is also immutable. */
        // So, we set keyword "mut" before "guess".
        .expect("Failed to read line"); 
        println!("You guess : {guess}");

    }
