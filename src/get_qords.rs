use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use falglib;
use permlib;

fn main() {
    let args_len = std::env::args().len();

    if args_len < 3 {
        println!("Usage: {} <size> <falg_file>", std::env::args().next().unwrap());
        return;
    }
    
    let mut cursize = 0usize;
    match std::env::args().nth(1).unwrap().parse() {
        Ok(val) => {cursize = val},
        Err(_e) => println!("Must be a number")
    }
   


    //let filename = String::from("C:/Users/mhycko/Documents/rust_genqord2/results/qord3_ord2canmax.txt");
    let filename = std::env::args().nth(2).unwrap();

    if let Ok(lines_falg) = read_lines(&filename) {
        for line_falg in lines_falg.map_while(Result::ok) {
            let parsed_falg = falglib::parse_vector(cursize, &line_falg);

            println!("{:?}", falglib::falg_get_qord1(&parsed_falg));
            let qord2 = falglib::falg_get_qord2(&parsed_falg);
            println!("{:?}", qord2);
            rel_get_classes(&qord2);
            println!("---")
        }
    }
    else {
        eprintln!("Error opening file '{}'.", &filename);
    }
}

fn rel_is_equiv(qord:&Vec<Vec<usize>>, x: usize, y:usize) -> bool {
    qord[x][y] == 1 && qord[y][x]==1
}

fn rel_get_classes(qord:&Vec<Vec<usize>>) {
    let n = qord.len();

    let mut b_has_class_vec = Vec::<bool>::new();
    let mut class_vec = Vec::<usize>::new();
    for i in 0..n {
        b_has_class_vec.push(false);
        class_vec.push(0);
    }

    let mut last_class_id = 0usize;
    for x in 0..n {
        if !b_has_class_vec[x] {
            last_class_id +=1;
            b_has_class_vec[x] = true;
            class_vec[x] = last_class_id;

            for y in (x+1)..n {
                if rel_is_equiv(qord, x, y) {
                    b_has_class_vec[y] = true;
                    class_vec[y] = last_class_id;
                }
            }
        }
    }

    println!("{last_class_id}");
    for i in 1..=last_class_id {
        print!("{i}: ");
        let mut b_first = true;
        for x in 0.. n {
            if class_vec[x] == i {
                if b_first {
                    print!("{x}");
                    b_first = false;
                }
                else {
                    print!(", {x}")
                }
            }
        }
        println!();
    }
//     let cl_id = 0;
}




// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: &P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}