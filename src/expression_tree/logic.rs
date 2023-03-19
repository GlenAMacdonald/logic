pub fn and(a: bool, b: bool) -> bool {
    return a && b;
}

pub fn or(a: bool, b: bool) -> bool {
    return a || b;
}

pub fn implies(a: bool, b: bool) -> bool {
    if !a && !b {return true;}
    else if a && !b {return false;}
    else if !a && b {return true;}
    else {return true;};
}

pub fn equivalent(a:bool, b: bool) -> bool {
    if !a && !b {return true;}
    else if a && !b {return false;}
    else if !a && b {return false;}
    else {return true;};
}

pub fn negative(a:bool) -> bool {
    return !a;
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
        
    #[test]
    fn test_bool_or() {
        assert!(or(true, true));
        assert!(or(true, false));
        assert!(or(false, true));
        assert!(!(or(false, false)));
    }

    #[test]
    fn implies_true() {
        assert!(implies(true, true));
        assert!(implies(false, true));
        assert!(!implies(true, false));
        assert!(implies(false, false));
    }

    #[test]
    fn equivalent_true() {
        assert!(equivalent(true, true));
        assert!(!equivalent(false, true));
        assert!(!equivalent(true, false));
        assert!(equivalent(false, false));
    }

    #[test]
    fn neg_true() {
        assert!(!negative(true));
        assert!(negative(false));
    }
}