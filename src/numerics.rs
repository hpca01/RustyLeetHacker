use std::cmp::max;

pub fn multiply_large_nums(left: isize, right: isize) -> isize {
    if left < 10 && right < 10 {
        left * right
    } else {
        let size = max(size_of_num(left), size_of_num(right));
        let half = size / 2;

        let (high_1, low_1) = split_num_at(half, left);
        let (high_2, low_2) = split_num_at(half, right);

        let z0 = multiply_large_nums(low_1, low_2);
        let z1 = multiply_large_nums(low_1 + high_1, low_2 + high_2);
        let z2 = multiply_large_nums(high_1, high_2);

        (z2 * (10_isize).pow(half * 2)) + (z1 - z0 - z2) * (10_isize).pow(half) + z0
    }
}

fn size_of_num(num: isize) -> u32 {
    let mut copy = num;
    let mut num_digits = 1;

    while copy > 9 {
        copy /= 10;
        num_digits += 1;
    }
    num_digits
}

fn split_num_at(pos: u32, num: isize) -> (isize, isize) {
    let shift = 10_isize.pow(pos);
    let high = num / shift;
    let low = num % shift;
    (high, low)
}
