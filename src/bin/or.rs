pub fn or(a: bool, b: bool) -> bool {
    return a || b;
}

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_bool_or() {
        assert!(or(true, true));
        assert!(or(true, false));
        assert!(or(false, true));
        assert!(!(or(false, false)));
    }
}