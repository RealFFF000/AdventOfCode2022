use std::fs;
use std::io;

fn main() {
    let mut enemy = 0;
    let mut me = 0;
    let mut points = 0;
    let file_contents = fs::read_to_string("../input.txt")
        .expect("nie znaleziono pliku");
    for line in file_contents.lines() {
        me = 0;
        enemy = 0;
        for character in line.chars() {
            if character != ' ' {
                if character == 'A' {
                    enemy = 1;
                }else if character == 'B' {
                    enemy = 2;
                }else if character == 'C' {
                    enemy = 3;
                }
                
                if character == 'X' {
                    me = 1;
                }else if character == 'Y' {
                    me = 2;
                }else if character == 'Z' {
                    me = 3;
                }

                
            }
        }
        println!("{}", line);

        if me == 1{
            if enemy == 1{
                me = 3;
            }else if enemy == 2{
                me = 1;
            }else if enemy == 3{
                me = 2;
            }
        }else if me == 2{
            points +=3;
            me = enemy;
        }else if me == 3{
            points +=6;
            if enemy == 1{
                me = 2;
            }else if enemy == 2{
                me = 3;
            }else if enemy == 3{
                me = 1;
            }
        }
        points+=me;
    }  
    println!(" {}", points);
}