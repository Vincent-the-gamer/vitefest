use std::io::stdin;

use crate::executer::frameworks::Frameworks;

const FRAMEWORKS: &str = include_str!("../../resources/choose_framework.txt");

pub fn choose_framework() -> Frameworks {
    println!("{}", FRAMEWORKS);
    let mut input_str = String::new();
    stdin().read_line(&mut input_str).expect("Cannot read your choice!");

    match input_str.trim() {
        "1" => Frameworks::React,
        "2" => Frameworks::Vue,
        "3" => Frameworks::Svelte,
        "4" => Frameworks::Solid,
        "5" => Frameworks::Qwik,
        _ => {
            println!("Your input is invalid!");
            Frameworks::Nothing
        }
    }
}