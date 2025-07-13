use std::fs::File;
use std::io;
use std::io::Write;

/// Converts pairs of connections between `AVLNode`s in `AVLTree` to graphviz description.
pub fn convert_to_graphviz<T: std::fmt::Display>(
    connections: &[(T, T)],
    filename: &str,
) -> io::Result<()> {
    let mut file = File::create(filename)?;

    writeln!(&mut file, "digraph AVL {{")?;
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
    use crate::avl_tree::AVLTree;
    use std::fs;
    use std::path::Path;

    fn setup() {
        fs::create_dir_all("dots/AVL").unwrap();
    }

    #[test]
    fn basic_tree_graphviz() {
        setup();

        let mut avl_diff_heights_null = AVLTree::new();
        let mut avl_diff_heights_one = AVLTree::new();

        let values_1 = vec![5, 3, 7, 2, 4, 6, 8];
        let values_2 = vec![4, 2, 6, 1, 3, 5];
        for value in &values_1 {
            avl_diff_heights_null.insert(value);
        }
        for value in &values_2 {
            avl_diff_heights_one.insert(value);
        }

        let connections_1 = avl_diff_heights_null.find_connections();
        let connections_2 = avl_diff_heights_one.find_connections();

        convert_to_graphviz(&connections_1, "dots/AVL/avl_diff_heights_null.dot").unwrap();
        convert_to_graphviz(&connections_2, "dots/AVL/avl_diff_heights_one.dot").unwrap();

        assert!(Path::new("dots/AVL/avl_diff_heights_null.dot").exists());
        assert!(Path::new("dots/AVL/avl_diff_heights_one.dot").exists());
    }

    #[test]
    fn degenerate_trees_graphviz() {
        setup();

        let mut avl_degenerate_right = AVLTree::new();
        let mut avl_degenerate_left = AVLTree::new();

        for i in 0..=10 {
            avl_degenerate_right.insert(i);
        }
        for i in (0..=10).rev() {
            avl_degenerate_left.insert(i);
        }

        let connections_1 = avl_degenerate_left.find_connections();
        let connections_2 = avl_degenerate_right.find_connections();

        convert_to_graphviz(&connections_1, "dots/AVL/avl_degenerate_left.dot").unwrap();
        convert_to_graphviz(&connections_2, "dots/AVL/avl_degenerate_right.dot").unwrap();

        assert!(Path::new("dots/AVL/avl_degenerate_right.dot").exists());
        assert!(Path::new("dots/AVL/avl_degenerate_left.dot").exists());
    }

    #[test]
    fn empty_tree_graphviz() {
        setup();

        let avl = AVLTree::<i32>::new();
        let connections = avl.find_connections();
        convert_to_graphviz(&connections, "dots/AVL/empty_tree.dot").unwrap();
        assert!(Path::new("dots/AVL/empty_tree.dot").exists());
    }

    #[test]
    fn check_rebalancing_avl_graphviz() {
        setup();

        let mut avl_rebalancing_1 = AVLTree::new();
        let mut avl_rebalancing_2 = AVLTree::new();
        let mut avl_rebalancing_3 = AVLTree::new();

        let values_1 = vec![4, 2, 6, 1, 3, 5, 7];
        let values_2 = vec![5, 2, 8, 1, 3, 7, 9, 4, 6];
        let values_3 = vec![10, 5, 15, 3, 7, 12, 17, 2, 4];

        for value in &values_1 {
            avl_rebalancing_1.insert(value);
        }
        for value in &values_2 {
            avl_rebalancing_2.insert(value);
        }
        for value in &values_3 {
            avl_rebalancing_3.insert(value);
        }

        let connections_1 = avl_rebalancing_1.find_connections();
        let connections_2 = avl_rebalancing_2.find_connections();
        let connections_3 = avl_rebalancing_3.find_connections();

        convert_to_graphviz(&connections_1, "dots/AVL/avl_rebalancing_1.dot").unwrap();
        convert_to_graphviz(&connections_2, "dots/AVL/avl_rebalancing_2.dot").unwrap();
        convert_to_graphviz(&connections_3, "dots/AVL/avl_rebalancing_3.dot").unwrap();

        assert!(Path::new("dots/AVL/avl_rebalancing_1.dot").exists());
        assert!(Path::new("dots/AVL/avl_rebalancing_2.dot").exists());
        assert!(Path::new("dots/AVL/avl_rebalancing_3.dot").exists());
    }
}
