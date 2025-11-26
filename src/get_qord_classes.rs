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
    // rel_get_classes(&curqord);
    let cls_map = falglib::rel_get_classes_map(&curqord);
    // println!("---\n{:?}", cls_map);
    // println!("---");
    // for (k,v) in cls_map.iter() {
    //     println!("{}: {:?}", k, v);
    // }
    println!("--Classes--");
    for k_id in 1..=cls_map.len() {
        println!("{}: {:?}", k_id, cls_map[&k_id]);
    }
    let cov_rel = rel_get_classes_cover_rel(&curqord, &cls_map);
    println!("--Cover_Rel--");
    for i in 0..cov_rel.len() {
        print!("{}: ", i+1);
        let mut b_first = true;
        for j in 0..cov_rel.len(){
            if cov_rel[i][j] == 1 {
                if b_first {
                    b_first = false;
                }
                else {
                    print!(", ");
                }
                print!("{}", j+1);
            }
        }
        println!();
    }
    
    // println!("---\n{:?}", rel_get_classes_order(&curqord, &cls_map));
    
    // println!("---\n{:?}", cov_rel);
    // get_levels(&cov_rel);
    let cls_coords = falglib::cov_rel_get_class_coords(&cov_rel);
    // println!("---\n{:?}", cls_coords);
    println!("---Tikz_picture---");
    falglib::_rel_qord_print_tikz(&cls_map, &cov_rel, &cls_coords);

    // println!("---");
    // falglib::rel_qord_print_tikz(&curqord);
}

// pub fn _rel_qord_print_tikz(cls_map: &HashMap<usize, Vec<usize>>, cov_rel: &Vec<Vec<usize>>, cls_coords: &Vec<(usize, usize)>) {
//     println!("\\tikz {{");
//     for i in 1..=cov_rel.len() {
//         print!("\\node [circle,fill,label={{[name=cls{}]below:$", i);
//         let mut b_first = true;
//         for e in &cls_map[&i] {
//             if b_first {
//                 b_first = false;
//             }
//             else {
//                 print!(", ");
//             }
//             print!("{}", e);
//         }
            
//         println!("$}}] at {:?} {{}};", cls_coords[i-1]);
//     }
//     for i in 0..cov_rel.len() {
//         for j in 0..cov_rel.len() {
//             if cov_rel[i][j] == 1 {
//                 println!("\\draw {:?} -- {:?};", cls_coords[i], cls_coords[j]);
//             }
//         }
//     }
//     println!("}}");
// }

// fn rel_qord_print_tikz(qord: &Vec<Vec<usize>>) {//cls_map: &HashMap<usize, Vec<usize>>, cov_rel: &Vec<Vec<usize>>, cls_coords: &Vec<(usize, usize)>) {
//     let cls_map = rel_get_classes_map(&qord);
//     let cov_rel = rel_get_classes_cover_rel(&qord, &cls_map);
//     let cls_coords = get_class_coords(&cov_rel);
//     _rel_qord_print_tikz(&cls_map, &cov_rel, &cls_coords);
// }



// fn get_class_coords(cov_rel: &Vec<Vec<usize>>) -> Vec<(usize,usize)> {
//     let n = cov_rel.len();

//     let mut max_levels = Vec::<usize>::new();
//     let mut x_coords = Vec::<usize>::new();
//     let mut last_x_coord = Vec::<usize>::new();
//     for _ in 0..n {
//         max_levels.push(0);
//         x_coords.push(0);
//         last_x_coord.push(0);
//     }

//     for i in 0..n {
//         if max_levels[i] == 0 {
//             max_levels[i] = 1;
//       //      x_coords[i] = 
//         }
//         for j in 0..n {
//             if cov_rel[i][j] == 1 {
//                 max_levels[j] = std::cmp::max(max_levels[j], max_levels[i] + 1);
//             }
//         }
//     }
//     //println!("{:?}", max_levels);
//     for j in 0..n {
//         last_x_coord[max_levels[j] - 1] +=1;
//         x_coords[j] = last_x_coord[max_levels[j] - 1]; 
//     }
//     //println!("{:?}", x_coords);
//     let iter = std::iter::zip(x_coords,max_levels);
//     let v:Vec<_> = iter.map(|(a,b)| (a-1,b-1)).collect();

