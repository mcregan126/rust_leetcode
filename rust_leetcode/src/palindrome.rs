fn main() {
  
    let test_cases = vec![
        (121, true),      
        (-121, false),   
        (10, false),      
        (0, true),        
        (12321, true),    
        (1221, true),    
        (1001, true),    
        (12345, false),   
        (1000021, false),  
    ];

    for (num, expected) in test_cases {
        let result = Solution::is_palindrome(num);
        println!("is_palindrome({}) = {}, expected {}", num, result, expected);
        assert_eq!(result, expected);
    }
}
struct Solution; 

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let y = x.to_string();
        if y.len() > 0 {
            false;
        }    
        let z = reverse(&y);
        if  z == y {
           true
        }
        else{
            false
        }

    }
}


fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}



