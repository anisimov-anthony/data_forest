use data_forest::red_black_tree::RedBlackTree;

use proptest::prelude::*;
use std::collections::HashSet;

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 111,
        ..ProptestConfig::default()
    })]
    #[test]
    fn prop_insert_contains(values in prop::collection::vec(any::<i32>(), 1..111)) {
        let mut rbt = RedBlackTree::new();

        for &v in &values {
            rbt.insert(v);
        }

        for &v in &values {
            assert!(rbt.contains(&v));
        }
    }
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 111,
        ..ProptestConfig::default()
    })]
    #[test]
    fn prop_insert_maintains_properties(values in prop::collection::vec(any::<i32>(), 1..111)) {
        let mut rbt = RedBlackTree::new();

        for &v in &values {
            rbt.insert(v);
            assert!(rbt.is_valid_red_black_tree(), "RB properties violated after inserting {}", v);
            assert!(rbt.is_valid_bst(), "BST property violated after inserting {}", v);
        }
    }
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 111,
        ..ProptestConfig::default()
    })]
    #[test]
    fn prop_remove_check_min_max_updating(values in prop::collection::vec(any::<i32>(), 1..100)) {
        let mut rbt = RedBlackTree::new();
        let unique_values: Vec<i32> = values.iter().cloned().collect::<HashSet<_>>().into_iter().collect();
        let mut remaining = unique_values.clone();

        for &v in &values {
            rbt.insert(v);
        }

        for &v in &values {
            rbt.remove(&v);
            remaining.retain(|&x| x != v);

            assert!(!rbt.contains(&v));
            assert_eq!(rbt.min(), remaining.iter().min());
            assert_eq!(rbt.max(), remaining.iter().max());
        }
    }
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 111,
        ..ProptestConfig::default()
    })]
    #[test]
    fn prop_remove_maintains_properties(values in prop::collection::vec(any::<i32>(), 1..100)) {
        let mut rbt = RedBlackTree::new();

        for &v in &values {
            rbt.insert(v);
        }

        for &v in &values {
            rbt.remove(&v);
            assert!(!rbt.contains(&v));
            assert!(rbt.is_valid_red_black_tree(), "RB properties violated after removing {}", v);
            assert!(rbt.is_valid_bst(), "BST property violated after removing {}", v);
        }
    }
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 111,
        ..ProptestConfig::default()
    })]
    #[test]
    fn prop_min_check_updating(values in prop::collection::vec(any::<i32>(), 1..100)) {
        let mut rbt = RedBlackTree::new();
        let mut current_min = None;

        for &v in &values {
            rbt.insert(v);
            current_min = Some(v)
                .filter(|&x| current_min.is_none_or(|min| x < min))
                .or(current_min);
            assert_eq!(rbt.min(), current_min.as_ref());
        }
    }
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 111,
        ..ProptestConfig::default()
    })]
    #[test]
    fn prop_max_check_updating(values in prop::collection::vec(any::<i32>(), 1..100)) {
        let mut rbt = RedBlackTree::new();
        let mut current_max = None;

        for &v in &values {
            rbt.insert(v);
            current_max = Some(v)
                .filter(|&x| current_max.is_none_or(|max| x > max))
                .or(current_max);
            assert_eq!(rbt.max(), current_max.as_ref());
        }
    }
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 111,
        ..ProptestConfig::default()
    })]
    #[test]
    fn prop_max_min_are_similar_for_single_element_tree(value in any::<i32>()) {
        let mut rbt = RedBlackTree::new();
        rbt.insert(value);

        assert!(rbt.min() == rbt.max() && rbt.min() == Some(&value));
    }
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 111,
        ..ProptestConfig::default()
    })]
    #[test]
    fn prop_height_is_logarithmic(values in prop::collection::vec(any::<i32>(), 1..111)) {
        let mut rbt = RedBlackTree::new();

        for &v in &values {
            rbt.insert(v);
        }

        let unique_count = values.iter().collect::<HashSet<_>>().len();
        if unique_count > 0 {
            // Red-Black tree height should be at most 2 * log2(n + 1)
            let max_height = 2.0 * ((unique_count + 1) as f64).log2().ceil();
            assert!(rbt.height() as f64 <= max_height,
                "Height {} exceeds maximum {} for {} unique elements",
                rbt.height(), max_height, unique_count);
        }
    }
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 111,
        ..ProptestConfig::default()
    })]
    #[test]
    fn prop_in_order_is_sorted(values in prop::collection::vec(any::<i32>(), 1..111)) {
        let mut rbt = RedBlackTree::new();

        for &v in &values {
            rbt.insert(v);
        }

        let in_order = rbt.in_order();

        // Check that in-order traversal is sorted
        for i in 1..in_order.len() {
            assert!(in_order[i - 1] < in_order[i],
                "In-order traversal is not sorted: {:?}", in_order);
        }
    }
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 111,
        ..ProptestConfig::default()
    })]
    #[test]
    fn prop_pre_order(values in prop::collection::vec(any::<i32>(), 1..111)) {
        let mut rbt = RedBlackTree::new();

        for &v in &values {
            rbt.insert(v);
        }

        let pre_order = rbt.pre_order();
        let unique_count = values.iter().collect::<HashSet<_>>().len();

        assert_eq!(pre_order.len(), unique_count);
    }
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 111,
        ..ProptestConfig::default()
    })]
    #[test]
    fn prop_post_order(values in prop::collection::vec(any::<i32>(), 1..111)) {
        let mut rbt = RedBlackTree::new();

        for &v in &values {
            rbt.insert(v);
        }

        let post_order = rbt.post_order();
        let unique_count = values.iter().collect::<HashSet<_>>().len();

        assert_eq!(post_order.len(), unique_count);
    }
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 111,
        ..ProptestConfig::default()
    })]
    #[test]
    fn prop_level_order(values in prop::collection::vec(any::<i32>(), 1..111)) {
        let mut rbt = RedBlackTree::new();

        for &v in &values {
            rbt.insert(v);
        }

        let level_order = rbt.level_order();
        let unique_count = values.iter().collect::<HashSet<_>>().len();

        assert_eq!(level_order.len(), unique_count);
    }
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 111,
        ..ProptestConfig::default()
    })]
    #[test]
    fn prop_number_of_elements(values in prop::collection::vec(any::<i32>(), 1..111)) {
        let mut rbt = RedBlackTree::new();

        for &v in &values {
            rbt.insert(v);
        }

        assert_eq!(rbt.number_of_elements(), values.iter().collect::<std::collections::HashSet<_>>().len());
    }
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 111,
        ..ProptestConfig::default()
    })]
    #[test]
    fn prop_ceil(values in prop::collection::vec(any::<i32>(), 1..111)) {
        let mut rbt = RedBlackTree::new();
        for &v in &values {
            rbt.insert(v);
        }

        let unique_values: Vec<i32> = values.into_iter().collect::<HashSet<_>>().into_iter().collect();

        for &v in &unique_values {
            assert_eq!(rbt.ceil(&v), Some(&v));
        }

        let test_points = {
            let mut points = Vec::new();
            points.push(i32::MIN);
            points.extend(unique_values.iter().cloned());
            let mut sorted_values = unique_values.clone();
            sorted_values.sort();
            for window in sorted_values.windows(2) {
                if window[1] > window[0] + 1 {
                    points.push(window[0] + 1);
                }
            }

            points.push(i32::MAX);
            points
        };

        for &i in &test_points {
            let expected = unique_values.iter()
            .filter(|&&x| x >= i)
            .min()
            .copied();
        assert_eq!(rbt.ceil(&i), expected.as_ref());
        }
    }
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 111,
        ..ProptestConfig::default()
    })]
    #[test]
    fn prop_floor(values in prop::collection::vec(any::<i32>(), 1..111)) {
        let mut rbt = RedBlackTree::new();
        for &v in &values {
            rbt.insert(v);
        }

        let unique_values: Vec<i32> = values.into_iter().collect::<HashSet<_>>().into_iter().collect();

        for &v in &unique_values {
            assert_eq!(rbt.floor(&v), Some(&v));
        }

        let test_points = {
            let mut points = Vec::new();
            points.push(i32::MIN);
            points.extend(unique_values.iter().cloned());
            let mut sorted_values = unique_values.clone();
            sorted_values.sort();
            for window in sorted_values.windows(2) {
                if window[1] > window[0] + 1 {
                    points.push(window[0] + 1);
                }
            }

            points.push(i32::MAX);
            points
        };

        for &i in &test_points {
            let expected = unique_values.iter()
            .filter(|&&x| x <= i)
            .max()
            .copied();
        assert_eq!(rbt.floor(&i), expected.as_ref());
        }
    }
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 111,
        ..ProptestConfig::default()
    })]
    #[test]
    fn prop_sequential_insert_maintains_balance(n in 1usize..50) {
        let mut rbt = RedBlackTree::new();

        for i in 1..=n {
            rbt.insert(i);
        }

        assert!(rbt.is_valid_red_black_tree());
        assert!(rbt.is_valid_bst());

        // Height should be logarithmic even for sequential inserts
        let max_height = 2.0 * ((n + 1) as f64).log2().ceil();
        assert!(rbt.height() as f64 <= max_height);
    }
}
