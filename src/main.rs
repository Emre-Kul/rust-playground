use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    // hello_world();
    // guess_the_number();
    // guess_the_random_number();
    // mut_example();
    // const_example();
    // shadow_example();
    // scalar_types_example();
    // compound_types_example();
    // println!("Result of 3 + 2 = {}", sum(3, 2));
    // println!("Result of 5 + 2 = {}", calc('+', 5, 2));
    // println!("Result of 5 - 2 = {}", calc('-', 5, 2));
    // println!("Result of 5 / 2 = {}", calc('/', 5, 2));
    // println!("Result of 5 * 2 = {}", calc('*', 5, 2));
    // condition_example(22);
    // loop_example();
    while_example();
}


fn hello_world() {
    println!("Hello, world!!!");
}

fn guess_the_number() {
    println!("Guess the Number");
    println!("Please input ur guess");
    let mut guess: String = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    println!("Your Guess : {}", guess);
}

fn guess_the_random_number() {
    println!("Guess the number");
    let secret_number: i32 = rand::thread_rng().gen_range(1, 10);
    println!("Secret number is : {}", secret_number);
    loop {
        println!("Please input ur guess");
        let mut guess: String = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line!");
        let guess: i32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Your Guess : {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("U guess small!"),
            Ordering::Greater => println!("U guess big!"),
            Ordering::Equal => {
                println!("U guess right!");
                break;
            },
        }
    }
}

fn mut_example() {
    let mut x = 5;
    println!("X: {}", x);
    x = 10;
    println!("X: {}", x);
}

fn const_example() {
    const X: i32 = 22;
    println!("const X : {}", X);
}

fn shadow_example() {
    let x = 10;
    println!("(1)x : {}", x);
    let x = 20;
    println!("(2)x : {}", x);
    let x = x + 1;
    println!("(3)x : {}", x);
}

fn scalar_types_example() {
       let x: u8 = 5;
    let x: i8 = 6;
    let x: u16 = 1;
    let x: i16 = 11;
    let x: u32 = 11;
    let x: i32 = 22;
    let x: u64 = 33;
    let x: i64 = 22;
    let x: u128 = 88;
    let x: i128 = 44;
    let x: usize = 999;
    let x: isize = 999;
    let x: f32 = 0.666;
    let x: f64 = 0.9999;
    let x: bool = false;
    let x: char = 'a';
    println!("Finished! types example!");
}

fn compound_types_example() {
    let tup: (u32, char, bool) = (1, 'a', true);
    let (x, y, z) = tup;
    let x = tup.0;
    let y = tup.1;
    let z = tup.2;
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let arr2 = [0;10];
    let first = arr2[0];
    let second = arr2[1];
    println!("First : {} Second: {}", first, second);
}

fn sum(num1: u32, num2: u32) -> u32 {
    num1 + num2
}

fn calc(calc_type: char, num1: u32, num2: u32) -> u32 {
    return match calc_type {
        ('+') => num1 + num2,
        ('-') => num1 - num2,
        ('/') => num1 % num2,
        ('*') => num1 * num2,
        (_) => 0
    }
}

fn condition_example(age: u32) {
    if age < 18 {
        println!("Small");
    } else if age > 30 {
        println!("Big");
    } else {
        println!("Ok");
    }


    let x: u32 = if age > 18 {
        age + 100
    } else {
        age + 1000
    };

    println!("New Age : {}", x);
}

fn loop_example() {
    let mut counter: u32 = 10;
    let result = loop {
        println!("Loop...");
        if counter == 0 {
            break counter;
        }
        counter-=1;
    };
    println!("Counter val at the end : {} ", counter);
}

fn while_example() {
    let start_of_fib = [1, 2, 3, 5, 8, 13];
    let mut index = 0;
    let mut counter = 0;
    while counter < 10 {
        println!("While loop...");
        counter+=1;
    }

    while index < 6 {
        println!("Fib at {} => {} ", index, start_of_fib[index]);
        index+= 1;
    }

    for elem in start_of_fib.iter() {
       println!("For-In {}", elem);
    }

}

