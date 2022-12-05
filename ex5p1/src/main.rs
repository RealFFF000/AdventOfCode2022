use std::fs;
use std::io;
use std::string::String;
use std::str::Split;
use std::str::FromStr;
fn main() {
    let file_contents = fs::read_to_string("../input.txt")
        .expect("nie znaleziono pliku");
    let mut stacks: [String; 9] = [String::new(),String::new(),String::new(),String::new(),String::new(),String::new(),String::new(),String::new(),String::new()];
    let mut extracteddata= [0;100];
    let mut counter = 0;
    let mut countertwo = 0;
    for line in file_contents.lines(){
        if line != " 1   2   3   4   5   6   7   8   9 "{
            counter = 0;
            for character in line.chars(){
                print!("{}",character);
                if counter >= 1{
                    if (((counter-1)/4) % 1) == 0{
                        if character != '[' && character != ']' && character != ' '{
                            stacks[(counter-1)/4].push(character);
                        }
                    }
                    
                }
                counter+=1;

            }
            print!("\n");
        }
        else{
            break;
        }
        
    }
    println!("\n");
    for a in 0..(stacks.len()){
        for character in stacks[a].chars(){
                print!("{}",character);
        }
        print!("\n");
    }
    counter = 0;
    for line in file_contents.lines(){
        let mut linewithoutwords = line;
        counter+=1;
        if counter > 10{
            let a = &linewithoutwords.replace(&['m', 'o', 'v', 'e', 'f', 'r', 't', ' '][..], "");
            let mut x:i32 = 0;
            let mut y:i32 = 0;
            let mut z:i32 = 0;
            if a.len() == 3{
                x = a.chars().nth(0).unwrap() as i32 - 0x30;
                y = a.chars().nth(1).unwrap() as i32 - 0x30;
                z = a.chars().nth(2).unwrap() as i32 - 0x30;
            }
            else if a.len() == 4{
                x = ((a.chars().nth(0).unwrap()as i32 - 0x30)*10)  + a.chars().nth(1).unwrap() as i32 - 0x30;
                y = a.chars().nth(2).unwrap() as i32 - 0x30;
                z = a.chars().nth(3).unwrap() as i32 - 0x30;
            }
            //println!("{},{},{}",x,y,z);
            
            while x > 0{
                let mut letter = stacks[(y-1) as usize].chars().nth(0).unwrap();
                stacks[(y - 1) as usize].remove(0);
                stacks[(z - 1) as usize ].insert(0, letter);
                x-=1;
            }   
        }
        //println!("");
    }
    for a in 0..(stacks.len()){
        for character in stacks[a].chars(){
                print!("{}",character);
        }
        print!("\n");
        

    }
}