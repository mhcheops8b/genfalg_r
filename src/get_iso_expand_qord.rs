use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;
// use std::time::{Duration, Instant};

fn main() {
    let args_len = std::env::args().len();

    if args_len < 5 {
        println!("Usage: {} <size> <rel_file> <ord_idx> <exp_iso_idx>", std::env::args().next().unwrap());
        return;
    }

    let mut cursize = 0usize;
    match std::env::args().nth(1).unwrap().parse() {
        Ok(val) => {cursize = val},
        Err(_e) => println!("Must be a number")
    }

    //let filename = String::from("C:/Users/mhycko/Documents/rust_genqord2/results/qord3_ord2canmax.txt");
    let filename = std::env::args().nth(2).unwrap();

    let mut curord_idx = 0usize;
    match std::env::args().nth(3).unwrap().parse() {
        Ok(val) => {curord_idx = val},
        Err(_e) => println!("Must be a number")
    }

    let mut isoexp_idx = 0usize;
    match std::env::args().nth(4).unwrap().parse() {
        Ok(val) => {isoexp_idx = val},
        Err(_e) => println!("Must be a number")
    }
    let mut cur_line = 0usize;
    let mut parsed_qord:Vec<Vec<usize>> = Vec::new();
    let mut b_read = false;
    //let mut red_qords = Vec::<Vec<Vec<usize>>>::new();
    if let Ok(lines_qord) = read_lines(&filename) {
        for line_qord in lines_qord.map_while(Result::ok) {
            cur_line+=1;
            if cur_line == curord_idx {
                parsed_qord = falglib::parse_vector(cursize, &line_qord);
                b_read = true;
                break;
            }
        }
    }
    else {
        println!("Error opening file '{}'.", &filename);
    }

    if b_read {
        let qord2_iso_exp = falglib::rel_isomorphic_expand_vec(&parsed_qord).0;
        let qord2_iso_exp_len = qord2_iso_exp.len();
        // for qord in qord2_iso_exp {
        //     println!("{:?}",qord);
        // }
        eprintln!("{qord2_iso_exp_len}");
        if isoexp_idx < 1 || isoexp_idx > qord2_iso_exp_len {
            eprintln!("Out of index, 1 <= iso_exp_idx <= {qord2_iso_exp_len}");
        }
        else {
            eprintln!("{isoexp_idx}");
            println!("{:?}", qord2_iso_exp[isoexp_idx - 1]);
            rel_get_classes(&qord2_iso_exp[isoexp_idx - 1]);
        }
    }
    else {
        eprintln!("Quasi-order index {curord_idx} exceeds the number of records in qords_file.")
    }
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
    for _i in 0..n {
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
