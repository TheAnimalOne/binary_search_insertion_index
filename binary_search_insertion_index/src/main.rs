fn binary_search_insertion_index (values: &Vec<f32>, target: f32) -> i32 {
    let mut upr_bnd: i32 = values.len() as i32 - 1;
    let mut lwr_bnd: i32 = 0;

    let fnl_ind = loop {
        let mid_ind: i32 = (upr_bnd+lwr_bnd)/2;
        if values[mid_ind as usize] <= target {
            lwr_bnd = mid_ind + 1;
        } else {
            upr_bnd = mid_ind - 1;
        }
        if lwr_bnd > upr_bnd {
            break mid_ind;
        }
    };
    if target < values[fnl_ind as usize] {
        return fnl_ind;
    } else {
        return fnl_ind + 1;
    }
}


fn main() {
    let lst = vec![0.5, 1.5, 2.5, 3.5, 4.5, 5.5, 6.5, 7.5, 8.5, 9.5];

    for test_target in -1..12 {
        let ind = binary_search_insertion_index(&lst, test_target as f32);
        println!("{:?}", ind);
    }
    let lst = vec![1.0, 1.0, 1.0, 1.0, 1.0, 2.0];
    let ind = binary_search_insertion_index(&lst, 1.0);
    println!("{:?}", ind);
    println!("{:?}", &lst[..ind as usize]);
}
