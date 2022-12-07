use std::fs;
use std::string::String;
use std::vec;
use std::array;
use std::fs::File;

fn main() {
    let file_contents = fs::read_to_string("../input.txt")
        .expect("nie znaleziono pliku");
    let mut pwd: Vec<String> =  vec![];
    let mut pwdata: Vec<i32> = vec![];
    let mut lsmode = false;
    let mut suma = 0;
    for line in file_contents.lines(){
        

        if lsmode{
            if line.contains("$"){
                lsmode = false; 
            }
            
            //I guess because empty folders are not taking space listing directories is useless  
            else if line.contains("dir"){
                let mut lin: String = String::from(line);
                for a in 0..5{
                    lin.remove(0);
                }
                //println!("{} - this is a directory",lin);
            }
            else if line == ""{
                println!("{}",suma);
            }else{
                
                for i in &pwd{
                    print!("{}",i);
                    print!("/");
                }

                let mut lin: String = String::from(line);
                let mut filename: String = String::from("");
                let mut size: i32 = 0;
                let mut len: String = String::from("");
                for w in lin.chars(){
                    if w!='0' && w != '1' && w != '2' && w != '3' && w != '4' && w != '5' && w != '6' && w != '7' && w != '8' && w != '9' && w != ' '{
                        filename.push(w);
                    }else{
                        if w != ' '{
                            len.push(w);
                        }
                    }
                }
                size = len.parse::<i32>().unwrap();
                print!("{}", filename);
                print!(" {}", size);
                print!("  ");

                let mut counter = 0;
                for i in &pwd{
                    pwdata[counter] += size; 
                    counter+=1
                }

                for i in &pwdata{
                    print!("{}",i);
                    print!("/");
                }
                
                print!("\n")
            }
        }

        if lsmode == false{
            if line.contains("$ ls"){
                //println!("{}", line);
                lsmode = true;
            }
            if line.contains("$ cd"){
                
                let mut lin: String = String::from(line);
                for a in 0..5{
                    lin.remove(0);
                }
                if lin == ".."{
                    println!("{} {}", line, pwdata[(pwdata.len()-1)]);

                    if pwdata[(pwdata.len()-1)] <= 100000{
                        suma+=pwdata[(pwdata.len()-1)];
                    }
                    pwd.remove((pwd.len()-1) as usize);
                    pwdata.remove((pwdata.len()-1) as usize);
                }else{
                    pwd.push(lin);
                    pwdata.push(0);
                }
                
                
                
                //println!("{}", lin);
            }
        }

    }
    println!("{}",suma);
}
