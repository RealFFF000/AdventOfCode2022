use std::fs;
use std::string::String;
use std::vec;
use std::array;
use std::fs::File;

fn main() {
    let file_contents = fs::read_to_string("../input.txt")
        .expect("nie znaleziono pliku");
    let mut mode = 0; // 0 - normal, 1 - addx
    let toprint = [20,60,100,140,180,220];
    let mut distance = 0;
    let mut regstate = 1;
    let mut suma = 0;
    for line in file_contents.lines(){
        if line.contains("addx"){
            mode = 2;
        }else{
            mode = 1;
        }
        for a in 0..mode{

            distance += 1;
            if toprint.contains(&distance){
                println!("{}", regstate*distance);
                suma += regstate*distance;
            }
            if a == 0 && mode == 1{
                //println!("noop");
            }
            if a == 1 && mode == 2{
                let mut number:i32 = 0;
                for character in line.chars(){
                    if character.is_numeric(){
                        number = number*10 + character.to_digit(10).unwrap() as i32;
                    }
                }
                if line.contains("-"){
                    number = number * -1;
                }
                //println!("numer to {}",number); 
                regstate += number;
                break;
            }
        }
    }
    println!("suma to {}",suma);
    
}
