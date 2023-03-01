pub fn implies(a: bool, b: bool) -> bool {
    if !a && !b {return true;}
    else if a && !b {return false;}
    else if !a && b {return true;}
    else {return true;};
}

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn implies_true() {
        assert!(implies(true, true));
        assert!(implies(false, true));
        assert!(!implies(true, false));
        assert!(implies(false, false));
    }
}