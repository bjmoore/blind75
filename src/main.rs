/* 
    Generate list of ints where exactly two of them add up to a specific target value

    1. Take target value and divide it randomly into 2 pieces a + b = target. a and b cannot be reused
    2. Start adding other random values c_i. For each c_i, c_i can be reused but target - c_i = d_i is now forbidden

    2 <= nums.length <= 104
    -109 <= nums[i] <= 109
    -109 <= target <= 109
*/

use std::collections::HashSet;
use std::collections::HashMap;
use std::time::Instant;
use rand::Rng;
use rand::seq::SliceRandom;

fn main() {
    let run_count = 1000;
    let sizes = [100, 1000, 10000, 100000, 1000000];

    for size in sizes {
        let start = Instant::now();
        for _x in 0..run_count {
            problem_1(size);
        }
        let duration = start.elapsed();   
        println!("input size: {}, time taken: {:?}", size, duration.div_f32(run_count as f32));
    }
}

fn problem_1(size: u32) {
    let mut rng = rand::thread_rng();
    let target = rng.gen_range(-50000..50000);
    let input_list = generate_test_input(target, size);
    let answer = solver(&input_list, target);
    
    //println!("target: {}, num_inputs: {}, result: [{}, {}]", target, size, input_list[answer[0]], input_list[answer[1]]);
}

fn generate_test_input(target: i32, size: u32) -> Vec<i32> {
    let mut output = Vec::new();
    let mut banned_values = HashSet::new();
    let mut rng = rand::thread_rng();

    let a: i32 = rng.gen_range(-50000..50000);
    output.push(a);
    output.push(target - a);

    while output.len() < size.try_into().unwrap() {
        let a: i32 = rng.gen_range(-50000..50000);
        if !banned_values.contains(&a) {
            output.push(a);
            banned_values.insert(target - a);
        }
    }

    output.shuffle(&mut rng);

    return output
}

fn solver(input_list: &Vec<i32>, target: i32) -> [usize; 2] {
    let mut hash = HashMap::new();

    for (i, x) in input_list.iter().enumerate() {
        let complement = target - x;
        if hash.contains_key(&complement) {
            return [i.try_into().unwrap(), *hash.get(&complement).unwrap()];
        } else {
            hash.insert(x, i.try_into().unwrap());
        }
    }

    return [0, 0]
}