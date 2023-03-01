// convert expects something in the form a & b
// Stripped completely of parenthesis

pub struct LogicObject <'a> {
    a: &'a str,
    b: &'a str,
    operator: &'a str
}

pub fn get_logic_operator(input_string: &str) -> &str {
    if input_string.contains("&") { return "&" }
    else if input_string.contains("=>") { return "=>" }
    else if input_string.contains("|") { return "|" }
    else { return "" }
}

pub fn convert(input_string: &str) -> LogicObject {
    let logic_operator = get_logic_operator(input_string);
    let attributes: Vec<&str> = input_string.split(logic_operator).collect();
    let logic_object: LogicObject = LogicObject { 
        a: attributes[0].trim(), 
        b: attributes[1].trim(), 
        operator: logic_operator };
    return logic_object;
}

pub fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_logic_operator() {
        assert_eq!("|", get_logic_operator("a | b"));
        assert_eq!("&", get_logic_operator("a & b"));
        assert_eq!("=>", get_logic_operator("a => b"));
    }

    #[test]
    fn test_convert_a_and_b() {
        let object_a: LogicObject = convert("a & b");
        assert_eq!("a", object_a.a);
        assert_eq!("b", object_a.b);
        assert_eq!("&", object_a.operator);
    }

    #[test]
    fn test_convert_apple_and_candy() {
        let object_a: LogicObject = convert("apple & candy");
        assert_eq!("apple", object_a.a);
        assert_eq!("candy", object_a.b);
        assert_eq!("&", object_a.operator);
    }

    #[test]
    fn test_convert_apple_or_candy() {
        let object_a: LogicObject = convert("apple | candy");
        assert_eq!("apple", object_a.a);
        assert_eq!("candy", object_a.b);
        assert_eq!("|", object_a.operator);
    }

    #[test]
    fn test_convert_apple_implies_candy() {
        let object_a: LogicObject = convert("apple => candy");
        assert_eq!("apple", object_a.a);
        assert_eq!("candy", object_a.b);
        assert_eq!("=>", object_a.operator);
    }
}