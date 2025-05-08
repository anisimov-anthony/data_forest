use super::binary_search_tree::BinarySearchTree;
use std::fs::File;
use std::io;
use std::io::Write;

pub fn find_connections<T: PartialOrd + Clone>(bst: &BinarySearchTree<T>) -> Vec<(&T, &T)> {
    let mut result = Vec::new();
    let mut queue = std::collections::VecDeque::new();

    if let Some(root) = &bst.root {
        queue.push_back(root);
    }

    while let Some(node) = queue.pop_front() {
        if let Some(left) = &node.left {
            queue.push_back(left);
            result.push((&node.value, &left.value));
        }
        if let Some(right) = &node.right {
            queue.push_back(right);
            result.push((&node.value, &right.value));
        }
    }

    result
}

pub fn convert_to_graphviz<T: std::fmt::Display>(
    connections: &[(T, T)],
    filename: &str,
) -> io::Result<()> {
    let mut file = File::create(filename)?;

    writeln!(&mut file, "digraph BST {{")?;
    writeln!(&mut file, "    node [shape=circle];")?;

    for (parent, child) in connections {
        writeln!(&mut file, "    {} -> {};", parent, child)?;
    }

    writeln!(&mut file, "}}")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    fn setup() {
        fs::create_dir_all("dots").unwrap();
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

        let connections_1 = find_connections(&bst_diff_heights_null);
        let connections_2 = find_connections(&bst_diff_heights_one);
        let connections_3 = find_connections(&bst_diff_heights_two);

        convert_to_graphviz(&connections_1, "dots/bst_diff_heights_null.dot").unwrap();
        convert_to_graphviz(&connections_2, "dots/bst_diff_heights_one.dot").unwrap();
        convert_to_graphviz(&connections_3, "dots/bst_diff_heights_two.dot").unwrap();

        assert!(Path::new("dots/bst_diff_heights_null.dot").exists());
        assert!(Path::new("dots/bst_diff_heights_one.dot").exists());
        assert!(Path::new("dots/bst_diff_heights_two.dot").exists());
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

        let connections_1 = find_connections(&bst_degenerate_left);
        let connections_2 = find_connections(&bst_degenerate_right);

        convert_to_graphviz(&connections_1, "dots/bst_degenerate_left.dot").unwrap();
        convert_to_graphviz(&connections_2, "dots/bst_degenerate_right.dot").unwrap();

        assert!(Path::new("dots/bst_degenerate_right.dot").exists());
        assert!(Path::new("dots/bst_degenerate_left.dot").exists());
    }

    #[test]
    fn empty_tree_graphviz() {
        setup();

        let bst = BinarySearchTree::<i32>::new();
        let connections = find_connections(&bst);
        convert_to_graphviz(&connections, "dots/empty_tree.dot").unwrap();
        assert!(Path::new("dots/empty_tree.dot").exists());
    }
}
