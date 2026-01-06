use std::fs::File;
use std::io;
use std::io::Write;

/// Converts pairs of connections between `RBNode`s in `RedBlackTree` to graphviz description.
///
/// This is a simple version that shows connections without colors.
pub fn convert_to_graphviz<T: std::fmt::Display>(
    connections: &[(T, T)],
    filename: &str,
) -> io::Result<()> {
    let mut file = File::create(filename)?;

    writeln!(&mut file, "digraph RBT {{")?;
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
    use crate::red_black_tree::RedBlackTree;
    use std::fs;
    use std::path::Path;

    fn setup() {
        fs::create_dir_all("dots/RBT").unwrap();
    }

    #[test]
    fn basic_tree_graphviz() {
        setup();

        let mut rbt = RedBlackTree::new();
        let values = vec![7, 3, 18, 10, 22, 8, 11, 26];

        for value in &values {
            rbt.insert(*value);
        }

        let connections = rbt.find_connections();
        convert_to_graphviz(&connections, "dots/RBT/rbt_basic.dot").unwrap();
        assert!(Path::new("dots/RBT/rbt_basic.dot").exists());
    }

    #[test]
    fn sequential_insert_graphviz() {
        setup();

        let mut rbt_ascending = RedBlackTree::new();
        let mut rbt_descending = RedBlackTree::new();

        for i in 1..=10 {
            rbt_ascending.insert(i);
        }
        for i in (1..=10).rev() {
            rbt_descending.insert(i);
        }

        let connections_asc = rbt_ascending.find_connections();
        let connections_desc = rbt_descending.find_connections();

        convert_to_graphviz(&connections_asc, "dots/RBT/rbt_sequential_ascending.dot").unwrap();
        convert_to_graphviz(&connections_desc, "dots/RBT/rbt_sequential_descending.dot").unwrap();

        assert!(Path::new("dots/RBT/rbt_sequential_ascending.dot").exists());
        assert!(Path::new("dots/RBT/rbt_sequential_descending.dot").exists());
    }

    #[test]
    fn empty_tree_graphviz() {
        setup();

        let rbt = RedBlackTree::<i32>::new();
        let connections = rbt.find_connections();
        convert_to_graphviz(&connections, "dots/RBT/empty_tree.dot").unwrap();
        assert!(Path::new("dots/RBT/empty_tree.dot").exists());
    }

    #[test]
    fn after_deletions_graphviz() {
        setup();

        let mut rbt = RedBlackTree::new();
        let values = vec![7, 3, 18, 10, 22, 8, 11, 26, 2, 6, 13];

        for value in &values {
            rbt.insert(*value);
        }

        // Remove some values
        rbt.remove(&3);
        rbt.remove(&18);
        rbt.remove(&11);

        let connections = rbt.find_connections();
        convert_to_graphviz(&connections, "dots/RBT/rbt_after_deletions.dot").unwrap();
        assert!(Path::new("dots/RBT/rbt_after_deletions.dot").exists());
    }
}
