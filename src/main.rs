extern crate rand;
extern crate slug;
use rand::Rng;
use slug::slugify;
use std::env;
use std::io;

pub fn use_env_arg(used_arg: &String, user_input: String) -> String {
    match used_arg as &str {
    "lowercase" => user_input.to_lowercase(),
    "uppercase" => user_input.to_uppercase(),
    "no-spaces" => user_input.replace(" ", ""),
    "slugify" => slugify(user_input),
    "go-nuts" => go_nuts(user_input),
    "how-long" => "This took me 19 days to make".to_string(),
    _ => "The argument provided is all kinds of weird... Try again... Pick either one of these \n1. lowercase \n2. uppercase \n3. no-spaces \n4. slugify \n5. go-nuts".to_string(),
}
}

fn go_nuts(user_input: String) -> String {
    let potenial_args: [&str; 4] = ["lowercase", "uppercase", "no-spaces", "slugify"];
    let mut rng = rand::thread_rng();
    let random_num: usize = rng.gen_range(0..4);
    let picked_arg = potenial_args[random_num].to_string();
    println!("{} was picked!", picked_arg);
    return use_env_arg(&picked_arg, user_input);
}

fn main() {
    let mut user_input = String::new();

    println!("Hey give me a string to format!");

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line...");

    let mut used_args: Vec<String> = env::args().collect();
    used_args.remove(0);

    if used_args.is_empty() {
        println!("No arguments were given: {}", user_input);
    } else {
        let used_arg = &used_args[0];
        println!("Here is the formated string: {}", use_env_arg(&used_arg, user_input));
    }
}
