use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main(){

    let mut max_calories = 0;
    let mut current_elf_calories = 0;

    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {

        match line.unwrap().parse::<i32>(){
            Ok(_increment)=>{
                println!("number {}", _increment);
                current_elf_calories += _increment;
            },
            Err(_e)=>{
                if current_elf_calories > max_calories {
                    max_calories = current_elf_calories;
                }
                current_elf_calories = 0;
                println!("no number");
            }
        }
    }

    println!("max calories: {}", max_calories);
}