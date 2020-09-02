use std::cmp;

#[inline(never)]
pub fn cmp_gt_and(in1: &[i16], in2: &[i16], destination: &mut [bool]) {
    let max = cmp::min(cmp::min(in1.len(), in2.len()), destination.len());

    let src1 = &in1[0..max];
    let src2 = &in2[0..max];
    let dst = &mut destination[0..max];

    for index in 0..max {
        dst[index] &= src1[index] < src2[index];
    }
}

fn main() {
    let len = 100;
    let a: Vec<i16> = (1..len).collect();
    let b: Vec<i16> = (1..len).map(|x| len - x).collect();
    let mut result = vec![false; len as usize];

    for _ in 0..100*1000*1000 {
        cmp_gt_and(&a, &b, &mut result);
    }

    let sum: i32 = b.into_iter().map(|x| x as i32).sum();
    std::process::exit(sum);
}