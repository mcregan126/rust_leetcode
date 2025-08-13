fn main() {
    let nums = [27, 0, -1, 1, 3, 9, 45, 81, 100, 243];
    
    for &n in &nums {
        println!("n = {:>4}, is_power_of_three = {}", n, Solution::is_power_of_three(n));
    }
}

struct Solution;

impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        
        let mut nn = n;
        if nn <= 0{
         return false;
        }
        
    
        while nn % 3 == 0 {
            nn /= 3;
            if nn == 1{
               return true; 
            }
        }
        return false;
    }
}