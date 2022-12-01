use std::fs;
use std::io;

fn main() {
    let mut elf_changed = false;
    let mut temporary_counter = 0;
    let mut highest = 0;
    let file_contents = fs::read_to_string("../input.txt")
        .expect("nie znaleziono pliku");
    for line in file_contents.lines() {
        if let Ok(result) = line.parse::<usize>() {
            println!("{}", result);
            temporary_counter+=result;
        }else{
            println!(" {}", temporary_counter);
            if temporary_counter > highest {
                highest = temporary_counter;
            }
            temporary_counter = 0; 
            println!(""); 
  
        }
        
    }
    println!("{}", highest)
}   
