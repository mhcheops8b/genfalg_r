use std::collections::{HashMap,HashSet};
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

    if let Ok(lines_falg) = read_lines(&filename) {
        for line_falg in lines_falg.map_while(Result::ok) {
            let parsed_falg = falglib::parse_vector(cursize, &line_falg);

            let qord1 = falglib::falg_get_qord1(&parsed_falg);
            let qord2 = falglib::falg_get_qord2(&parsed_falg);
            process_orders2_just_tikz(&qord1, &qord2);
            // print!("Rel1: ");
            // process_order(&qord1);
            
            
            // print!("Rel2: ");
            // process_order(&qord2);
            

            println!("\n%%% ==================");
        }
    }
    else {
        eprintln!("Error opening file '{}'.", &filename);
    }
}

fn process_order(qord: &Vec<Vec<usize>>) {
    println!("{:?}", qord);    
    let cls_map = falglib::rel_get_classes_map(qord);
    println!("--Classes--");
    for k_id in 1..=cls_map.len() {
        println!("{}: {:?}", k_id, cls_map[&k_id]);
    }
    let cov_rel = falglib::rel_get_classes_cover_rel(qord, &cls_map);
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
    let cls_coords = falglib::cov_rel_get_class_coords(&cov_rel);
    println!("---Tikz_picture---");
    falglib::_rel_qord_print_tikz(&cls_map, &cov_rel, &cls_coords);
}

fn test_coords(cov_rel: &Vec<Vec<usize>>, cls_coords: &Vec<(usize,usize)>) -> (bool,usize) {
     for i in 0..cov_rel.len() {
        for j in 0..cov_rel.len() {
            if cov_rel[i][j] == 1 {
                for k in 0..cov_rel.len() {
                    if i!=k && j!=k && (cov_rel[i][k] == 1  || cov_rel[k][j] == 1) {
                        if contains_point(&(cls_coords[i].0 as f32, cls_coords[i].1 as f32), 
                            &(cls_coords[j].0 as f32, cls_coords[j].1 as f32), 
                            &(cls_coords[k].0 as f32, cls_coords[k].1 as f32)
                            ) {
                            print!("Problem: cls{} -> cls{} contains cls{}: ", i+1, j+1, k+1);
                            return (false, i+1);
                        }                   
                    }
                }
            }
        }
    }
    (true,0)
}

