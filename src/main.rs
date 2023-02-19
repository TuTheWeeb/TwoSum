/*

Solution to the TwoSum problem on LeetCode

*** I didn't optimze it ***

 */

// Get the array and the target and gives back the result, if there's no solution the it gives back [0,0]
fn two_sum (arr: &Vec<i32>, target: &i32) -> [i32;2] {

    // Declare a result variable so that can be returned    
    let mut result: [i32;2] = [0,0];

    // Just to be more readable
    let lenght_arr: usize = arr.len();


    // Rows the first number
    'outer_loop: for y in 0..lenght_arr{
        // Rows the second number
        for x in 0..lenght_arr{
            
            // Check to see if the sum is right, if so the break the nested loop using a label
            if &arr[y] + &arr[x] == *target {
                println!("arr[y]: {}, arr[x]: {}", &arr[y], arr[x]);
                result = [ y as i32 , x as i32 ];
                break 'outer_loop;
            }           
        }
    }

    return result;
}

fn main() {
    // Declare the array and initiate the array, so do the target
    let nums: Vec<i32> = vec![2,3,7,15,0];
    let target = 15; 
    
    // Executes the code directly in the println macro
    println!("The solution is: {:?}", two_sum(&nums, &target));
    
}
