
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Copy, Clone)]
struct Vec2 {
    x:i8,
    y:i8,
}

fn main(){
    let input = read_file();

    let mut p = Vec2{x: -2, y: 0};
    let chars = [' ', ' ', '1', ' ', ' ',
                ' ', '2', '3', '4', ' ',
                '5', '6', '7', '8', '9', 
                ' ', 'A', 'B', 'C', ' ',
                ' ', ' ', 'D', ' ', ' '];


    for c in input.chars(){
        match c {
            'U'|'D' => if c == 'U' {p.y -= 1;} else {p.y += 1;},
            'L'|'R' => if c == 'L' {p.x -= 1;} else {p.x += 1;},
            _ => {
                let i = (p.x + 2) + (p.y + 2) * 5;
                print!("{}", chars[i as usize])
                },
        }
        if p.x.abs() + p.y.abs() > 2 
        {
           match c {
            'U' => p.y +=1,
            'D' => p.y -=1,
            'L' => p.x +=1,
            'R' => p.x -=1,
            _ => println!("should not happen"),
           }
        }
    }

}

fn read_file() -> String{
    let path = Path::new("input.txt");
    let display = path.display();

    let mut file = match File::open(&path){
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s){
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(_) => s,
    }
}
