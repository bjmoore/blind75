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
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let target = rng.gen_range(-109..110);
    let size = rng.gen_range(2..104);

    let input_list = generate_test_input(target, size);

    let answer = solver(&input_list, target);

    println!("target: {}, num_inputs: {}, result: [{}, {}]", target, size, input_list[answer[0]], input_list[answer[1]]);

    // plug into solver

    // check that result a + b = target
}

fn generate_test_input(target: i32, size: i32) -> Vec<i32> {
    let mut output = Vec::new();
    let mut banned_values = HashSet::new();
    let mut rng = rand::thread_rng();

    let a: i32 = rng.gen_range(-109..110);
    output.push(a);
    output.push(target - a);

    while output.len() < size.try_into().unwrap() {
        let a: i32 = rng.gen_range(-109..110);
        if !banned_values.contains(&a) {
            output.push(a);
            banned_values.insert(target - a);
        }
    }

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