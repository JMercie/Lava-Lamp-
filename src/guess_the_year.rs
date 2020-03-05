use std::io;

pub fn run() {
    println!(
        "Hello. We can tell you in wich year you turn 100!! Can you pls give us your name and age",
    );

    loop {
        let mut user_input = String::new();

        let exit = String::from("exit");

        io::stdin()
            .read_line(&mut user_input)
            .expect("make a valid input");

        if user_input == exit {
            break;
        }

        let mut age_input = String::new();
        io::stdin()
            .read_line(&mut age_input)
            .expect("make a valid input");

        let age_input: u32 = match age_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let date: u32 = 2020;

        let age_calculated: u32 = 100 - age_input;

        let result: u32 = date + age_calculated;

        println!("{} will turn 100 in year {}", user_input, result);

        break;
    }
}
