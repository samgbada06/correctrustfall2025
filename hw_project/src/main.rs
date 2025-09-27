

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

fn is_even(n: i32) -> bool {
    if n % 2 == 0 {
        true
    } else {
        false
    }
}

fn check_guess(guess: i32, secret: i32) -> i32 {
    let secret = 64;
    if guess > secret {
        return 1
    }
    else if guess < secret {
        return -1
    }
    else {
        return 0
    }
}

fn fahrenheit_to_celsius(f: f64) -> f64{
    let c = (f - 32.0) * 5.0 / 9.0;
    return c
}
fn celsius_to_fahrenheit(c: f64) -> f64{
    let f = (c * 9.0 / 5.0) + 32.0;
    return f
}

fn main() {
    let fahrFreeze = 32.0;
    let mut temp = 32.0;
    let mut i = 0;

    loop{
        let result = fahrenheit_to_celsius(temp);
        println!("{}", result);
        i = 1 + i;
        temp = temp + 1.0;
        if i == 5{break;}
    }
}

