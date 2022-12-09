use std::fs;
use std::string::String;
use std::vec;
use std::array;
use std::fs::File;

fn main() {
    let file_contents = fs::read_to_string("../input.txt")
        .expect("nie znaleziono pliku");
    let mut headpos = [0,0];
    let mut tailpos = [0,0];
    let mut length = 1;
    let mut tailused:Vec<[i32; 2]> = vec![];
    for line in file_contents.lines(){
        let mut direction = [0,0];
        let mut distance = 0;
        for character in line.chars(){
            if character == 'R'{
                direction = [1,0];
            }else if character == 'U'{
                direction = [0,1];
            }else if character == 'L'{
                direction = [-1,0];
            }else if character == 'D'{
                direction = [0,-1];
            }

            if character.is_numeric(){
                distance = distance*10 + character.to_digit(10).unwrap();
            }
        }
        for a in 0..distance{
            headpos[0] += direction[0];
            headpos[1] += direction[1];
            if tailpos[0] > headpos[0]+length || tailpos[0] < headpos[0]-length || tailpos[1] > headpos[1]+length || tailpos[1] < headpos[1]-length{
                tailpos[0] = headpos[0]-direction[0];
                tailpos[1] = headpos[1]-direction[1];   
            }
            if tailused.contains(&tailpos){
                print!("pominieto [{},{}] \n", tailpos[0], tailpos[1]);
            }else{
                tailused.push([tailpos[0],tailpos[1]]);
            }
        }
        
    }
    print!("{} ", length);
    print!("{} ", tailused.len());
}