//     v
// }

fn get_levels(cov_rel: &Vec<Vec<usize>>) {
    let n = cov_rel.len();

    let mut max_levels = Vec::<usize>::new();
    let mut x_coords = Vec::<usize>::new();
    let mut last_x_coord = Vec::<usize>::new();
    for _ in 0..n {
        max_levels.push(0);
        x_coords.push(0);
        last_x_coord.push(0);
    }

    for i in 0..n {
        if max_levels[i] == 0 {
            max_levels[i] = 1;
      //      x_coords[i] = 
        }
        for j in 0..n {
            if cov_rel[i][j] == 1 {
                max_levels[j] = std::cmp::max(max_levels[j], max_levels[i] + 1);
            }
        }
    }
    println!("{:?}", max_levels);
    for j in 0..n {
        last_x_coord[max_levels[j] - 1] +=1;
        x_coords[j] = last_x_coord[max_levels[j] - 1]; 
    }
    println!("{:?}", x_coords);
    let iter = std::iter::zip(x_coords,max_levels);
    let v:Vec<_> = iter.map(|(a,b)| (a-1,b-1)).collect();
    println!("{:?}", v);

//    for i in 0..n {
//        print!("\\node [circle,fill,label={[name=cls{}]below:$", i+1);
//        for e in cls_map
//        println!("$}] {};");
//    }
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

// fn rel_is_equiv(qord:&Vec<Vec<usize>>, x: usize, y:usize) -> bool {
//     qord[x][y] == 1 && qord[y][x]==1
// }

// fn rel_get_classes_map(qord:&Vec<Vec<usize>>) -> HashMap<usize, Vec<usize>> {
//     let n = qord.len();

//     let mut b_has_class_vec = Vec::<bool>::new();
//     let mut id_to_cls_vec_map = HashMap::<usize, Vec<usize>>::new();
//     for i in 0..n {
//         b_has_class_vec.push(false);
//     }

//     let mut last_class_id = 0usize;
//     for x in 0..n {
//         if !b_has_class_vec[x] {
//             last_class_id +=1;
//             id_to_cls_vec_map.insert(last_class_id, Vec::from([x]));

//             for y in (x+1)..n {
//                 if rel_is_equiv(qord, x, y) {
//                     b_has_class_vec[y] = true;
//                     id_to_cls_vec_map.get_mut(&last_class_id).unwrap().push(y);

//                 }
//             }
//         }
//     }
//     id_to_cls_vec_map 
// }

fn rel_get_classes_order(qord:&Vec<Vec<usize>>, cls_map:&HashMap<usize, Vec<usize>>) -> Vec<Vec<usize>> {
    let n = cls_map.len();

    let mut vec_res = Vec::<Vec<usize>>::new();
    for i in 0..n {
        vec_res.push(Vec::<usize>::new());
        for _ in 0..n {
            vec_res[i].push(0);
        }

    }

    for i in 1..=n {
        for j in 1..=n {
            if i != j {
                let x_i = cls_map.get(&i).unwrap()[0];
                let x_j = cls_map.get(&j).unwrap()[0];
                if qord[x_i][x_j] == 1 {
                    vec_res[i-1][j-1] = 1;
                }
            }
            else {
                vec_res[i-1][j-1] = 1;
            }

        }
    }
    vec_res
}

fn rel_get_classes_cover_rel(qord:&Vec<Vec<usize>>, cls_map:&HashMap<usize, Vec<usize>>) -> Vec<Vec<usize>> {
    let n = cls_map.len();

    let mut vec_res = Vec::<Vec<usize>>::new();
    for i in 0..n {
        vec_res.push(Vec::<usize>::new());
        for _ in 0..n {
            vec_res[i].push(0);
        }

    }

    for i in 1..=n {
        for j in 1..=n {
            if i != j {
                let x_i = cls_map.get(&i).unwrap()[0];
                let x_j = cls_map.get(&j).unwrap()[0];
                if qord[x_i][x_j] == 1 {
                    let mut b_found = false;
                    for k in 1..=n {
                        if k != i && k != j {
                            let x_k = cls_map.get(&k).unwrap()[0];
                            if qord[x_i][x_k] == 1 && qord[x_k][x_j] == 1 {
                                b_found = true;
                                break;
                            }
                        }
                    }
                    if !b_found {
                        vec_res[i-1][j-1] = 1;
                    }
                }
            }
        }
    }
    vec_res
}


fn rel_get_classes(qord:&Vec<Vec<usize>>) {
    let n = qord.len();

    let mut b_has_class_vec = Vec::<bool>::new();
    let mut class_vec = Vec::<usize>::new();
    let mut id_to_cls_map = HashMap::<usize, HashSet<usize>>::new();
    let mut el_to_id_map =  HashMap::<usize, usize>::new();
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
            id_to_cls_map.insert(last_class_id, HashSet::from([x]));
            el_to_id_map.insert(x, last_class_id);

            for y in (x+1)..n {
                if falglib::rel_is_equiv(qord, x, y) {
                    b_has_class_vec[y] = true;
                    class_vec[y] = last_class_id;
                    id_to_cls_map.get_mut(&last_class_id).unwrap().insert(y);
                    el_to_id_map.insert(y, last_class_id);

                }
            }
        }
    }
    // eprintln!("Dbg: {:?}", id_to_cls_map);
    // eprintln!("Dbg: {:?}", el_to_id_map);
    //eprintln!("{:?}", id_to_cls_map[&1].iter().nth(0).unwrap());

    println!("{last_class_id}");
    for i in 1..=last_class_id {
        print!("{i}: ");
        let mut b_first = true;
        for ex in id_to_cls_map[&i].iter() {
            if b_first {
                b_first = false;
            }
            else {
                print!(", ");
            }
            print!("{}", *ex);
        }
        // for x in 0.. n {
        //     if class_vec[x] == i {
        //         if b_first {
        //             print!("{x}");
        //             b_first = false;
        //         }
        //         else {
        //             print!(", {x}")
        //         }
        //     }
        // }
        println!();
    }

    println!("--Cls_Less--");
    for i in 1..=last_class_id {
        let repr_i = id_to_cls_map[&i].iter().nth(0).unwrap();
        // let mut repr_i = 0usize;
        // for x in 0..n {
        //     if class_vec[x]==i {
        //         repr_i = x;
        //         break;
        //     }
        // }
        print!("{i}: ");
        let mut b_first = true;
        for j in 1..=last_class_id {
            let repr_j = id_to_cls_map[&j].iter().nth(0).unwrap();
            // let mut repr_j = 0usize;
            // for x in 0..n {
            //     if class_vec[x]==j {
            //         repr_j = x;
            //         break;
            //     }
            // }
            if qord[*repr_i][*repr_j] == 1 {
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

    println!("--Cls_Cover--");
    for i in 1..=last_class_id {
        let repr_i = id_to_cls_map[&i].iter().nth(0).unwrap();
        print!("{i}: ");
        let mut b_first = true;
        for j in 1..=last_class_id {
            if i == j {
                continue
            }
            let repr_j = id_to_cls_map[&j].iter().nth(0).unwrap();
            // let mut repr_j = 0usize;
            // for x in 0..n {
            //     if class_vec[x]==j {
            //         repr_j = x;
            //         break;
            //     }
            // }
            if qord[*repr_i][*repr_j] == 1 {
                let mut b_found = false;
                for k in 1..=last_class_id {
                    if i != k && j != k {
                        let repr_k = id_to_cls_map[&k].iter().nth(0).unwrap();
                        if qord[*repr_i][*repr_k] == 1 && qord[*repr_k][*repr_j] == 1 {
                            b_found = true;
                        }
                    }
                    
                }
                if !b_found {
                    if !b_first {
                        print!(", ");
                    }
                    else {
                        b_first = false;
                    }
                    print!("{j}")
                }
            }
        }
        println!();
    }

//     let cl_id = 0;
}
