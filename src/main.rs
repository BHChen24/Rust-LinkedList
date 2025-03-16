use std::collections::BinaryHeap;
use std::cmp::Reverse;


fn main() {
    let my_heap: BinaryHeap<i32> = BinaryHeap::from(vec![11, 2, 5, 6, 7, 8, 9, 12, 4, 10, 1, 3, 15, 13, 14]);

    // let mut second_head :BinaryHeap<Reverse<i32>> = BinaryHeap::new();

    // second_head.push(Reverse(10));
    // second_head.push(Reverse(6));
    // second_head.push(Reverse(5));
    // second_head.push(Reverse(4));
    // second_head.push(Reverse(9));
    // second_head.push(Reverse(16));
    // second_head.push(Reverse(7));
    // second_head.push(Reverse(18));
    // second_head.push(Reverse(22));
    // second_head.push(Reverse(1));
    // second_head.push(Reverse(3));
    // second_head.push(Reverse(8));
    // second_head.push(Reverse(2));
    // second_head.push(Reverse(11));

    let sort_vec = my_heap;
    println!("{:?}", sort_vec);
    // println!("{:?}", second_head);
}