use std::collections::{HashMap, HashSet};
//use std::fmt::Write;
use std::fs::File;
use std::io::{self, BufRead, BufWriter, Write};
use std::path::Path;
// use std::time::Instant;
// use serde::{Serialize, Deserialize};

// use std::time::{Duration, Instant};

// use falglib;

fn main() {
    let args_len = std::env::args().len();

    if args_len < 5 {
        println!("Usage: {} <size> <falg_res_file> <falg_already_processed_file> <file_base> <base_index>", std::env::args().next().unwrap());
        return;
    }

    let mut cursize = 0usize;
    match std::env::args().nth(1).unwrap().parse() {
        Ok(val) => {cursize = val},
        Err(_e) => println!("Must be a number")
    }

    //let filename = String::from("C:/Users/mhycko/Documents/rust_genqord2/results/qord3_ord2canmax.txt");
    let res_filename = std::env::args().nth(2).unwrap();
    let alr_filename = std::env::args().nth(3).unwrap();
    let base_filename = std::env::args().nth(4).unwrap();

    let mut pairord_idx = 0usize;
    match std::env::args().nth(5).unwrap().parse() {
        Ok(val) => {pairord_idx = val},
        Err(_e) => println!("Must be a number")
    }

    // let mut line_no = 0usize;
    // let mut b_first_qord = true;
    // let mut last_qord = Vec::<Vec<usize>>::new();
    // let mut cur_group_len = 0usize;
    let mut mapped = HashMap::< Vec<Vec<usize>>, HashSet< Vec<Vec<usize>> > >::new();

    // Load results into memory

    let mut all_res = Vec::<Vec<Vec<usize>>>::new();
    let mut b_processed = Vec::<bool>::new();
    if let Ok(lines_qord) = read_lines(&res_filename) {
        for line_qord in lines_qord.map_while(Result::ok) {
            // line_no+=1;
            let parsed_falg = falglib::parse_vector(cursize, &line_qord);
            all_res.push(parsed_falg);
            b_processed.push(false);
        }
    }
    else {
        eprintln!("Error opening file '{}'.", &res_filename);
        return;
    }

    let n = all_res.len();
    eprintln!("Loaded: {}", n);

    let mut marked_cnt = 0usize;
    if let Ok(lines_qord) = read_lines(&alr_filename) {
        for line_qord in lines_qord.map_while(Result::ok) {
            // line_no+=1;
            let parsed_falg = falglib::parse_vector(cursize, &line_qord);

            if let Some(idx) = all_res.iter().position(|n| n==&parsed_falg) {
                b_processed[idx] = true;
                marked_cnt +=1;
                // eprintln!(" marked idx {}", idx);

            }
            else {
                eprintln!("Problem idx does not exist");
                return;
            }
        }
    }
    else {
        eprintln!("Error opening file '{}'.", &res_filename);
        return;
    }

    eprintln!("Marked: {}", marked_cnt);
    
    for i in 0..n {
        
        if b_processed[i] {
            continue;
        }
        eprintln!("Processing {}", i);
        b_processed[i] = true;
        let mut filename = String::from(&base_filename);
        filename.push_str("_pair-");
        filename.push_str(&format!("{}", pairord_idx));
        filename.push_str(".txt");


        let mut out_file = BufWriter::new(File::create(&filename).expect("Err writing"));
        let cur_qord1 = falglib::falg_get_qord1(&all_res[i]);
        let cur_qord2 = falglib::falg_get_qord2(&all_res[i]);
        let _ = write!(&mut out_file, "# qord1: ");
        let _ = writeln!(&mut out_file, "{:?}", &cur_qord1);
        let _ = write!(&mut out_file, "# qord2: ");
        let _ = writeln!(&mut out_file, "{:?}", &cur_qord2);
        let _ = writeln!(&mut out_file, "{:?}", &&all_res[i]);

        for j in (i+1)..n {
            if b_processed[j] {
                continue;
            }

            let jth_cur_qord1 = falglib::falg_get_qord1(&all_res[j]);
            if cur_qord1 != jth_cur_qord1 {
                continue;
            }
            let jth_cur_qord2 = falglib::falg_get_qord2(&all_res[j]);
            if cur_qord2 != jth_cur_qord2 {
                continue;
            }
            let _ = writeln!(&mut out_file, "{:?}", &&all_res[j]);
            b_processed[j] = true;
        }
        pairord_idx+=1;
    }

}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: &P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_filename(basename: &String, cur_pt: usize, log_len: usize) -> String {
    let mut res_filename = String::from(basename);

    res_filename.push_str("_pt");
    let nec_zero = log_len - get_log10(cur_pt);
    for _ in 0..nec_zero {
        res_filename.push('0');
    }
    res_filename.push_str(format!("{cur_pt}").as_str());
    res_filename.push_str(".txt");

    res_filename
}

fn get_log10(size: usize) -> usize {
    if size<=9 {
        return 1;
    }
    else if size >=10 && size <=99 {
        return 2;
    }
    else if size >=100 && size <=999 {
        return 3;
                
    }
    else if size >=1000 && size <=9999 {
        return 4;
    }
    5
}