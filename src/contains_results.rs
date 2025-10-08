use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;
// use std::time::{Duration, Instant};

use falglib;
fn main2() {
    let qord1= vec!(vec!(1,1,0,0), vec!(1,1,0,0), vec!(1,1,1,1),vec!(1,1,1,1));
    let qord2= vec!(vec!(1,0,0,0), vec!(0,1,0,0), vec!(1,0,1,0),vec!(1,1,0,1));

    for qo1 in falglib::rel_isomorphic_expand(&qord1).0 {
        for qo2 in falglib::rel_isomorphic_expand(&qord2).0 {
            if falglib::rel_are_pair_antisymmetric(&qord1, &qord2) {
                falglib::falg_generate_with_qords(&qo1, &qo2);
            }
        }
    }


    // println!("{}", falglib::rel_are_pair_antisymmetric(&qord1, &qord2));

    // let status = falglib::falg_generate_with_qords(&qord1, &qord2);
    // println!("{status}");

}

fn main() {
    let args_len = std::env::args().len();

    if args_len < 4 {
        println!("Usage: {} <size> <res_file> <res_to_check_file>", std::env::args().next().unwrap());
        return;
    }

    let mut cursize = 0usize;
    match std::env::args().nth(1).unwrap().parse() {
        Ok(val) => {cursize = val},
        Err(_e) => println!("Must be a number")
    }

    //let filename = String::from("C:/Users/mhycko/Documents/rust_genqord2/results/qord3_ord2canmax.txt");
    let res_filename = std::env::args().nth(2).unwrap();
    let res_chk_filename = std::env::args().nth(3).unwrap();

    // load all results
    let mut all_results = HashSet::<Vec<Vec<usize>>>::new();
    if let Ok(lines_res) = read_lines(&res_filename) {
        for line_qord in lines_res.map_while(Result::ok) {
            all_results.insert(falglib::parse_vector(cursize, &line_qord));
        }
    }
    else {
        println!("Error opening res file '{}'.", &res_filename);
    }
    let all_results_size = all_results.len();
    eprintln!("{}", all_results_size);
    
    if let Ok(lines_chk_res) = read_lines(&res_chk_filename) {
        for line_chk_res in lines_chk_res.map_while(Result::ok) {
            let parsed_chk_res = falglib::parse_vector(cursize, &line_chk_res);
            if !all_results.contains(&parsed_chk_res) {
                // println!("Err: Result: {parsed_chk_res:?} not included");
                println!("{parsed_chk_res:?}");
            }
        }
    }
    else {
        println!("Error opening res file '{}'.", &res_chk_filename);
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
