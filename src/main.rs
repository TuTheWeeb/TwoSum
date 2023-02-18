fn two_sum (arr: &Vec<i32>, target: &i32) -> [i32;2] {
    
    let mut result: [i32;2] = [0,0];
    let lenght_arr: usize = arr.len();


    // Rows the first number
    'outer_loop: for y in 0..lenght_arr{
        // Rows the second number
        for x in 0..lenght_arr{
            
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
    let nums: Vec<i32> = vec![2,3,7,15,0];
    let target = 15; 
    println!("The solution is: {:?}", two_sum(&nums, &target));
    
}
