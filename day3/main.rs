
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main(){
    let input = read_file();

    let mut abc: [u16; 9] = [0; 9];
    let mut count = 0;
    let mut i = 0;

    for line in input.lines(){
        for entry in line.trim().split("  "){
            match entry.trim().parse(){
                Err(_) => continue,
                Ok(n) => {
                    abc[i] = n;
                    i += 1;
                }
            }
        }

        if i == 9 {
            i = 0;
            for x in 0 .. 3 {

                let i0 = x + 0 * 3;
                let i1 = x + 1 * 3;
                let i2 = x + 2 * 3;
         
                let a = abc[i0] + abc[i1] > abc[i2];
                let b = abc[i0] + abc[i2] > abc[i1];
                let c = abc[i1] + abc[i2] > abc[i0];

                if a && b && c {
                    count += 1;
                }
            }
        }

    }

    println!("num valid triangles {}", count);

}

fn part_a(){
    let input = read_file();

    let mut abc: [u16; 3] = [0; 3];
    let mut count = 0;

    for line in input.lines(){
        let mut i = 0;
        for entry in line.trim().split("  "){
            match entry.trim().parse(){
                Err(_) => continue,
                Ok(n) => {
                    abc[i] = n;
                    i += 1;
                }

            }
        }

        let a = abc[0] + abc[1] > abc[2];
        let b = abc[0] + abc[2] > abc[1];
        let c = abc[1] + abc[2] > abc[0];

        if a && b && c {
            count += 1;
        }

    }

    println!("num valid triangles {}", count);

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
