use std::collections::HashMap;

use crate::expression_tree::logic;

type ChildNode<T> = Option<Box<BTNode<T>>>;
type BChildNode<'a, T> = Option<Box<&'a BBTNode<'a, T>>>;

pub struct BTNode<T> {
    op: Op<T>,
    left: ChildNode<T>,
    right: ChildNode<T>
}

pub struct BBTNode<'a, T> {
    op: Op<T>,
    left: &'a BChildNode<'a, T>,
    right: &'a BChildNode<'a, T>
}

pub enum Op<T> {
    Neg,
    And,
    Or,
    Implies,
    Equivalent,
    Val(T),
    Key(String)
}

// This Binary Tree holding logic expressions also has a truth table
pub struct BinaryTree<T> {
    head: Option<BTNode<T>>,
}

impl BTNode<bool> {
    pub fn new(op: Op<bool>, l: BTNode<bool>, r: BTNode<bool>) -> Self {
        BTNode::<bool> {
            op: op, left: Some(Box::new(l)), right: Some(Box::new(r))
        }
    }
}

pub fn and_node(l: BTNode<bool>, r: BTNode<bool>) -> BTNode<bool> {
    BTNode::new(Op::And, l, r)
}

// pub fn anda_node <'a> (l: &BBTNode<'a, bool>, r: &'a' BBTNode<'a, bool>) -> BBTNode<'a, bool> {
//     BBTNode::<bool> {op: Op::And, left: &Some(Box::new(l)), right: &Some(Box::new(r)) }
// }

pub fn or_node(l: BTNode<bool>, r: BTNode<bool>) -> BTNode<bool> {
    BTNode::new(Op::Or, l, r)
}

// Negative can only have one branch...
pub fn neg_node(l: BTNode<bool>) -> BTNode<bool> {
    BTNode::<bool> { op: Op::Neg, left: Some(Box::new(l)), right: None}
}

pub fn eq_node(l: BTNode<bool>, r: BTNode<bool>) -> BTNode<bool> {
    BTNode::new(Op::Equivalent, l, r)
}

pub fn implies_node(l: BTNode<bool>, r: BTNode<bool>) -> BTNode<bool> {
    BTNode::new(Op::Implies, l, r)
}

pub fn val_node(value: bool) -> BTNode<bool> {
    BTNode::<bool> { op: Op::Val(value), left: None, right: None }
}

pub fn key_node(value: String) -> BTNode<bool> {
    BTNode::<bool> { op: Op::Key(value), left: None, right: None }
}

impl BinaryTree<bool> {
    pub fn new(head: BTNode<bool>) -> Self {
        BinaryTree::<bool> {head: Some(head)}
    }

