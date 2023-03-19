use crate::expression_tree::logic_expression_tree::*;

pub fn postfix_to_expression_tree(postfix: String) -> BinaryTree<bool> {
    let bt = BinaryTree::new(
        implies_node(val_node(true), val_node(true))
    );
    return bt;
}


#[cfg(test)]
mod tests {

    #[test]
    fn test_and(){

    }
}