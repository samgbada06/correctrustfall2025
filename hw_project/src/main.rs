

fn borrow_to_mut_watchout() {
    let mut word = "UT".to_string(); 
    fn update(word: &mut String) {
        word.push_str("RGV");
    }
    update(&mut word);
    println!("{word}")
}

fn concat_strings(s1: &String, s2: &String) -> String {
   
    let mut s = s1.clone();
    s.push_str(s2);
    return s
}

fn clone_and_modify(s: &String) -> String {
    let mut clone = s.clone();
    clone.push_str("viana");
    return clone
}

fn sum_into(low: i32, high: i32, result: &mut i32) {
    *result = 0;
    for i in low..=high {
        *result += i;
    }
}

fn main() {
    let mut destination = 0;
    sum_into(1, 5, &mut destination);
    println!("The sum is: {}", destination);
    
