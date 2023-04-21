use std::{fs, collections::BinaryHeap };
use std::cmp;
use std::path::Path;

fn main(){
    let input = fs::read_to_string("inputs/inp1.txt").unwrap();
    
    let mut heap: BinaryHeap<i32> = BinaryHeap::new();
    let mut max_calories: i32 = 0;
    let mut cur_sum: i32 = 0;
    
//    println!("all clear");

    for line in input.lines(){
        if line.len()==0{
//            max_calories = cmp::max(max_calories, cur_sum);
            heap.push(cur_sum);
            cur_sum = 0;
        }else{
 //           println!("{}", line);
            cur_sum+= line.parse::<i32>().unwrap();
        }
    }

    if cur_sum>0{
        heap.push(cur_sum);
//        max_calories = cmp::max(max_calories, cur_sum);             
    }
    let top3_total = heap.pop().unwrap() + heap.pop().unwrap() + heap.pop().unwrap();
    println!("{}", top3_total);

}
