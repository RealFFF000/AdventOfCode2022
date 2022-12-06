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
            if analysedstring.len() > 4{
                analysedstring.remove(0);

                if (analysedstring.chars().nth(0) ) != (analysedstring.chars().nth(1) ) && (analysedstring.chars().nth(0) ) != (analysedstring.chars().nth(2) ) && (analysedstring.chars().nth(0) ) != (analysedstring.chars().nth(3) ){
                    if (analysedstring.chars().nth(1) ) != (analysedstring.chars().nth(2) ) && (analysedstring.chars().nth(1) ) != (analysedstring.chars().nth(3) ){
                        if (analysedstring.chars().nth(2) ) != (analysedstring.chars().nth(3) ){
                            println!("{}",analysedstring);
                            println!("{}", counter);
                            break;
                        }
                    }
                }
            }
        }
    }
}