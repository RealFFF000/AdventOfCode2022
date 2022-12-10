use std::fs;
use std::string::String;
use std::vec;
use std::array;
use std::fs::File;

fn main() {
    let file_contents = fs::read_to_string("../input.txt")
        .expect("nie znaleziono pliku");
    let mut headpos = [0,0];
    let mut length:i32 = 9;
    let mut tailpos = [[0,0];10];
    let mut counter = 0;
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
        println!("{:?}",direction);
        println!("{}",distance);
        for a in 0..distance{
            headpos[0] += direction[0];
            headpos[1] += direction[1];
            for len in 0..(length as usize){
                let mut following: [i32; 2];
                if len == 0{
                    following = headpos;
                }else{
                    following = tailpos[len-1];
                }
                if tailpos[len][0] > following[0]+1 || tailpos[len][0] < following[0]-1 || tailpos[len][1] > following[1]+1 || tailpos[len][1] < following[1]-1{
                    tailpos[len][0] = following[0]-direction[0];
                    tailpos[len][1] = following[1]-direction[1];   
                }
                if len as i32 == length-1{
                    if tailused.contains(&tailpos[len]){
                        print!("pominieto [{},{}] \n", tailpos[len][0], tailpos[len][1]);
                    }else{
                        print!("wpisano [{},{}] \n", tailpos[len][0], tailpos[len][1]);
                        tailused.push([tailpos[len][0],tailpos[len][1]]);
                    }
                }
                

            }
            
        }

    }
    print!("{} ", tailused.len());
    
}
    

