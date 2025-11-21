use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufWriter};
use std::path::Path;
use std::time::Instant;
use std::fmt::Write as FW;
use std::io::Write as OW;
// use std::time::{Duration, Instant};

use falglib;

fn main() {
//     let mut cursize = 0usize;
//     match std::env::args().nth(1).unwrap().parse() {
//         Ok(val) => {cursize = val},
//         Err(_e) => println!("Must be a number")
//     }

// //    let n = 5;
//     let mut jj = String::new();
//     //let fmt=format!("{{:0{n}}}");
//     //println!("{}", fmt);
//     write!(&mut jj, "{:0cursize$}", 120).unwrap();
//     println!("{}", jj);
//     return;

    let args_len = std::env::args().len();

    if args_len < 4 {
        println!("Usage: {} <size> <rel_file> <qord_num>", std::env::args().next().unwrap());
        return;
    }

    let mut cursize = 0usize;
    match std::env::args().nth(1).unwrap().parse() {
        Ok(val) => {cursize = val},
        Err(_e) => println!("Must be a number")
    }

    //let filename = String::from("C:/Users/mhycko/Documents/rust_genqord2/results/qord3_ord2canmax.txt");
    let filename = std::env::args().nth(2).unwrap();

    let mut qordnum = 1usize;
    match std::env::args().nth(3).unwrap().parse() {
        Ok(val) => {qordnum = val},
        Err(_e) => println!("Must be a number")
    }

    // get qornum qord from file
    //let mut red_qords = Vec::<Vec<Vec<usize>>>::new();
    let mut curqord:Vec<Vec<usize>> = Vec::new();
    let mut curord_idx = 0usize;
    if let Ok(lines_qord) = read_lines(&filename) {
        for line_qord in lines_qord.map_while(Result::ok) {
            curord_idx+=1;
            if curord_idx == qordnum {
                curqord = falglib::parse_vector(cursize, &line_qord);
            }
        }
    }
    else {
        println!("Error opening file '{}'.", &filename);
    }
    
    println!("{:?}", &curqord);
    rel_get_classes(&curqord);
}

fn tst1(already_checked_set: &HashSet<Vec<Vec<usize>>>, rel_qord1: &Vec<Vec<usize>>, cur_rel_qord2: &Vec<Vec<usize>>) -> bool {
    for perm in falglib::rel_get_stabilizer_perms(&rel_qord1) {
        let iso_qord2 = falglib::rel_isomorphic_image(&cur_rel_qord2, &perm);
        if already_checked_set.contains(&iso_qord2) {
            return true;
        }
    }
    false
}


// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: &P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
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

    println!("--Cls_Less--");
    for i in 1..=last_class_id {
        let mut repr_i = 0usize;
        for x in 0..n {
            if class_vec[x]==i {
                repr_i = x;
                break;
            }
        }
        print!("{i}: ");
        let mut b_first = true;
        for j in 1..=last_class_id {
            let mut repr_j = 0usize;
            for x in 0..n {
                if class_vec[x]==j {
                    repr_j = x;
                    break;
                }
            }
            if qord[repr_i][repr_j] == 1 {
                if !b_first {
                    print!(", ");
                }
                else {
                    b_first = false;
                }
                print!("{j}")
            }
        }
        println!();
    }


//     let cl_id = 0;
}