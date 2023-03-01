struct FromToString<'a>{
    from: &'a str,
    to: &'a str
}

pub fn standardize(input_string: &str) -> String {


    let mut replacements: Vec<FromToString> = vec![];

    replacements.push(FromToString{from:"^",        to:"&"});
    replacements.push(FromToString{from:"and",      to:"&"});
    replacements.push(FromToString{from:"or",       to:"|"});
    replacements.push(FromToString{from:"implies",  to:"=>"});
    replacements.push(FromToString{from:"{",        to:"["});
    replacements.push(FromToString{from:"(",        to:"["});
    replacements.push(FromToString{from:"}",        to:"]"});
    replacements.push(FromToString{from:")",        to:"]"});

    let mut return_string = String::from(input_string).to_lowercase();
    for pair in replacements {
        return_string = return_string.replace(&pair.from, &pair.to);
    }

    return return_string;
}

// pub fn interpret(input_string: str) -> bool {
    
//     let mut splitString = input_string.split('&');

//     return true;
// }

fn main() {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standardize_string() {
        assert_eq!("a & b",   standardize("a ^ b"));
        assert_eq!("a & b",   standardize("a aNd b"));
        assert_eq!("a | b",   standardize("a or b"));
        assert_eq!("a | b",   standardize("a OR b"));
        assert_eq!("a => b",  standardize("a implies b"));
        assert_eq!("a => b",  standardize("a iMplIeS b"));
        assert_eq!("[a & b]", standardize("(a ^ b)"));
        assert_eq!("[a & b]", standardize("{a ^ b}"));
        assert_eq!("[a & b]", standardize("[a ^ b]"));
        assert_eq!("[a & b]", standardize("(a AND b)"));
    }
}