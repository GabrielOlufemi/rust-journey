use std::io;
use rand::Rng;

fn main () { 

    // random number generation
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {

        let mut user_guess = String::new();

        println!("Make a guess: ");

        // read user input 
        io::stdin()
            .read_line(&mut user_guess)
            .expect("Failed to read line");

        // parse user input to integer
        let user_guess = user_guess.trim().parse::<i32>().expect("Please type a number!");

        // println!("You guessed: {}", user_guess);

        // println!("The secret number is: {}", secret_number);

        if user_guess < secret_number {
            println!("Too small!");
        } else if user_guess > secret_number {
            println!("Too big!");
        } else {
            println!("You win!");
            break;
        }
    }

}



// fn main() {
//     let num1 = 5;
//     let num2 = 6;

//     let result = multiply(num1, num2);
//     println!("The result is {}", result);


//     // is even check 
//     if is_even(result) {
//         println!("The result is even");
//     } else {
//         println!("The result is odd");
//     }

//     // square the result fn
//     let squared_result = square(result);
//     println!("The squared result is {}", squared_result);

//     // check
//     if result > 0 {
//         println!("The result is positive");
//     } else if result < 0 {
//         println!("the result is negative");
//     } else {
//         println!("The result is zero");
//     }

//     // for loop printing numbers from 1 to 10
//     for i in 1..=result {

//         if i % 2 == 0 {
//             println!("Even: {}", i);
//         }

//     }

//     // while loop p
//     let mut counter = 10;

//     while counter > 0 {
//         print!("{} ", counter);
//         counter -= 1;

//         if counter == 0 {
//             println!("\nBlastoff!");
//         }
//     }


//     // loop - till 10 reached 
//     let mut counter2 = 0;
//     loop {
//         counter2 += 1;

//         if counter2 == 10 {
//             println!("Counter reached 10, breaking the loop.");
//             break;
//         }
//     }

//     // loop - continue when 5 is reached

//     let mut counter3 = 1;
//     loop {
//         counter3 += 1;

//         if counter3 == 5 {
//             println!("Counter reached 5, continuing to next iteration.");
//             continue;
//         }

//         print!("{} ", counter3);

//         if counter3 == 10 {
//             println!("Counter reached 10, breaking the loop.");
//             break;
//         }
//     }


// }


// fn main() {
    
//     for i in -5 ..= 5 {
//         let classfication = classify_number(i);
//         println!("{} is classified as {}", i, classfication);
//     }

// }

// fn classify_number(num: i32) -> &'static str {

//     if num == 0 {
//      "zero"   
//     } else if num > 0{
//         if num % 2 == 0 {
//             "even positive"
//         }else {
//             "odd positive"
//         }
//     } else {
//         if num % 2 == 0 {
//             "even negative"
//         } else {
//             "odd negative"
//         }
//     }
// }



// fn multiply(int1: i32, int2:i32) -> i32 {
//     int1 * int2
// }

// fn is_even(num: i32) -> bool {
//     num % 2 == 0
// }

// fn square(num: i32) -> i32 {
//     num * num
// }