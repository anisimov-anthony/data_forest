use std::fs::File;
use std::io;
use std::io::Write;

/// Converts pairs of connections between `BinaryNode`s in `BinarySearchTree` to graphviz description.
pub fn convert_to_graphviz<T: std::fmt::Display>(
    connections: &[(T, T)],
    filename: &str,
) -> io::Result<()> {
    let mut file = File::create(filename)?;

    writeln!(&mut file, "digraph BST {{")?;
    writeln!(&mut file, "    node [shape=circle];")?;

    for (parent, child) in connections {
        writeln!(&mut file, "    {parent} -> {child};")?;
    }

    writeln!(&mut file, "}}")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binary_search_tree::BinarySearchTree;
    use std::fs;
    use std::path::Path;

    fn setup() {
        fs::create_dir_all("dots/BST").unwrap();
    }

    #[test]
    fn basic_tree_graphviz() {
        setup();

        let mut bst_diff_heights_null = BinarySearchTree::new();
        let mut bst_diff_heights_one = BinarySearchTree::new();
        let mut bst_diff_heights_two = BinarySearchTree::new();

        let values_1 = vec![5, 3, 7, 2, 4, 6, 8];
        let values_2 = vec![4, 2, 6, 1, 3, 5];
        let values_3 = vec![8, 4, 12, 2, 6, 10, 1, 7];
        for value in &values_1 {
            bst_diff_heights_null.insert(value);
        }
        for value in &values_2 {
            bst_diff_heights_one.insert(value);
        }
        for value in &values_3 {
            bst_diff_heights_two.insert(value);
        }

        let connections_1 = bst_diff_heights_null.find_connections();
        let connections_2 = bst_diff_heights_one.find_connections();
        let connections_3 = bst_diff_heights_two.find_connections();

        convert_to_graphviz(&connections_1, "dots/BST/bst_diff_heights_null.dot").unwrap();
        convert_to_graphviz(&connections_2, "dots/BST/bst_diff_heights_one.dot").unwrap();
        convert_to_graphviz(&connections_3, "dots/BST/bst_diff_heights_two.dot").unwrap();

        assert!(Path::new("dots/BST/bst_diff_heights_null.dot").exists());
        assert!(Path::new("dots/BST/bst_diff_heights_one.dot").exists());
        assert!(Path::new("dots/BST/bst_diff_heights_two.dot").exists());
    }

    #[test]
    fn degenerate_trees_graphviz() {
        setup();

        let mut bst_degenerate_right = BinarySearchTree::new();
        let mut bst_degenerate_left = BinarySearchTree::new();

        for i in 0..=10 {
            bst_degenerate_right.insert(i);
        }
        for i in (0..=10).rev() {
            bst_degenerate_left.insert(i);
        }

        let connections_1 = bst_degenerate_left.find_connections();
        let connections_2 = bst_degenerate_right.find_connections();

        convert_to_graphviz(&connections_1, "dots/BST/bst_degenerate_left.dot").unwrap();
        convert_to_graphviz(&connections_2, "dots/BST/bst_degenerate_right.dot").unwrap();

        assert!(Path::new("dots/BST/bst_degenerate_right.dot").exists());
        assert!(Path::new("dots/BST/bst_degenerate_left.dot").exists());
    }

    #[test]
    fn empty_tree_graphviz() {
        setup();

        let bst = BinarySearchTree::<i32>::new();
        let connections = bst.find_connections();
        convert_to_graphviz(&connections, "dots/BST/empty_tree.dot").unwrap();
        assert!(Path::new("dots/BST/empty_tree.dot").exists());
    }
}
