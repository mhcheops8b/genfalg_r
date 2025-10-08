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

    if args_len < 4 {
        println!("Usage: {} <size> <res_file> <file_base>", std::env::args().next().unwrap());
        return;
    }

    let mut cursize = 0usize;
    match std::env::args().nth(1).unwrap().parse() {
        Ok(val) => {cursize = val},
        Err(_e) => println!("Must be a number")
    }

    //let filename = String::from("C:/Users/mhycko/Documents/rust_genqord2/results/qord3_ord2canmax.txt");
    let res_filename = std::env::args().nth(2).unwrap();
    let base_filename = std::env::args().nth(3).unwrap();
    // let mut line_no = 0usize;
    // let mut b_first_qord = true;
    // let mut last_qord = Vec::<Vec<usize>>::new();
    // let mut cur_group_len = 0usize;
    let mut mapped = HashMap::< Vec<Vec<usize>>, HashSet< Vec<Vec<usize>> > >::new();

    if let Ok(lines_qord) = read_lines(&res_filename) {
        for line_qord in lines_qord.map_while(Result::ok) {
            // line_no+=1;
            let parsed_falg = falglib::parse_vector(cursize, &line_qord);
            
            //
            let cur_qord1 = falglib::falg_get_qord1(&parsed_falg);
            // if mapped.contains_key(&cur_qord1) {
            //     mapped.get_mut(&cur_qord1).unwrap().insert(parsed_falg);
            // }
            // else {
            //     let mut hs = HashSet::<Vec<Vec<usize>>>::new();
            //     hs.insert(parsed_falg);
            //     mapped.insert(cur_qord1, hs);
            // }

            // (mapped.entry(cur_qord1).or_insert_with(|| {
            //     let mut hs = HashSet::<Vec<Vec<usize>>>::new();
            //     hs.insert(parsed_falg.clone());
            //     hs
            //     }
            // )).insert(parsed_falg);

            // mapped.entry(cur_qord1).or_insert_with(|| {
            //     let mut hs = HashSet::<Vec<Vec<usize>>>::new();
            //     hs.insert(parsed_falg.clone());
            //     hs
            //     }
            // ).insert(parsed_falg);

            mapped.entry(cur_qord1).or_insert(HashSet::from([parsed_falg.clone()])).insert(parsed_falg);

        }
        
        let mut cur_pt = 0usize;
        let mapped_len = mapped.len();
        // get log_10 mapped_len
        let log10_mapped_len = get_log10(mapped_len);
        for v in mapped.values() {
            cur_pt += 1;
            let filename = get_filename(&base_filename, cur_pt, log10_mapped_len);                       
            {
                let mut out_file = BufWriter::new(File::create(&filename).expect("Err writing"));

                for fa in v.iter() {
                    if let Err(msg) = writeln!(&mut out_file, "{:?}", fa) {
                        eprintln!("Cannot write to {filename}. {msg}");
                    }
                }
                eprintln!("{filename} - {}", v.len());
            }
        }

        // serialize
        // let mut file_buf = BufWriter::new(File::create(&pickle_filename).expect("Cannot {pickle_filename} open for writing"));
	
	    // let all_qords_vec_serialized = serde_pickle::to_vec(&all_qords_vec, Default::default()).unwrap();
        // if let Ok(_status)  = file_buf.write(&stable_perm_vec_serialized) {
        // }
        // else {
        //     
        // }
        // if let Err(status)  = file_buf.write(&all_qords_vec_serialized) {
        //     eprintln!("Error writing a pickle file '{pickle_filename}' returning {status:?}");
        // }
    }
    else {
        eprintln!("Error opening file '{}'.", &res_filename);
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