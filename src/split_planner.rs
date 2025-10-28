use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;
// use std::time::{Duration, Instant};

fn main() {
    let args_len = std::env::args().len();

    if args_len < 5 {
        println!("Usage: {} <size> <rel_file> <ord_idx> <num_parts> [<ordexp_from> [<ordexp_to>]]", std::env::args().next().unwrap());
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

    let mut num_parts = 0usize;
    match std::env::args().nth(4).unwrap().parse() {
        Ok(val) => {num_parts = val},
        Err(_e) => println!("Must be a number")
    }


    let mut b_has_from = false;
    let mut ordexp_from = 0usize;
    if args_len >= 6 {
        // only from 
        match std::env::args().nth(5).unwrap().parse() {
            Ok(val) => {ordexp_from = val},
            Err(_e) => println!("Must be a number")
        }
        b_has_from = true;
    }
    let mut b_has_to = false;
    let mut ordexp_to = 0usize;

    if args_len == 7 {
        // from to
        match std::env::args().nth(6).unwrap().parse() {
            Ok(val) => {ordexp_to = val},
            Err(_e) => println!("Must be a number")
        }
        b_has_to = true;
    }

    // eprintln!("QQ: {b_has_from} - {ordexp_from} - {b_has_to} - {ordexp_to}");

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
    let red_qords_size = red_qords.len();
    eprintln!("{}", red_qords_size);
    

    // Výpočet:
    //  1. redukovaného, páry ( qo_i, iso_exp(qo_j) ) pre j>=i
    //  2. iso_exp sa volá pre každé j iba raz

    if curord_idx < 1 || curord_idx > red_qords_size {
        eprintln!("Current order index must be in range 1 <= curord_idx <= {red_qords_size}.");
        return;
    }

    let time_iter_start = Instant::now();            
    let qord2_iso_exp = falglib::rel_isomorphic_expand_vec(&red_qords[curord_idx-1]).0;
    let qord2_iso_exp_len = qord2_iso_exp.len();
    eprintln!("Line: {} - {qord2_iso_exp_len}", curord_idx);
    let mut num_compat = 0usize;
    let mut cur_perm_cnt = 0usize;
    let mut ordexp_ffrom  = 0usize;
    if b_has_from {
        if ordexp_from < 1 || ordexp_from > qord2_iso_exp_len {
            eprintln!("Specified ordexp_from must be 1 <= ordexpfrom <= {qord2_iso_exp_len}");
            return;
        }
        ordexp_ffrom = ordexp_from;
    }
    else {
        ordexp_ffrom = 1usize;
    }
    let mut ordexp_tto  = 0usize;
    if b_has_to {
        if ordexp_to < 1 || ordexp_to > qord2_iso_exp_len {
            eprintln!("Specified ordexp_to must be 1 <= ordexpfrom <= {qord2_iso_exp_len}");
            return;
        }
        ordexp_tto = ordexp_to;
    }
    else {
        ordexp_tto = qord2_iso_exp_len;
    }

    // eprintln!("HH: {ordexp_ffrom} - {ordexp_tto}");
    // return;
    let num_items = ordexp_tto - ordexp_ffrom + 1;
    let iter_limit = 1;
    //let mut time_cur_iter_start:Instant = Instant::now();
    let mut l_num_compat:usize = 0;
    let mut l_cnt:usize = 0;
    let mut sizes = Vec::<(usize,usize,usize)>::new();
    
    for qord2exp_idx in (ordexp_ffrom-1)..=(ordexp_tto-1) {
            l_num_compat = 0;
 
            for qord1_idx in 0..curord_idx { 
                    
                if falglib::rel_are_pair_antisymmetric(&red_qords[qord1_idx], &qord2_iso_exp[qord2exp_idx]) {
                    num_compat+=1;
                    l_num_compat+=1;
                    //falglib::falg_generate_with_qords(&red_qords[qord1_idx], &qord2_iso_exp[qord2exp_idx]);                            
                }
            }
            //l_cnt+=1;
            // if l_cnt == iter_limit {
            //     eprintln!(" - {} ", l_num_compat);
            //     l_cnt = 0;
            // }
            
                // already_checked_set.insert(qord2);
                // if cur_perm_cnt % 500 == 1 {
                //     eprintln!("Skipped count: {}", num_skipped);
                // }
        if qord2exp_idx % 500 == 0 {
            eprintln!("{} - {} - {}", qord2exp_idx+1, l_num_compat, num_compat);
            sizes.push((qord2exp_idx+1, l_num_compat, num_compat));    
        }
        
    }
    // if l_cnt != 0 {
    //     eprintln!(" - {}", l_num_compat);
    // }
    eprintln!("{:?}",sizes);
    eprintln!("{}\t{}\t{}\t{}", curord_idx, num_items, num_compat, time_iter_start.elapsed().as_secs_f64());
    
    let mut per_instance = num_compat / num_parts;    
    if num_items % num_parts != 0 {
        per_instance +=1;
    }   

    let mut start_vec_idx = 0usize;
    let mut cur_instance_limit = per_instance;
    let mut end_vec_idx = 0usize;

    while end_vec_idx < sizes.len() {
        if sizes[end_vec_idx].2 > cur_instance_limit {
            println!("{start_vec_idx} - {}: {} {}", end_vec_idx - 1, sizes[start_vec_idx].0, sizes[end_vec_idx -1].0);
            start_vec_idx  = end_vec_idx;
            cur_instance_limit += per_instance;

            if cur_instance_limit > num_compat {
                cur_instance_limit = num_compat;
            }

        }
        else {
            end_vec_idx += 1;
        }
    }
    
    println!("{start_vec_idx} - {}: {} {}", sizes.len() - 1, sizes[start_vec_idx].0, sizes[sizes.len() - 1].0);

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
