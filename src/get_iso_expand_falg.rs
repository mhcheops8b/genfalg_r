use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;
// use std::time::{Duration, Instant};

fn main() {
    let args_len = std::env::args().len();

    if args_len < 3 {
        println!("Usage: {} <size> <falg_file> [from_line [to_line]]", std::env::args().next().unwrap());
        return;
    }

    let mut cursize = 0usize;
    match std::env::args().nth(1).unwrap().parse() {
        Ok(val) => {cursize = val},
        Err(_e) => println!("Must be a number")
    }

    //let filename = String::from("C:/Users/mhycko/Documents/rust_genqord2/results/qord3_ord2canmax.txt");
    let filename = std::env::args().nth(2).unwrap();

    let mut from_line = 1usize;
    if args_len >= 4 {	
	    match std::env::args().nth(3).unwrap().parse() {
		Ok(val) => {from_line = val},
		Err(_e) => println!("Must be a number")
	    }
    }

    let mut to_line = 0usize;
    if args_len >= 5 {	
	    match std::env::args().nth(4).unwrap().parse() {
		Ok(val) => {to_line = val},
		Err(_e) => println!("Must be a number")
	    }
    }


    //let mut parsed_falg:Vec<Vec<usize>> = Vec::new();
    let mut b_read = false;
    //let mut red_qords = Vec::<Vec<Vec<usize>>>::new();
    let mut cur_line = 0usize;
    if let Ok(lines_qord) = read_lines(&filename) {
        for line_qord in lines_qord.map_while(Result::ok) {
		cur_line+=1;
		if cur_line >= from_line && (to_line == 0 || cur_line<=to_line) {
			let parsed_falg = falglib::parse_vector(cursize, &line_qord);
			let iso_exp_size = falglib::falg_isomorphic_expand_just_algs(&parsed_falg).len();
			println!("{}\t{}", cur_line, iso_exp_size);
		}
        }
    }
    else {
        println!("Error opening file '{}'.", &filename);
    }
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: &P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
