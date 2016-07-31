use std::vec;


fn main() {
    let mut v = vec![0,1,1,2,3,5,8,13];
    bubble_sort(v.as_mut_slice());

    for num in v {
        println!("Num {}", num);
    }
}


fn bubble_sort(A: &mut [u32]) {
    for x in A.len()-1..0 {
        for y in 0..x {
            if (A[y] > A[y+1]) {
                let temp = A[y];
                A[y] = A[y+1];
                A[y+1] = temp;
            }
        }
    }
}

