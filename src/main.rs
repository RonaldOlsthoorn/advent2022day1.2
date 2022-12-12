use std::fs::File;
use std::io::{prelude::*, BufReader};


fn main() {

    let mut gold_calories = 0;
    let mut silver_calories = 0;
    let mut bronze_calories = 0;

    let mut current_elf_calories = 0;

    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {

        match line.unwrap().parse::<i32>(){
            Ok(_increment)=>{
                current_elf_calories += _increment;
            },
            Err(_e)=>{
                if current_elf_calories > gold_calories {
                    bronze_calories = silver_calories;
                    silver_calories = gold_calories;
                    gold_calories = current_elf_calories;
                } else if current_elf_calories > silver_calories {
                    bronze_calories = silver_calories;
                    silver_calories = current_elf_calories;
                } else if current_elf_calories > bronze_calories {
                    bronze_calories = current_elf_calories;
                }
                current_elf_calories = 0;
            }
        }
    }

    println!("gold calories: {}", gold_calories);
    println!("silver calories: {}", silver_calories);
    println!("bronze calories: {}", bronze_calories);

    println!("total calories: {}", gold_calories + silver_calories + bronze_calories);

}
