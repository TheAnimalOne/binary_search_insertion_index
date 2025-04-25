use std::cmp::PartialOrd;


fn binary_search_insertion_index<T> (values: &[T], target: T) -> i32
where
    T: PartialOrd,
{
    // early exit condition:
    // 1. if empty list -> return 0
    // 2. target >= max value -> return len of list
    let n: usize = values.len();
    if (n == 0) | (target >= values[n-1]) {
        return n as i32;
    }

    let mut upr_bnd: i32 = n as i32 - 1;
    let mut lwr_bnd: i32 = 0;

    let fnl_ind: i32 = loop {
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
    // test standard logic working
    let lst = vec![0.5, 1.5, 2.5, 3.5, 4.5, 5.5, 6.5, 7.5, 8.5, 9.5];

    for test_target in -1..12 {
        let ind = binary_search_insertion_index(&lst, test_target as f32);
        println!("{:?}", ind);
    }
    // test repeats add to back
    let lst = vec![1.0, 1.0, 1.0, 1.0, 1.0, 2.0];
    let ind = binary_search_insertion_index(&lst, 1.0);
    println!("{:?}", ind);
    // test slicing
    println!("{:?}", &lst[..ind as usize]);
    // test switch to list
    let lst = [1, 1, 1, 2, 2, 2];
    let ind = binary_search_insertion_index(&lst, 2);
    println!("{:?}", ind);
}
