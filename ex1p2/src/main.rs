use std::fs;
use std::io;

fn main() {
    let mut elf_changed = false;
    let mut temporary_counter = 0;
    let mut highest1 = 0;
    let mut highest2 = 0;
    let mut highest3 = 0;
    let file_contents = fs::read_to_string("../input.txt")
        .expect("nie znaleziono pliku");
    for line in file_contents.lines() {
        if let Ok(result) = line.parse::<usize>() {
            println!("{}", result);
            temporary_counter+=result;
        }else{
            println!(" {}", temporary_counter);
            if temporary_counter > highest1 {
                highest3 = highest2;
                highest2 = highest1;
                highest1 = temporary_counter;
            }else if temporary_counter > highest2 {
                highest3 = highest2;
                highest2 = temporary_counter;
            }else if temporary_counter > highest3 {
                highest3 = temporary_counter;
            }

            temporary_counter = 0; 
            println!(""); 
  
        }
        
    }
    println!("{}", highest1);
    println!("{}", highest2);
    println!("{}", highest3);
    println!("{}", highest1+highest2+highest3);
}   
