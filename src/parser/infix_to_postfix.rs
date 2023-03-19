use std::{collections::HashMap};

// Note that this only works with binary operators w/o Parenthesis atm.

fn generate_input_stack_precedence_map() -> HashMap<char,HashMap<&'static str,u8>>{

    let close_precedence: HashMap<&str,u8> =    [("Stack",0),("Input",0)].into(); // "Stack" = 0 as it's never going to be in the stack.
    let open_precedence:  HashMap<&str,u8> =    [("Stack",0),("Input",10)].into();
    let neg_precedence: HashMap<&str,u8> =      [("Stack",9),("Input",8)].into();
    let and_precedence: HashMap<&str,u8> =      [("Stack",7),("Input",6)].into();
    let or_precedence: HashMap<&str,u8> =       [("Stack",5),("Input",4)].into();
    let implies_precedence: HashMap<&str,u8> =  [("Stack",3),("Input",2)].into();
    let iff_precedence: HashMap<&str,u8> =      [("Stack",1),("Input",0)].into();

    let mut operator_precedence: HashMap<char,HashMap<&str,u8>> = HashMap::new();
    operator_precedence.insert(']', close_precedence);
    operator_precedence.insert('[', open_precedence);
    operator_precedence.insert('!', neg_precedence);
    operator_precedence.insert('&', and_precedence);
    operator_precedence.insert('|', or_precedence);
    operator_precedence.insert('>', implies_precedence);
    operator_precedence.insert('~', iff_precedence);

    return operator_precedence;
}

/*
fn generate_precendence_map() -> HashMap<char,HashMap<char,char>>{
    let neg_precedence:     HashMap<char,char> = [('!','r'),('&','l'),('|','l'),('>','l'),('~','l')].into();
    let and_precedence:     HashMap<char,char> = [('!','r'),('&','l'),('|','l'),('>','l'),('~','l')].into();
    let or_precedence:      HashMap<char,char> = [('!','r'),('&','r'),('|','l'),('>','l'),('~','l')].into();
    let implies_precedence: HashMap<char,char> = [('!','r'),('&','r'),('|','r'),('>','l'),('~','l')].into();
    let iff_precedence:     HashMap<char,char> = [('!','r'),('&','r'),('|','r'),('>','r'),('~','l')].into();
    
    let mut operator_precedence: HashMap<char,HashMap<char,char>> = HashMap::new();
    operator_precedence.insert('!', neg_precedence);
    operator_precedence.insert('&', and_precedence);
    operator_precedence.insert('|', or_precedence);
    operator_precedence.insert('>', implies_precedence);
    operator_precedence.insert('~', iff_precedence);

    return operator_precedence;
}
*/

fn infix_to_postfix(input_string: &str) -> String {
    let mut output: Vec<char> = Vec::with_capacity(input_string.len()+2);
    let mut stack: Vec<char> = Vec::with_capacity(input_string.len());
    let operators: [char; 7] = ['!','&','|','>','~','[',']'];
    let order_precedence = generate_input_stack_precedence_map();
    
    stack.push('[');
    let mut modified_string = String::from(input_string);
    modified_string.push(']');

    for character in modified_string.chars() {
        if !operators.contains(&character) {
            output.push(character);
        } else {
            let input_p = order_precedence[&character]["Input"];
            let mut found = false;
            while !found && !stack.is_empty() {
                let stack_p = order_precedence[&stack.last().unwrap()]["Stack"];
                if input_p <= stack_p {
                    let output_char = stack.pop().unwrap();
                    if output_char != '[' {
                        output.push(output_char);     
                    } else if character == ']' {
                        found = true;
                    }
                } else {
                    if character != ']' {
                        stack.push(character);
                    }
                    found = true;

                }
            }
        }
    }

    return output.iter().collect();
}

#[cfg(test)]
mod tests {
    use super::*;
    
/*
    #[test]
    fn check_order_precedence(){
        let order_p: HashMap<char,HashMap<char,char>> = generate_precendence_map();
        assert_eq!('r',order_p[&'!'][&'!']);
        assert_eq!('l',order_p[&'!'][&'&']);
        assert_eq!('l',order_p[&'!'][&'|']);
        assert_eq!('l',order_p[&'!'][&'>']);
        assert_eq!('l',order_p[&'!'][&'~']);
        assert_eq!('r',order_p[&'&'][&'!']);
        assert_eq!('l',order_p[&'&'][&'&']);
        assert_eq!('l',order_p[&'&'][&'|']);
        assert_eq!('l',order_p[&'&'][&'>']);
        assert_eq!('l',order_p[&'&'][&'~']);
        assert_eq!('r',order_p[&'|'][&'!']);
        assert_eq!('r',order_p[&'|'][&'&']);
        assert_eq!('l',order_p[&'|'][&'|']);
        assert_eq!('l',order_p[&'|'][&'>']);
        assert_eq!('l',order_p[&'|'][&'~']);
        assert_eq!('r',order_p[&'>'][&'!']);
        assert_eq!('r',order_p[&'>'][&'&']);
        assert_eq!('r',order_p[&'>'][&'|']);
        assert_eq!('l',order_p[&'>'][&'>']);
        assert_eq!('l',order_p[&'>'][&'~']);
        assert_eq!('r',order_p[&'~'][&'!']);
        assert_eq!('r',order_p[&'~'][&'&']);
        assert_eq!('r',order_p[&'~'][&'|']);
        assert_eq!('r',order_p[&'~'][&'>']);
        assert_eq!('l',order_p[&'~'][&'~']);
    }
 */

    #[test]
    fn check_a_and_b(){
        let postfix = infix_to_postfix("A&B");
        println!("{}",postfix);
        assert_eq!("AB&",postfix);
    }

    #[test]
    fn check_a_and_b_and_c(){
        let postfix = infix_to_postfix("A&B&C");
        println!("{}",postfix);
        assert_eq!("AB&C&",postfix);
    }

    #[test]
    fn check_a_and_b_or_c(){
        let postfix = infix_to_postfix("A&B|C");
        println!("{}",postfix);
        assert_eq!("AB&C|",postfix);
    }

    #[test]
    fn check_a_and_b_or_c_and_d(){
        let postfix = infix_to_postfix("A&B|C&D");
        println!("{}",postfix);
        assert_eq!("AB&CD&|",postfix);
    }

    #[test]
    fn check_not_a_and_b_or_c_and_d(){
        let postfix = infix_to_postfix("![A&B|C&D]");
        println!("{}",postfix);
        assert_eq!("AB&CD&|!",postfix);
    }

    #[test]
    fn check_not_a_and_b_or_not_c_and_d(){
        let postfix = infix_to_postfix("![A&B]|![C&D]");
        println!("{}",postfix);
        assert_eq!("AB&!CD&!|",postfix);
    }

    #[test]
    fn check_not_a_and_b_and_c_or_d_and_e_and_f(){
        let postfix = infix_to_postfix("![A&B&C]|[D&E&F]");
        println!("{}",postfix);
        assert_eq!("AB&C&!DE&F&|",postfix);
    }
}