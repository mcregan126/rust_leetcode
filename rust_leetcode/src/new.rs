// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
fn main(){

    Solution::delete_duplicates;

}


struct Solution; 

impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
         
       let mut current = head.as_mut();
       
       while let Some(node) = current {
            while let Some(next) = node.next.as_mut(){
                if node.val == next.val {
                    node.next = next.next.take();
                }
                else{
                    break;
                }
            }
            current = node.next.as_mut();
        }   
        head
    }
}