pub fn and(a: bool, b: bool) -> bool {
    return a && b;
}

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_bool_and() {
        assert!(and(true, true));
        assert!(!(and(true, false)));
        assert!(!(and(false, true)));
        assert!(!(and(false, false)));
    }
}