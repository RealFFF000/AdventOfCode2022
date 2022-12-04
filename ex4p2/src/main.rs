use std::fs;
use std::io;
use std::str::Split;
use std::str::FromStr;

fn main() {
    let mut split;
    let mut splited;
    let mut extracteddata: [i32; 4] = [0,0,0,0]; 
    let mut biggercounter = 0;
    let mut counter = 0;
    let mut points = 0;
    let file_contents = fs::read_to_string("../input.txt")
        .expect("nie znaleziono pliku");
    for line in file_contents.lines() {   
        split = line.split(",");
        biggercounter = 0;
        for s in split{
            counter = 0;
            splited = s.split("-");
            for a in splited{
                extracteddata[counter+biggercounter*2] = i32::from_str(a).unwrap_or(0);
                counter+=1; 
            }
            biggercounter +=1;
        }
        println!("{},{},{},{}",extracteddata[0],extracteddata[1],extracteddata[2],extracteddata[3]);
        if extracteddata[1]>=extracteddata[2] && extracteddata[3]>=extracteddata[0] || extracteddata[3]>=extracteddata[0] && extracteddata[1] >= extracteddata [2]{
            points+=1;
            println!("tak");
        }
        println!("");
    }  
    println!(" {}", points);
}