    pub fn collapse(node: &Box<BTNode<bool>>, truth_table: &mut HashMap<String,bool>) -> bool {
        // l and r are optional boolean variables (which can get updated)
        let mut l: Option<bool> = None;
        let mut r: Option<bool> = None;

        // if the input node 'left' exists then collapse it and set l to be the collapsed boolean value.
        if let Some(left) = &node.left {
            l = Some(BinaryTree::collapse(left, truth_table));
        }

        // if the input node 'right' exists then collapse it and set r to be the collapsed boolean value
        if let Some(right) = &node.right {
            r = Some(BinaryTree::collapse(right, truth_table));
        }

        // This section should only get accessed once the node is a terminating node.

        // If l / r is a boolean, then set l / r to be the boolean value for the final time, with a default value of false.
        let l: bool = if let Some(x) = l { x } else { false };
        let r: bool = if let Some(x) = r { x } else { false };

        // Rust returns the last expression in a function (if there isn't an explicit return)
        match &node.op {
            Op::And => { logic::and(l,r)}
            Op::Or =>  { logic::or(l,r)}
            //  Negative must live on the left branch
            Op::Neg =>  { logic::negative(l)}
            Op::Equivalent => {logic::equivalent(l,r)}
            Op::Implies => {logic::implies(l, r)}
            Op::Val(x) => { *x }
            Op::Key(x) => { match truth_table.get(x) {
                None => {println!("{} was not in the dictionary, returned false", x); return false;}
                Some(truth_value) => {*truth_value}
                }}
         }

    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_and() {
        let mut truth_table = HashMap::new();

        let bt = BinaryTree::new(
            and_node(val_node(true), val_node(true))
        );
        assert!(BinaryTree::collapse(&Box::new(bt.head.expect("No Head")),&mut truth_table));

        let bt = BinaryTree::new(
            and_node(val_node(true), val_node(false))
        );
        assert!(!BinaryTree::collapse(&Box::new(bt.head.expect("No Head")),&mut truth_table));

        let bt = BinaryTree::new(
            and_node(val_node(false), val_node(true))
        );
        assert!(!BinaryTree::collapse(&Box::new(bt.head.expect("No Head")),&mut truth_table));

        let bt = BinaryTree::new(
            and_node(val_node(false), val_node(false))
        );
        assert!(!BinaryTree::collapse(&Box::new(bt.head.expect("No Head")),&mut truth_table));

        let and_keyed_bt = BinaryTree::new(
            and_node(key_node("A".to_string()), key_node("B".to_string()))
        );

        truth_table.insert("A".to_string(), true);
        truth_table.insert("B".to_string(), true);

        // assert!(BinaryTree::collapse(&Box::new(and_keyed_bt.head.as_ref().expect("No Head")),&mut truth_table));

        *truth_table.entry("B".to_string()).or_insert(false) = false;

        assert!(!BinaryTree::collapse(&Box::new(and_keyed_bt.head.expect("No Head")),&mut truth_table));

    }

    #[test]
    fn test_tree_or() {
        let mut truth_table = HashMap::new();

        let bt = BinaryTree::new(
            or_node(val_node(true), val_node(true))
        );
        assert!(BinaryTree::collapse(&Box::new(bt.head.expect("No Head")),&mut truth_table));

        let bt = BinaryTree::new(
            or_node(val_node(true), val_node(false))
        );
        assert!(BinaryTree::collapse(&Box::new(bt.head.expect("No Head")),&mut truth_table));

        let bt = BinaryTree::new(
            or_node(val_node(false), val_node(true))
        );
        assert!(BinaryTree::collapse(&Box::new(bt.head.expect("No Head")),&mut truth_table));

        let bt = BinaryTree::new(
            or_node(val_node(false), val_node(false))
        );
        assert!(!BinaryTree::collapse(&Box::new(bt.head.expect("No Head")),&mut truth_table));
    }

    #[test]
    fn test_tree_implies() {
        let mut truth_table = HashMap::new();

        let bt = BinaryTree::new(
            implies_node(val_node(true), val_node(true))
        );
        assert!(BinaryTree::collapse(&Box::new(bt.head.expect("No Head")),&mut truth_table));

        let bt = BinaryTree::new(
            implies_node(val_node(true), val_node(false))
        );
        assert!(!BinaryTree::collapse(&Box::new(bt.head.expect("No Head")),&mut truth_table));

        let bt = BinaryTree::new(
            implies_node(val_node(false), val_node(true))
        );
        assert!(BinaryTree::collapse(&Box::new(bt.head.expect("No Head")),&mut truth_table));

        let bt = BinaryTree::new(
            implies_node(val_node(false), val_node(false))
        );
        assert!(BinaryTree::collapse(&Box::new(bt.head.expect("No Head")),&mut truth_table));
    }

    #[test]
    fn test_tree_equivalent() {
        let mut truth_table = HashMap::new();

        let bt = BinaryTree::new(
            eq_node(val_node(true), val_node(true))
        );
        assert!(BinaryTree::collapse(&Box::new(bt.head.expect("No Head")),&mut truth_table));

        let bt = BinaryTree::new(
            eq_node(val_node(true), val_node(false))
        );
        assert!(!BinaryTree::collapse(&Box::new(bt.head.expect("No Head")),&mut truth_table));

        let bt = BinaryTree::new(
            eq_node(val_node(false), val_node(true))
        );
        assert!(!BinaryTree::collapse(&Box::new(bt.head.expect("No Head")),&mut truth_table));

        let bt = BinaryTree::new(
            eq_node(val_node(false), val_node(false))
        );
        assert!(BinaryTree::collapse(&Box::new(bt.head.expect("No Head")),&mut truth_table));
    }

    #[test]
    fn test_tree_negative() {
        let mut truth_table = HashMap::new();

        let bt = BinaryTree::new(
            neg_node(val_node(true))
        );
        assert!(!BinaryTree::collapse(&Box::new(bt.head.expect("No Head")),&mut truth_table));

        let bt = BinaryTree::new(
            neg_node(val_node(true))
        );
        assert!(!BinaryTree::collapse(&Box::new(bt.head.expect("No Head")),&mut truth_table));

        let bt = BinaryTree::new(
            neg_node(val_node(false))
        );
        assert!(BinaryTree::collapse(&Box::new(bt.head.expect("No Head")),&mut truth_table));

        let bt = BinaryTree::new(
            neg_node(val_node(false))
        );
        assert!(BinaryTree::collapse(&Box::new(bt.head.expect("No Head")),&mut truth_table));
    }

    #[test]
    fn test_triple_and() {
        let mut truth_table = HashMap::new();

        let bt = BinaryTree::new(
            and_node(
                and_node(val_node(true), val_node(true)), val_node(true))
            );
        assert!(BinaryTree::collapse(&Box::new(bt.head.expect("No Head")),&mut truth_table));

        let bt = BinaryTree::new(
            and_node(
                and_node(val_node(true), val_node(true)), val_node(false))
            );
        assert!(!BinaryTree::collapse(&Box::new(bt.head.expect("No Head")),&mut truth_table));

        let bt = BinaryTree::new(
            and_node(
                and_node(val_node(true), val_node(false)), val_node(true))
            );
        assert!(!BinaryTree::collapse(&Box::new(bt.head.expect("No Head")),&mut truth_table));

        let bt = BinaryTree::new(
            and_node(
                and_node(val_node(false), val_node(true)), val_node(true))
            );
        assert!(!BinaryTree::collapse(&Box::new(bt.head.expect("No Head")),&mut truth_table));
    }
    
    #[test]
    fn test_triple_and_reverse() {
        let mut truth_table = HashMap::new();

        let bt = BinaryTree::new(
            and_node(
                val_node(true), and_node(val_node(true), val_node(true)))
            );
        assert!(BinaryTree::collapse(&Box::new(bt.head.expect("No Head")),&mut truth_table));

        let bt = BinaryTree::new(
            and_node(
                val_node(false), and_node(val_node(true), val_node(true)))
            );
        assert!(!BinaryTree::collapse(&Box::new(bt.head.expect("No Head")),&mut truth_table));

        let bt = BinaryTree::new(
            and_node(
                val_node(true), and_node(val_node(true), val_node(false)))
            );
        assert!(!BinaryTree::collapse(&Box::new(bt.head.expect("No Head")),&mut truth_table));

        let bt = BinaryTree::new(
            and_node(
                val_node(true), and_node(val_node(false), val_node(true)))
            );
        assert!(!BinaryTree::collapse(&Box::new(bt.head.expect("No Head")),&mut truth_table));
    }

    #[test]
    fn test_triple_or() {
        let mut truth_table = HashMap::new();

        let bt = BinaryTree::new(
            or_node(
                or_node(val_node(false), val_node(false)), val_node(false))
            );
        assert!(!BinaryTree::collapse(&Box::new(bt.head.expect("No Head")),&mut truth_table));

        let bt = BinaryTree::new(
            or_node(
                or_node(val_node(true), val_node(false)), val_node(false))
            );
        assert!(BinaryTree::collapse(&Box::new(bt.head.expect("No Head")),&mut truth_table));

        let bt = BinaryTree::new(
            or_node(
                or_node(val_node(false), val_node(true)), val_node(false))
            );
        assert!(BinaryTree::collapse(&Box::new(bt.head.expect("No Head")),&mut truth_table));

        let bt = BinaryTree::new(
            or_node(
                or_node(val_node(false), val_node(false)), val_node(true))
            );
        assert!(BinaryTree::collapse(&Box::new(bt.head.expect("No Head")),&mut truth_table));
    }

    #[test]
    fn test_triple_or_reverse() {
        let mut truth_table = HashMap::new();
        
        let bt = BinaryTree::new(
            or_node(
                val_node(false), or_node(val_node(false), val_node(false)))
            );
        assert!(!BinaryTree::collapse(&Box::new(bt.head.expect("No Head")),&mut truth_table));

        let bt = BinaryTree::new(
            or_node(
                val_node(false), or_node(val_node(true), val_node(false)))
            );
        assert!(BinaryTree::collapse(&Box::new(bt.head.expect("No Head")),&mut truth_table));

        let bt = BinaryTree::new(
            or_node(
                val_node(false), or_node(val_node(false), val_node(true)))
            );
        assert!(BinaryTree::collapse(&Box::new(bt.head.expect("No Head")),&mut truth_table));

        let bt = BinaryTree::new(
            or_node(
                val_node(true), or_node(val_node(false), val_node(false)))
            );
        assert!(BinaryTree::collapse(&Box::new(bt.head.expect("No Head")),&mut truth_table));
    }
}