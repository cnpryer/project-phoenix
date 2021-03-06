fn merge<T: Ord + Copy>(vec: &mut Vec<T>, low: usize, middle: usize, high: usize) -> &Vec<T> {
    let mut left_half = Vec::new();
    let mut right_half = Vec::new();

    for v in vec.iter().take(middle + 1).skip(low) {
        left_half.push(*v);
    }
    for v in vec.iter().take(high + 1).skip(middle + 1) {
        right_half.push(*v);
    }

    let left_len = left_half.len();
    let right_len = right_half.len();

    let mut l_i = 0;
    let mut r_i = 0;
    let mut a_i = low;

    // get smaller of left or right halves
    while l_i < left_len && r_i < right_len {
        if left_half[l_i] < right_half[r_i] {
            vec[a_i] = left_half[l_i];
            l_i += 1;
        } else {
            vec[a_i] = right_half[r_i];
            r_i += 1;
        }

        a_i += 1;
    }

    // handle the remaining values
    while l_i < left_len {
        vec[a_i] = left_half[l_i];
        l_i += 1;
        a_i += 1;
    }

    while r_i < right_len {
        vec[a_i] = right_half[r_i];
        r_i += 1;
        a_i += 1;
    }

    return vec;
}

fn inner_merge_sort<T: Ord + Copy>(vec: &mut Vec<T>, low: usize, high: usize) -> &Vec<T> {
    if low >= high {
        return vec;
    }

    let middle = low + (high - low) / 2;

    inner_merge_sort(vec, low, middle);
    inner_merge_sort(vec, middle + 1, high);

    let new_vec = merge(vec, low, middle, high);

    return new_vec;
}

pub fn merge_sort<T: Ord + Copy>(vec: &mut Vec<T>) -> &Vec<T> {
    let len = vec.len();
    if len <= 1 {
        return vec;
    }

    let new_vec = inner_merge_sort(vec, 0, len - 1);

    return new_vec;
}
