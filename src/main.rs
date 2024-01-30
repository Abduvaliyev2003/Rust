// // fn main() {
// //     println!("Hello, Asad!");
// // }


// struct Cli {
//     pattern: String,
//     path: std::path::PathBuf,
//  }

// fn main()
// {
//     let pattern = std::env::args().nth(1).expect("no pettern given");
//     let path = std::env::args().nth(2).expect("no path given");
//     let args = Cli {
//         pattern: pattern,
//         path: std::path::PathBuf::from(path),
//     };

//     println!("pattern: {:?}, path: {:?}", args.pattern, args.path);

//  }
// Processing a Guess

use std::io;

fn main ()
{
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}




