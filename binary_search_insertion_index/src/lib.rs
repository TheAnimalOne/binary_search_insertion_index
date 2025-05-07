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
    // check mid-point
    // 1. when mid-point value is less than target -> lwr = mid + 1
    // 2. when mid-point value is greater than target -> upr = mid - 1
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
    // for repeating values, check if we are at the end or need to increment by 1
    if target < values[fnl_ind as usize] {
        fnl_ind
    } else {
        fnl_ind + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // test standard logic working
    #[test]
    fn test_bin_search_standard_logic() {
        let lst = vec![0.5, 1.5, 2.5, 3.5, 4.5, 5.5, 6.5, 7.5, 8.5, 9.5];
        for test_target in -1..12 {
            let ind = binary_search_insertion_index(&lst, test_target as f64);
            let exp_ind = if test_target < 0 {
                0
            } else if test_target > lst.len() as i32 {
                lst.len() as i32
            } else {
                test_target as i32
            };
            assert_eq!(ind, exp_ind);
        }
    }

    // test repeated values
    #[test]
    fn test_repeated_values() {
        let lst = vec![1.0, 1.0, 1.0, 1.0, 1.0, 2.0];
        let ind = binary_search_insertion_index(&lst, 1.0);
        assert_eq!(ind, 5);
    }

    // test list slice includes all values up to return index
    #[test]
    fn test_list_slice_values() {
        let lst = vec![1.0, 1.0, 1.0, 1.0, 1.0, 2.0];
        let ind = binary_search_insertion_index(&lst, 1.0);
        assert_eq!(&lst[..ind as usize], [1.0, 1.0, 1.0, 1.0, 1.0]);
    }

    // test using list over vec
    fn test_list_input_instead_of_vec() {
        let lst = [1, 1, 1, 2, 2, 2];
        let ind = binary_search_insertion_index(&lst, 2);
        assert_eq!(ind, 5);
    }
}
