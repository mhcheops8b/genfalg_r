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
    let args_len = std::env::args().len();

    if args_len < 3 {
        println!("Usage: {} <size> <rel_file> [from [to]]", std::env::args().next().unwrap());
        return;
    }

    let mut cursize = 0usize;
    match std::env::args().nth(1).unwrap().parse() {
        Ok(val) => {cursize = val},
        Err(_e) => println!("Must be a number")
    }

    //let filename = String::from("C:/Users/mhycko/Documents/rust_genqord2/results/qord3_ord2canmax.txt");
    let filename = std::env::args().nth(2).unwrap();


    let mut b_has_from = false;
    let mut n_from = 0usize;
    if args_len >= 4 {
        // only from 
        match std::env::args().nth(3).unwrap().parse() {
            Ok(val) => {n_from = val},
            Err(_e) => println!("Must be a number")
        }
        b_has_from = true;
    }
    let mut b_has_to = false;
    let mut n_to = 0usize;

    if args_len == 5 {
        // from to
        match std::env::args().nth(4).unwrap().parse() {
            Ok(val) => {n_to = val},
            Err(_e) => println!("Must be a number")
        }
        b_has_to = true;
    }

    // load all reduced qords
    let mut red_qords = Vec::<Vec<Vec<usize>>>::new();
    if let Ok(lines_qord) = read_lines(&filename) {
        for line_qord in lines_qord.map_while(Result::ok) {
            red_qords.push(falglib::parse_vector(cursize, &line_qord));
        }
    }
    else {
        println!("Error opening file '{}'.", &filename);
    }
    eprintln!("{}", red_qords.len());
    

    // Výpočet:
    //  1. redukovaného, páry ( qo_i, iso_exp(qo_j) ) pre j>=i
    //  2. iso_exp sa volá pre každé j iba raz

    let mut c_from = 1usize;
    let mut c_to = 1usize;
    if b_has_from {
        c_from = n_from;
    }

    if b_has_to {
        c_to = n_to;
    }
    else {
        c_to = red_qords.len();
    }

    for qord2_idx in c_from-1..=c_to-1 {
        let mut log_filename = String::new();
        let mut out_filename = String::new();
        write!(&mut log_filename, "redfalg9-{:06}-v7.log", qord2_idx + 1).unwrap();
        write!(&mut out_filename, "redfalg9-{:06}-v7.txt", qord2_idx + 1).unwrap();
        // eprintln!("Log filename: {}", &log_filename);
        
        let out_pth = Path::new(&out_filename);
        if out_pth.exists() {
            eprintln!("Output file {out_filename} already exists.");
            eprintln!("Skipping qord2_idx {}", qord2_idx+1);
            continue;
        }

        let out_pth = Path::new(&log_filename);
        if out_pth.exists() {
            eprintln!("Output log file {log_filename} already exists.");
            eprintln!("Skipping qord2_idx {}", qord2_idx+1);
            continue;
        }

        let result = File::create(&log_filename);
        let log_file:File;
        match result {
            Ok(rrr) => {log_file = rrr},
            Err(err) => { eprintln!("Cannot open {} for writting.\nError message: {}", &log_filename, err);
            return;}
        }
        let mut log_writer = BufWriter::new(&log_file);

        let result = File::create(&out_filename);
        let out_file:File;
        match result {
            Ok(rrr) => {out_file = rrr},
            Err(err) => { eprintln!("Cannot open {} for writting.\nError message: {}", &out_filename, err);
            return;}
        }
        let mut out_writer = BufWriter::new(&out_file);

        eprintln!("Computing qord2_idx {}", qord2_idx + 1);
        let time_iter_start = Instant::now();
        let iso_expand_full_size = falglib::rel_isomorphic_expand_full_size(&red_qords[qord2_idx]);            
        let qord2_iso_exp = falglib::rel_isomorphic_expand_reduced_vec(&red_qords[qord2_idx]).0;
        let qord2_iso_exp_len = qord2_iso_exp.len();
        let min_count = falglib::rel_count_strict_minimal_elements(&red_qords[qord2_idx]);
        if min_count >=2 {
            // log_writer.write_fmt(format_args!("Line: {} - {qord2_iso_exp_len} - Skipping\n", qord2_idx+1)).unwrap();
            // log_writer.write_fmt(format_args!("{}\t{}\t{}\t{}\t{}\t{:.4}", qord2_idx, qord2_iso_exp_len, 0, 0, iso_expand_full_size - qord2_iso_exp_len, 0.00)).unwrap();
            //log_writer.flush().unwrap();   
            // writeln!();
            writeln!(&mut log_writer, "Line: {} - {qord2_iso_exp_len} - Skipping", qord2_idx+1).unwrap();
            writeln!(&mut log_writer, "{}\t{}\t{}\t{}\t{}\t{:.4}", qord2_idx, qord2_iso_exp_len, 0, 0, iso_expand_full_size - qord2_iso_exp_len, 0.00).unwrap();
            // eprintln!("Line: {} - {qord2_iso_exp_len} - Skipping", qord2_idx);   
            // eprintln!("{}\t{}\t{}\t{}\t{}\t{:.4}", qord2_idx, qord2_iso_exp_len, 0, 0, iso_expand_full_size - qord2_iso_exp_len, 0.00);
            continue; 
        }

        writeln!(&mut log_writer, "Line: {} - {qord2_iso_exp_len}", qord2_idx + 1).unwrap();
        log_writer.flush().unwrap();            
        //eprintln!("Line: {} - {qord2_iso_exp_len}", qord2_idx + 1);
        let mut num_compat = 0usize;
        let mut num_with_cands = 0usize;
        let mut cur_perm_cnt = 0usize;
        for qord2 in qord2_iso_exp {
                cur_perm_cnt += 1;
                if cur_perm_cnt % 100 == 1 {
                    writeln!(&mut log_writer, "\t- Cur perm: {cur_perm_cnt} / {qord2_iso_exp_len} - {num_compat} - {num_with_cands} - {:.4}", time_iter_start.elapsed().as_secs_f64()).unwrap();
                    log_writer.flush().unwrap();
                    //eprintln!("\t- Cur perm: {cur_perm_cnt} / {qord2_iso_exp_len} - {num_compat} - {num_with_cands} - {:.4}", time_iter_start.elapsed().as_secs_f64());    
                }
                for qord1_idx in 0..=qord2_idx { 
                        
                    if falglib::rel_are_pair_antisymmetric(&red_qords[qord1_idx], &qord2) {
                        num_compat+=1;
                        if falglib::rel_pair_has_all_candidates_check(&red_qords[qord1_idx], &qord2) {
                            num_with_cands+=1;
                            falglib::falg_generate_with_qords_writer(out_writer.get_mut(), &red_qords[qord1_idx], &qord2);
                            out_writer.flush().unwrap();
                        }                            
                    }


                        
                        // for perm in falglib::rel_get_stabilizer_perms(&parsed_ord1) {
                        //     already_checked_set.insert(falglib::rel_isomorphic_image(&qord2, &perm));
                        // }
                        // }
                        // else {
                        //     num_skipped+=1;
                        // }

                }
                    // already_checked_set.insert(qord2);
                    // if cur_perm_cnt % 500 == 1 {
                    //     eprintln!("Skipped count: {}", num_skipped);
                    // }

        }
        writeln!(&mut log_writer, "{}\t{}\t{}\t{}\t{}\t{:.4}", qord2_idx+1, iso_expand_full_size, num_compat, num_with_cands, iso_expand_full_size - qord2_iso_exp_len, time_iter_start.elapsed().as_secs_f64()).unwrap();
        //eprintln!("{}\t{}\t{}\t{}\t{}\t{:.4}", qord2_idx+1, iso_expand_full_size, num_compat, num_with_cands, iso_expand_full_size - qord2_iso_exp_len, time_iter_start.elapsed().as_secs_f64());
    }
    
       

    

    // let mut line2_idx = 0usize;
    // if let Ok(lines_qord2) = read_lines(&filename) {
    //     for line_qord2 in lines_qord2.map_while(Result::ok) {
    //         //println!("{}", &line);
    //         line2_idx +=1;
    //         if b_has_from && line2_idx < n_from {
    //             continue;
    //         }

    //         if b_has_to && line2_idx > n_to {
    //             continue;
    //         }
            
    //         //eprintln!("Line: {line2_idx}");
    //         let time_iter_start = Instant::now();
    //         let parsed_ord2 = falglib::parse_vector(cursize, &line_qord2);
            
    //         let qord2_iso_exp = falglib::rel_isomorphic_expand(&parsed_ord2).0;
    //         let qord2_iso_exp_len = qord2_iso_exp.len();
    //         eprintln!("Line: {line2_idx} - {qord2_iso_exp_len}");
    //         let mut num_compat = 0usize;
    //         // let mut cur_perm_cnt = 0usize;
    //         // let mut num_skipped = 0usize;
    //         //let mut map_already_checked =HashMap::< Vec<Vec<usize>>, HashSet< Vec<Vec<usize>> >>::new();
    //         // let mut already_checked_set = HashSet::<Vec<Vec<usize>>>::new();
    //         for qord2 in qord2_iso_exp {
    //             // cur_perm_cnt+=1;
    //             // if cur_perm_cnt % 500 == 1 {
    //             //     eprintln!("Cur perm: {cur_perm_cnt} / {qord2_iso_exp_len}");    
    //             // }
    //             let mut line1_idx = 0usize;
    //             if let Ok(lines_qord1) = read_lines(&filename) {
    //                 for line_qord1 in lines_qord1.map_while(Result::ok) {
    //                     line1_idx+=1;
    //                     if line1_idx > line2_idx {
    //                         break;
    //                     }
    //                     let parsed_ord1 = falglib::parse_vector(cursize, &line_qord1);
    //                     // let mut already_checked_set = HashSet::<Vec<Vec<usize>>>::new();

    //                     // if false {
    //                     //     if falglib::rel_are_pair_antisymmetric(&parsed_ord1, &qord2) {
    //                     //         num_compat+=1;
    //                     //         if already_checked_set.is_empty() || !tst1(&already_checked_set, &parsed_ord1, &qord2) {
    //                     //             falglib::falg_generate_with_qords(&parsed_ord1, &qord2);
    //                     //         }
    //                     //         else {
    //                     //             num_skipped+=1;
    //                     //             //eprintln!("Skipped count: {}", num_skipped);
    //                     //         }
    //                     //     }
    //                     // }
                        
    //                     if falglib::rel_are_pair_antisymmetric(&parsed_ord1, &qord2) {
    //                         num_compat+=1;
    //                             falglib::falg_generate_with_qords(&parsed_ord1, &qord2);                            
    //                     }


                        
    //                     // for perm in falglib::rel_get_stabilizer_perms(&parsed_ord1) {
    //                     //     already_checked_set.insert(falglib::rel_isomorphic_image(&qord2, &perm));
    //                     // }
    //                     // }
    //                     // else {
    //                     //     num_skipped+=1;
    //                     // }

    //                 }
    //                 // already_checked_set.insert(qord2);
    //                 // if cur_perm_cnt % 500 == 1 {
    //                 //     eprintln!("Skipped count: {}", num_skipped);
    //                 // }

    //             }
    //         }
    //         eprintln!("{}\t{}\t{}\t{}", line2_idx, qord2_iso_exp_len, num_compat, time_iter_start.elapsed().as_secs_f64());
    //         // eprintln!("{}\t{}\t{}\t{}\t{}", line2_idx, qord2_iso_exp_len, num_compat, num_skipped, time_iter_start.elapsed().as_secs_f64());
    //         // println!("{:?}", &parsed_ord);
    //         // println!("{:?}", falglib::rel_quasi_order_find_can_min_repr(&parsed_ord));
    //         //println!("{:?}", falglib::rel_quasi_order_find_can_min_repr(&parsed_ord));
    //     }
    // }
    // else {
    //     println!("Error opening file '{}'.", &filename);
    // }



    // Problém s efektívnosťou, dochádza stále k opakovanému výpočtu rel_isomorphic_expand
    // let mut line1_idx = 0usize;
    // if let Ok(lines_qord1) = read_lines(&filename) {
    //     for line_qord1 in lines_qord1.map_while(Result::ok) {
    //         //println!("{}", &line);
    //         line1_idx +=1;
    //         let parsed_ord1 = falglib::parse_tuple(cursize, &line_qord1);
            
    //         let mut line2_idx = 0usize;
    //         if let Ok(lines_qord2) = read_lines(&filename) {
    //             for line_qord2 in lines_qord2.map_while(Result::ok) {
    //                 line2_idx+=1;
    //                 if line2_idx < line1_idx {
    //                     continue;
    //                 }
    //                 let parsed_ord2 = falglib::parse_tuple(cursize, &line_qord2);
    //                 for qord2 in falglib::rel_isomorphic_expand(&parsed_ord2).0 {
    //                     falglib::falg_generate_with_qords(&parsed_ord1, &qord2);
    //                 }
    //             }

    //         }

            
            
    //         // println!("{:?}", &parsed_ord);
    //         // println!("{:?}", falglib::rel_quasi_order_find_can_min_repr(&parsed_ord));
    //         //println!("{:?}", falglib::rel_quasi_order_find_can_min_repr(&parsed_ord));
    //     }
    // }
    // else {
    //     println!("Error opening file '{}'.", &filename);
    // }
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