fn process_orders(qord1: &Vec<Vec<usize>>, qord2: &Vec<Vec<usize>>) {
    println!("Rel1: {:?}", qord1);    
    let cls_map1 = falglib::rel_get_classes_map(qord1);
    println!("--Classes--");
    for k_id in 1..=cls_map1.len() {
        println!("{}: {:?}", k_id, cls_map1[&k_id]);
    }
    let cov_rel1 = falglib::rel_get_classes_cover_rel(qord1, &cls_map1);
    println!("--Cover_Rel--");
    for i in 0..cov_rel1.len() {
        print!("{}: ", i+1);
        let mut b_first = true;
        for j in 0..cov_rel1.len(){
            if cov_rel1[i][j] == 1 {
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
    println!("Rel2: {:?}", qord2);    
    let cls_map2 = falglib::rel_get_classes_map(qord2);
    println!("--Classes--");
    for k_id in 1..=cls_map2.len() {
        println!("{}: {:?}", k_id, cls_map2[&k_id]);
    }
    let cov_rel2 = falglib::rel_get_classes_cover_rel(qord2, &cls_map2);
    println!("--Cover_Rel--");
    for i in 0..cov_rel2.len() {
        print!("{}: ", i+1);
        let mut b_first = true;
        for j in 0..cov_rel2.len(){
            if cov_rel2[i][j] == 1 {
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

    let cls_coords1 = falglib::cov_rel_get_class_coords(&cov_rel1);
    let cls_coords2 = falglib::cov_rel_get_class_coords(&cov_rel2);
    println!("---Tikz_picture---");
    falglib::_rel_qord_print_tikz(&cls_map1, &cov_rel1, &cls_coords1);
    println!("\\quad");
    falglib::_rel_qord_print_tikz(&cls_map2, &cov_rel2, &cls_coords2);
}

fn process_orders2_just_tikz(qord1: &Vec<Vec<usize>>, qord2: &Vec<Vec<usize>>) {
    let cls_map1 = falglib::rel_get_classes_map(qord1);
    let cov_rel1 = falglib::rel_get_classes_cover_rel(qord1, &cls_map1);
    let cls_map2 = falglib::rel_get_classes_map(qord2);
    let cov_rel2 = falglib::rel_get_classes_cover_rel(qord2, &cls_map2);
    let cls_coords1 = falglib::cov_rel_get_class_coords(&cov_rel1);
    let cls_coords2 = falglib::cov_rel_get_class_coords(&cov_rel2);
    println!("%%% ---Tikz_picture---");
    falglib::_rel_qord_print_tikz(&cls_map1, &cov_rel1, &cls_coords1);
    println!("\\quad");
    // let c2:Vec<_> = cls_coords2.iter().map(|(a,b)| (a+(b%2),*b)).collect();
    
    // for i in 0..cov_rel2.len() {
    //     for j in 0..cov_rel2.len() {
    //         if cov_rel2[i][j] == 1 {
    //             for k in 0..cov_rel2.len() {
    //                 if i!=k && j!=k && (cov_rel2[i][k] == 1  || cov_rel2[k][j] == 1) {
    //                     print!("Checking cls{} -> cls{} vs. cls{}: ", i+1, j+1, k+1);
    //                     println!("{}", contains_point(&(cls_coords2[i].0 as f32, cls_coords2[i].1 as f32), 
    //                         &(cls_coords2[j].0 as f32, cls_coords2[j].1 as f32), 
    //                         &(cls_coords2[k].0 as f32, cls_coords2[k].1 as f32)
    //                     ));
    //                 }
                    
    //             }
    //         }
    //     }
    // }
    let tst = test_coords(&cov_rel2, &cls_coords2);
    if !tst.0 {
        let lvl2 = falglib::cov_rel_get_class_levels(&cov_rel2);
        // println!("{:?}", lvl2);
        let mut lvl2_map = HashMap::<usize,Vec<usize>>::new();
        for i in 0..lvl2.len() {
            
            if lvl2_map.contains_key(&lvl2[i]) {
                lvl2_map.get_mut(&lvl2[i]).unwrap().push(i);
            }
            else {
                lvl2_map.insert(lvl2[i], vec![i]);
            }
        }
        
        let mut perm:Vec<_> = (0..cov_rel2.len()).collect();
        if lvl2_map.contains_key(&tst.1) {
            let vv = &lvl2_map[&tst.1];
            if vv.len() >=2 {
                let tmp = perm[vv[0]];
                perm[vv[0]] = perm[vv[1]];
                perm[vv[1]] = tmp;
            }
        }
        // println!("{:?}", lvl2_map);
        let new_cls_coords2 = falglib::cov_rel_get_class_coords_perm(&cov_rel2, &perm);
        falglib::_rel_qord_print_tikz(&cls_map2, &cov_rel2, &new_cls_coords2);
    }
    else {
        falglib::_rel_qord_print_tikz(&cls_map2, &cov_rel2, &cls_coords2);
    }

}

fn my_gcd(a:usize, b:usize) -> usize {
    let mut aa = a;
    let mut bb = b;
    if a < b {
        (aa,bb) = (b,a)
    }

    while bb > 0 {
        (aa, bb) = (bb,aa%bb);
    }
    aa
}

fn contains_point (p:&(f32,f32), q:&(f32,f32), r:&(f32,f32)) -> bool {
    println!("{:?}, {:?}, {:?}",p,q,r);
    let d1_x = q.0 - p.0;
    let d1_y = q.1 - p.1;

    let e1_x = r.0 - p.0;
    let e1_y = r.1 - p.1;

    let mut has_coef_x = true;
    let mut has_coef_y = true;
    if d1_x.abs() < 1e-7 {
        has_coef_x = false;
    }
    if d1_y.abs() < 1e-7 {
        has_coef_y = false;
    }

    if has_coef_x && has_coef_y  {
        // println!("Cx&Cy:{} {}", e1_x/d1_x,e1_y/d1_y);
        return e1_x/d1_x == e1_y/d1_y && e1_x/d1_x <= 1.0;
    }

    // no solution
    if !has_coef_x && !has_coef_y {
        return false;
    }
    
    if !has_coef_x {
        let c = e1_y/d1_y;
        // println!("NoCx:{}", c);
        return e1_x == 0.0 && c>=0.0 && c<=1.0;
    }

    if !has_coef_y {
        let c = e1_x/d1_x;
        // println!("NoCy:{}", c);
        return e1_y == 0.0 && c>=0.0 && c<=1.0;
    }

    // let x1 = p1.0;
    // let y1 = p1.1;
    // let x2 = q1.0;
    // let y2 = q1.1;

    // let x3 = p2.0;
    // let y3 = p2.1;
    // let x4 = q2.0;
    // let y4 = q2.1;

    // let mut vx1:usize;
    // if (x2 >= x1) {
    //     vx1 = x2-x1;
    // }
    // else {
    //     vx1 = x1-x2;
    // }
    // let mut vy1:usize;
    // if (y2 >= y1) {
    //     vy1 = y2-y1;
    // }
    // else {
    //     vy1 = y1-y2;
    // }

    // let mut vx2:usize;
    // if (x4 >= x3) {
    //     vx2 = x4-x3;
    // }
    // else {
    //     vx2 = x3-x4;
    // }
    // let mut vy2:usize;
    // if (y4 >= y3) {
    //     vy2 = y4-y3;
    // }
    // else {
    //     vy2 = y3-y4;
    // }

    // let d1 = my_gcd(vx1,vy1);
    // let d2 = my_gcd(vx2,vy2);

    // let u = ((x2-x1)/d1, (y2-y1)/d1);
    // let v = ((x4-x3)/d2, (y4-y3)/d2);
    
    // println!("(u,v) = ({:?}, {:?})", u, v);
    true
}

fn rel_is_equiv(qord:&Vec<Vec<usize>>, x: usize, y:usize) -> bool {
    qord[x][y] == 1 && qord[y][x]==1
}

fn rel_get_classes(qord:&Vec<Vec<usize>>) {
    let n = qord.len();

    let mut b_has_class_vec = Vec::<bool>::new();
    let mut class_vec = Vec::<usize>::new();
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




// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: &P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
