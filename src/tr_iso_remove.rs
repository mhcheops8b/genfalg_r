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

    let mut all_falg: HashSet<Vec<Vec<usize>>> = HashSet::new();
    if let Ok(lines_falg) = read_lines(&filename) {
        for line_falg in lines_falg.map_while(Result::ok) {
            let parsed_falg = falglib::parse_vector(cursize, &line_falg);
            if falglib::falg_all_tests_no_print(&parsed_falg) {
                all_falg.insert(falglib::falg_transpose(&parsed_falg));
            }
            
        }
    }
    else {
        eprintln!("Error opening file '{}'.", &filename);
    }
    eprintln!("{}", all_falg.len());


    // for cur_falg in all_falg {
    //     println!("{:?}", falglib::falg_find_min_repr(&cur_falg));
    // }

    loop {
        if all_falg.is_empty() {
            break;
        }
        let cur_falg = all_falg.iter().next().cloned().unwrap();
        println!("{:?}", falglib::falg_find_min_repr(&cur_falg));
        all_falg.take(&cur_falg).unwrap();
        let mut perm:Vec<usize> = (0..cursize).collect();
        loop {
            let cur_falg_iso = falglib::falg_isomorphic_image(&cur_falg, &perm);

            if all_falg.contains(&cur_falg_iso) {
                all_falg.remove(&cur_falg_iso);    
            }
            
            if !permlib::next_perm(&mut perm, cursize) {
                break;
            }
        }
    }


}



// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: &P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}