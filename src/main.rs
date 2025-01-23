extern crate rand;
extern crate slug;
use rand::Rng;
use slug::slugify;
use std::env;
use std::io;

fn main() {
    let mut user_input = String::new();

    println!("Hey give me a string to slugify!");

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line...");

    let used_arg: Vec<String> = env::args().collect();
    let used_arg = &used_arg[1];

    if used_arg.is_empty() {
        println!("{}", user_input);
    } else {
        println!("{}", use_env_arg(&used_arg, user_input));
    }
}

fn use_env_arg(used_arg: &String, user_input: String) -> String {
    match used_arg as &str {
        "lowercase" => user_input.to_lowercase(),
        "uppercase" => user_input.to_uppercase(),
        "no-spaces" => user_input.replace(" ", ""),
        "slugify" => slugify(user_input),
        "go-nuts" => go_nuts(user_input),
        _ => "This is all kinds of weird".to_string(),
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
