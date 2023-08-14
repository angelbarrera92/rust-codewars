// There is a queue for the self-checkout tills at the supermarket. 
// Your task is write a function to calculate the total time required for all the customers to check out!

// Input
// - customers: an array of positive integers representing the queue. Each integer represents a customer, 
//   and its value is the amount of time they require to check out.
// - n: a positive integer, the number of checkout tills.

// Output
// The function should return an integer, the total time required.

// Important
// Please look at the examples and clarifications below, to ensure you understand the task correctly :)

// Examples
// queueTime([5,3,4], 1)
// // should return 12
// // because when there is 1 till, the total time is just the sum of the times

// queueTime([10,2,3,3], 2)
// // should return 10
// // because here n=2 and the 2nd, 3rd, and 4th people in the 
// // queue finish before the 1st person has finished.

// queueTime([2,3,10], 2)
// // should return 12

// Clarifications
// - There is only ONE queue serving many tills, and
// - The order of the queue NEVER changes, and
// - The front person in the queue (i.e. the first element in the array/list) 
//   proceeds to a till as soon as it becomes free.

// N.B. You should assume that all the test input will be valid, as specified above.
// P.S. The situation in this kata can be likened to the more-computer-science-related idea of a thread pool, 
// with relation to running multiple processes at the same time: https://en.wikipedia.org/wiki/Thread_pool

use std::convert::TryFrom;

fn queue_time(customers: &[u32], n: u32) -> u32 {
    
    // If there are no customers...
    let c_l: usize = (*customers).len();
    if c_l == 0 {
        return 0;
    }
    
    // If there is only one queue
    if n == 1 {
        return customers.iter().sum();
    }

    // Total amount of time
    let mut counter = 0;
    // Create a mutable vector with the customers
    let mut c_i: Vec<u32> = Vec::new();
    for c in customers.iter() {
        c_i.push(*c);
    }

    // Init cajas with a zero value
    let mut cajas: Vec<u32> = Vec::new();
    for _ in 0..n {
        cajas.push(u32::MIN);
    }

    // While the queue is not empty...
    while !is_the_queue_empty(&c_i) || !is_the_cajas_empty(&cajas){
        // Fill the cajas
        for i in 0..n {
            // Only if there is no one on it already
            if cajas[usize::try_from(i).unwrap()] == u32::MIN {
                let mut value = u32::MIN;
                for (j, z) in c_i.iter().enumerate() {
                    if *z!=u32::MIN {
                        value = *z;
                        c_i[j] = u32::MIN;
                        break;
                    }
                }
                cajas[usize::try_from(i).unwrap()] = value;
            }
        }
        let min_index_cajas = get_min_index(&cajas);
        let minutes = cajas[min_index_cajas];
        counter += minutes;
        for i in 0..n {
            if i==u32::try_from(min_index_cajas).unwrap(){
                cajas[usize::try_from(i).unwrap()] = u32::MIN;
            }else {
                if cajas[usize::try_from(i).unwrap()]!=u32::MIN {
                    cajas[usize::try_from(i).unwrap()] -= minutes;
                }
            }
        }
    }

    return counter;
}

fn is_the_queue_empty(customers: &Vec<u32>) -> bool {
    return customers.iter().sum::<u32>() == 0;
}

fn is_the_cajas_empty(cajas: &Vec<u32>) -> bool {
    return cajas.iter().sum::<u32>() == 0;
}

fn get_min_index(cajas: &Vec<u32>) -> usize {
    let mut min_index: usize = usize::MAX;
    let mut min_value: u32 = u32::MAX;
    for (j, z) in cajas.iter().enumerate() {
        if z < &min_value && *z!=u32::MIN {
            min_value = *z;
            min_index = j;
        }
    }
    return min_index;
}

fn dotest(c: &[u32], n: u32, expected: u32) {
    let actual = queue_time(c, n);
    assert!(actual == expected, "With customers = {c:?}, n = {n}\nExpected {expected} but got {actual}")
}

fn main(){
    dotest(&[], 1, 0);
    dotest(&[5], 1, 5);
    dotest(&[2], 5, 2);
    dotest(&[1, 2, 3, 4, 5], 1, 15);
    dotest(&[1, 2, 3, 4, 5], 100, 5);
    dotest(&[2, 2, 3, 3, 4, 4], 2, 9);
}