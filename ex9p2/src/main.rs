use std::fs;
use std::string::String;
use std::vec;
use std::array;
use std::fs::File;

fn main() {
    let file_contents = fs::read_to_string("../input.txt")
        .expect("nie znaleziono pliku");

    //dont judge me i am doing it for the 4th hour now
    let mut headpos = [0,0];
    let mut ftailpos = [0,0];
    let mut stailpos = [0,0];
    let mut ttailpos = [0,0];
    let mut fotailpos = [0,0];
    let mut fitailpos = [0,0];
    let mut sitailpos = [0,0];
    let mut sittailpos = [0,0];
    let mut setailpos = [0,0];
    let mut eitaipos = [0,0];
    let mut ntailpos = [0,0];
    let mut tentailpos = [0,0];

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
            if ftailpos[0] > headpos[0]+length || ftailpos[0] < headpos[0]-length || ftailpos[1] > headpos[1]+length || ftailpos[1] < headpos[1]-length{
                ftailpos[0] = headpos[0]-direction[0];
                ftailpos[1] = headpos[1]-direction[1];   
            }
            if stailpos[0] > ftailpos[0]+length || stailpos[0] < ftailpos[0]-length || stailpos[1] > ftailpos[1]+length || stailpos[1] < ftailpos[1]-length{
                stailpos[0] = ftailpos[0]-direction[0];
                stailpos[1] = ftailpos[1]-direction[1];   
            }
            if ttailpos[0] > stailpos[0]+length || ttailpos[0] < stailpos[0]-length || ttailpos[1] > stailpos[1]+length || ttailpos[1] < stailpos[1]-length{
                ttailpos[0] = stailpos[0]-direction[0];
                ttailpos[1] = stailpos[1]-direction[1];   
            }
            if fotailpos[0] > ttailpos[0]+length || fotailpos[0] < ttailpos[0]-length || fotailpos[1] > ttailpos[1]+length || fotailpos[1] < ttailpos[1]-length{
                fotailpos[0] = ttailpos[0]-direction[0];
                fotailpos[1] = ttailpos[1]-direction[1];   
            }
            if fitailpos[0] > fotailpos[0]+length || fitailpos[0] < fotailpos[0]-length || fitailpos[1] > fotailpos[1]+length || fitailpos[1] < fotailpos[1]-length{
                fitailpos[0] = fotailpos[0]-direction[0];
                fitailpos[1] = fotailpos[1]-direction[1];   
            }
            if sitailpos[0] > fitailpos[0]+length || sitailpos[0] < fitailpos[0]-length || sitailpos[1] > fitailpos[1]+length || sitailpos[1] < fitailpos[1]-length{
                sitailpos[0] = fitailpos[0]-direction[0];
                sitailpos[1] = fitailpos[1]-direction[1];   
            }
            if sittailpos[0] > sitailpos[0]+length || sittailpos[0] < sitailpos[0]-length || sittailpos[1] > sitailpos[1]+length || sittailpos[1] < sitailpos[1]-length{
                sittailpos[0] = sitailpos[0]-direction[0];
                sittailpos[1] = sitailpos[1]-direction[1];   
            }
            if setailpos[0] > sittailpos[0]+length || setailpos[0] < sittailpos[0]-length || setailpos[1] > sittailpos[1]+length || setailpos[1] < sittailpos[1]-length{
                setailpos[0] = sittailpos[0]-direction[0];
                setailpos[1] = sittailpos[1]-direction[1];   
            }
            if eitaipos[0] > setailpos[0]+length || eitaipos[0] < setailpos[0]-length || eitaipos[1] > setailpos[1]+length || eitaipos[1] < setailpos[1]-length{
                eitaipos[0] = setailpos[0]-direction[0];
                eitaipos[1] = setailpos[1]-direction[1];   
            }
            if ntailpos[0] > eitaipos[0]+length || ntailpos[0] < eitaipos[0]-length || ntailpos[1] > eitaipos[1]+length || ntailpos[1] < eitaipos[1]-length{
                ntailpos[0] = eitaipos[0]-direction[0];
                ntailpos[1] = eitaipos[1]-direction[1];   
            }
            if tentailpos[0] > ntailpos[0]+length || tentailpos[0] < ntailpos[0]-length || tentailpos[1] > ntailpos[1]+length || tentailpos[1] < ntailpos[1]-length{
                tentailpos[0] = ntailpos[0]-direction[0];
                tentailpos[1] = ntailpos[1]-direction[1];   
            }

            if tailused.contains(&tentailpos){
                print!("pominieto [{},{}] \n", tentailpos[0], tentailpos[1]);
            }else{
                tailused.push([tentailpos[0],tentailpos[1]]);
            }
        }
        
    }
    print!("{} ", length);
    print!("{} ", tailused.len());
}
