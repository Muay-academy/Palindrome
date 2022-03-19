//การใช้งานแบบ array(สำหรับการเปรียบเทียบ)
//Arguments
//*'Phrase' วลี สำหรับ inspect
//ใกล้เคียงกับความหมายที่ใช้ในภาษา C ยกเว้น chars().collect()สิ่งนี้เป็น 
//0(n/2)
use std::io;   // Module สำหรับการอ่านและการเขียน ของ   input and output
fn is_palindrome(phrase: &str) -> bool {
    if phrase.len() == 0 { return true }
    //arrays

  let phrase: Vec<char> = phrase.chars().collect();
    let mut first_idx = 0;  
    
    
    let mut last_idx = phrase.len() - 1;
    
    while first_idx < last_idx {
       
        if !phrase[first_idx].is_alphabetic() { first_idx += 1; continue }
        if !phrase[last_idx].is_alphabetic() { last_idx -= 1; continue }
        // filter out non-alphabetics, the += and -= would be something you could accidentally screw up,
        //   avoided in the iterator based impl

        // compare the values, did we compare the correct indexes? again avoided in the iterator impl
        if phrase[first_idx].to_ascii_lowercase() != phrase[last_idx].to_ascii_lowercase() {
            return false;
        }
        // same += and -= potential bug avoided in the iterator impl
        first_idx += 1;
        last_idx -= 1;
    }
        return true
}

fn main() {
    let mut input_text = String::new();
    println!("Enter a string for check palindrome:  ");
    io::stdin().read_line(&mut input_text).expect("Read input");

    println!("length: {}",input_text.trim().len());
    println!("palindrome {} is {}",input_text,is_palindrome(&input_text));

}



