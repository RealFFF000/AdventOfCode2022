use std::fs;
use std::string::String;
use std::vec;
use std::array;
use std::fs::File;

fn main() {
    let file_contents = fs::read_to_string("../input.txt")
        .expect("nie znaleziono pliku");
    let mut data: Vec<Vec<i32>> = vec![vec![]];
    let mut visible: Vec<Vec<i32>> = vec![vec![]];
    let mut lines = 0;
    for line in file_contents.lines(){
        let mut highest = -1;
        let mut chars = 0;
        for character in line.chars(){
            let num = character.to_string().parse::<i32>().unwrap();
            print!("{}",num);
            if num > highest{
                highest = num;
                visible[lines].push(1);
            }
            else{
                visible[lines].push(0);
            }
            
            data[lines].push(num);
            chars += 1;
        }
        lines+=1;
        data.push(vec![]);
        visible.push(vec![]);
        print!("\n");
    }   
    let mut lines = 0; 
    for line in file_contents.lines(){
        let mut highest = -1;
        let mut chars = data[1].len()-1;
        for character in line.chars(){
            let num = data[lines][chars].to_string().parse::<i32>().unwrap();
            print!("{}",num);
            if num > highest{
                highest = num;
                visible[lines][chars] = 1;
            }
            chars -= 1;
        }
        lines+=1;
        print!("\n");
    }
    
    print!("\n");

    for line in visible{
        for char in line{
            print!("{}",char);
        }
        print!("\n");
    }
}
