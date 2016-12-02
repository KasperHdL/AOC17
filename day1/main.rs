
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Copy, Clone)]
struct Vec2 {
    x:i16,
    y:i16,
}


fn main(){
    let input = read_file();

    let mut p:Vec2 = Vec2{x:0,y:0};
    let mut lp:Vec2 = p;

    let mut dir = 0; //0 = n, 1 = e, 2 = s, 3 = w

    let number_trim:&[char] = &['R', 'L', ' '];
    let raw = input.split(',');

    let mut visited:Vec<Vec2> = Vec::new();

    for el in raw{
        let n = el.trim_matches(number_trim).trim();

        if el.contains('R') {
            dir += 1;
            if dir > 3{
                dir -= 4;
            }
        }else if el.contains('L') {
            dir -= 1;
            if dir < 0{
                dir += 4;
            }
        }

        //println!("{} - dir {}",n, dir);
        match n.parse(){
            Err(e) => println!("string {} = {}", n, e),
            Ok(v) => {
                match dir {
                    0 => p.y += v,
                    1 => p.x += v,
                    2 => p.y -= v,
                    3 => p.x -= v,
                    _ => println!("error"),
                }
            },
        }
        //println!("({},{})",p.x,p.y);

        if p.x == lp.x{
            for q in lp.y .. p.y{
                visited.push(Vec2{x:p.x,y:q});
            }
        }else{
            for q in lp.x .. p.x{
                visited.push(Vec2{x:q, y:p.y});
            }
        }
        

        lp = p;
    }

    let mut found = false;

    for f in 0 .. visited.len(){
        for c in f+1 .. visited.len(){
            let p1 = visited[f];
            let p2 = visited[c];
            
            //println!("checking p({},{}) and ({},{})", p1.x,p1.y,p2.x,p2.y);
            if p1.x == p2.x && p1.y == p2.y {
                println!("Found first duplicate ({},{}) = dist {}", p1.x, p1.y, p1.x.abs() + p1.y.abs());
                found = true;

                break;
            }

        }
        if found { break;}
    }


    println!("Day1 A: {}", p.x.abs() + p.y.abs());


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
