// Given an array of integers, return a new array such that each element at index i of the new
// array is the product of all the numbers in the original array except the one at i.
//
// For example,
// if our input was [1, 2, 3, 4, 5], the expected output would be [120, 60, 40, 30, 24].
//
// If our input was [3, 2, 1], the expected output would be [2, 3, 6].
//
// Follow-up: what if you can't use division?

fn main() {
    let array = vec![10, 3, 5, 6, 2];
    println!("array: {:?}", array);

    let prod = |array: Vec<i32>| {
        let mut prod_array = Vec::with_capacity(array.len());
        let mut temp = 1;

        for x in 0..array.len() {
            prod_array.push(1);
        }

        for x in 0..array.len() {
            prod_array[x] = temp;
            temp *= array[x];
            println!("Prod_array: {:?}", prod_array);
        }

        temp = 1;

        for i in (0..array.len()).rev() {
            prod_array[i] *= temp;
            temp *= array[i];
            println!("Prod_array 2: {:?}", prod_array);
        }


        prod_array
    };

    println!("prod {:?}", prod(array));
}
