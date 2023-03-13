pub fn check(input_string: &str) -> bool {
    let n_left_brackets: usize = input_string.chars().filter(|c| *c == '[').count();
    let n_right_brackets: usize = input_string.chars().filter(|c| *c == ']').count();
    
    let mut n_operators: usize = input_string.chars().filter(|c| *c == '&').count();
    n_operators += input_string.chars().filter(|c| *c == '|').count();
    n_operators += input_string.chars().filter(|c| *c == '>').count();

    if n_left_brackets != n_right_brackets {
        return false;
    } 

    if n_operators != n_left_brackets {
        return false;
    }

    return true;
}

pub fn main() {
    
}