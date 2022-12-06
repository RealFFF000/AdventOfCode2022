use std::fs;
use std::io;
use std::string::String;
fn main() {
    let file_contents = fs::read_to_string("../input.txt")
        .expect("nie znaleziono pliku");
    let mut analysedstring: String = "".to_string();
    for line in file_contents.lines(){
        let mut counter = 0;
        for character in line.chars(){
            analysedstring.push(character);
            counter+=1;
            if analysedstring.len() > 14{
                analysedstring.remove(0);
                let mut correct = true;
                for a in 0..13{
                    for b in 0..13{
                        if a != b{
                            if (analysedstring.chars().nth(a) ) == (analysedstring.chars().nth(b)){
                                correct = false;
                            }
                        }
                    }
                }
                if correct == true{
                    println!("{} {}",analysedstring,counter);
                }
            }
        }
    }
}