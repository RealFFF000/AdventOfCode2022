use std::fs;
use std::io;

fn main() {
    let mut points = 0;
    let mut checked = false;
    let mut lineone = String::new();
    let mut linetwo = String::new();
    let mut linethree = String::new();
    let mut counter = 0;
    let file_contents = fs::read_to_string("../input.txt")
        .expect("nie znaleziono pliku");
    for line in file_contents.lines() {                                
        lineone = linetwo;
        linetwo = linethree;
        linethree = line.to_string();
        counter+=1;
        if counter == 3{
            checked = false;
            counter = 0;
            println!("{}\n{}\n{}", lineone,linetwo,linethree);

            for characterone in lineone.chars() {
                for charactertwo in linetwo.chars(){
                    for characterthree in linethree.chars(){
                        if checked == false{
                            if characterone == charactertwo && charactertwo == characterthree{
                                let mut thechar = characterone;
                                print!("{}\n\n", thechar);
                                checked = true;
                                match thechar{
                                    'a'=>{points+=1; break;},
                                    'b'=>{points+=2; break;},
                                    'c'=>{points+=3; break;},
                                    'd'=>{points+=4; break;},
                                    'e'=>{points+=5; break;},
                                    'f'=>{points+=6; break;},
                                    'g'=>{points+=7; break;},
                                    'h'=>{points+=8; break;},
                                    'i'=>{points+=9; break;},
                                    'j'=>{points+=10; break;},
                                    'k'=>{points+=11; break;},
                                    'l'=>{points+=12; break;},
                                    'm'=>{points+=13; break;},
                                    'n'=>{points+=14; break;},
                                    'o'=>{points+=15; break;},
                                    'p'=>{points+=16; break;},
                                    'q'=>{points+=17; break;},
                                    'r'=>{points+=18; break;},
                                    's'=>{points+=19; break;},
                                    't'=>{points+=20; break;},
                                    'u'=>{points+=21; break;},
                                    'v'=>{points+=22; break;},
                                    'w'=>{points+=23; break;},
                                    'x'=>{points+=24; break;},
                                    'y'=>{points+=25; break;},
                                    'z'=>{points+=26; break;},
                                    'A'=>{points+=27; break;},
                                    'B'=>{points+=28; break;},
                                    'C'=>{points+=29; break;},
                                    'D'=>{points+=30; break;},
                                    'E'=>{points+=31; break;},
                                    'F'=>{points+=32; break;},
                                    'G'=>{points+=33; break;},
                                    'H'=>{points+=34; break;},
                                    'I'=>{points+=35; break;},
                                    'J'=>{points+=36; break;},
                                    'K'=>{points+=37; break;},
                                    'L'=>{points+=38; break;},
                                    'M'=>{points+=39; break;},
                                    'N'=>{points+=40; break;},
                                    'O'=>{points+=41; break;},
                                    'P'=>{points+=42; break;},
                                    'Q'=>{points+=43; break;},
                                    'R'=>{points+=44; break;},
                                    'S'=>{points+=45; break;},
                                    'T'=>{points+=46; break;},
                                    'U'=>{points+=47; break;},
                                    'V'=>{points+=48; break;},
                                    'W'=>{points+=49; break;},
                                    'X'=>{points+=50; break;},
                                    'Y'=>{points+=51; break;},
                                    'Z'=>{points+=52; break;},
                                    _=>println!(""),
                                    
                                }
                                
                            }
                        }

                    }
                }
            }

        }
        
        
    }  
    println!("{}",points);
}