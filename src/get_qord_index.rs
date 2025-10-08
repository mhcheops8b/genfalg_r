use std::collections::{HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args_len = std::env::args().len();

    if args_len < 4 {
        println!("Usage: {} <size> <all_ni_qords_file> <falgs_file>", std::env::args().next().unwrap());
        return;
    }

    let mut cursize = 0usize;
    match std::env::args().nth(1).unwrap().parse() {
        Ok(val) => {cursize = val},
        Err(_e) => println!("Must be a number")
    }

    //let filename = String::from("C:/Users/mhycko/Documents/rust_genqord2/results/qord3_ord2canmax.txt");
    let all_ni_qords_filename = std::env::args().nth(2).unwrap();
    let falgs_filename = std::env::args().nth(3).unwrap();

    // load all ni_qords
    let mut all_ni_qords = Vec::<Vec<Vec<usize>>>::new();
    if let Ok(lines_all_ni_qords) = read_lines(&all_ni_qords_filename) {
        for line_qord in lines_all_ni_qords.map_while(Result::ok) {
            all_ni_qords.push(falglib::parse_vector(cursize, &line_qord));
        }
    }
    else {
        println!("Error opening res file '{}'.", &all_ni_qords_filename);
    }
    let all_ni_qords_size = all_ni_qords.len();
    eprintln!("{}", all_ni_qords_size);
    
    if let Ok(lines_falgs) = read_lines(&falgs_filename) {
        for line_falg in lines_falgs.map_while(Result::ok) {
            let parsed_falg = falglib::parse_vector(cursize, &line_falg);
            let qord1_repr = falglib::rel_quasi_order_find_max_repr(&falglib::falg_get_qord1(&parsed_falg));
            let qord2_repr = falglib::rel_quasi_order_find_max_repr(&falglib::falg_get_qord2(&parsed_falg));
            // let qord1_repr = falglib::rel_quasi_order_find_can_max_repr(&qord1);
            // let qord2_repr = falglib::rel_quasi_order_find_max_repr(&qord2);
            // println!("{qord2_repr:?}");
            // println!("{}", falglib::rel_is_reflexive(&qord2_repr));
            // println!("{}", falglib::rel_is_antisymmetric(&qord2_repr));
            // println!("{}", falglib::rel_is_transitive(&qord2_repr));
            let pos1 = all_ni_qords.iter().position(|v| *v == qord1_repr);
            let pos2 = all_ni_qords.iter().position(|v| *v == qord2_repr);
            println!("pos1: {pos1:?}, pos2: {pos2:?}");
        }
    }
    else {
        println!("Error opening res file '{}'.", &falgs_filename);
    }
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: &P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
