fn main() {
    let mut word = String::from("hbello qsdlkjhqlsdjfhqd");

    let toto = first_word(&word);
    println!("toto {}", toto);

    word.clear();

    println!("word {}", word);
}

fn first_word(s: &String) -> usize {
    let size = s.as_bytes();

    for (i, &item) in size.iter().enumerate() {
        if item == b' ' {
            println!("i {}", i);
            return i;
        }
    }

    s.len()
}

// fn calculate_length(s: String) -> usize {
//         s.len()
// }

// use std::io;
// use std::cmp::Ordering;
// use rand::Rng;

// fn main() {
//     println!("Guess the number!");

//     let secret_number: u8 = rand::thread_rng().gen_range(1, 101);

//     println!("The secret number is: {}", secret_number);

//     loop {

//         println!("Please input your guess.");
//         let mut guess = String::new();
//         io::stdin()
//             .read_line(&mut guess)
//             .expect("Failed to read line");
//             let guess: u8 = match guess.trim().parse() {
//                 Ok(num) => num,
//                 Err(_) => continue,
//             };
//         println!("You guesse: {}", guess);
//         match guess.cmp(&secret_number) {
//             Ordering::Less => println!("Too small!"),
//             Ordering::Greater => println!("Too big!"),
//             Ordering::Equal => {

//                 println!("You win!");
//                 break;
//             }
//         }
//     }

// }
