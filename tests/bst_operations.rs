use data_forest::binary_search_tree::BinarySearchTree;

use bst_rs::{BinarySearchTree as BinarySearchTreeOther, IterativeBST as IterativeBSTOther};
use proptest::prelude::*;
use std::collections::HashSet;

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 111,
        ..ProptestConfig::default()
    })]
    #[test]
    fn prop_insert_contains(values in prop::collection::vec(any::<i32>(), 1..111)) {
        let mut bst = BinarySearchTree::new();

        for &v in &values {
            bst.insert(v);
        }

        for &v in &values {
            assert!(bst.contains(&v));
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
        let mut bst = BinarySearchTree::new();
        let unique_values: Vec<i32> = values.iter().cloned().collect::<HashSet<_>>().into_iter().collect();
        let mut remaining = unique_values.clone();

        for &v in &values {
            bst.insert(v);
        }

        for &v in &values {
            bst.remove(&v);
            remaining.retain(|&x| x != v);

            assert!(!bst.contains(&v));
            assert_eq!(bst.min(), remaining.iter().min());
            assert_eq!(bst.max(), remaining.iter().max());
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
        let mut bst = BinarySearchTree::new();
        let mut current_min = None;

        for &v in &values {
            bst.insert(v);
            current_min = Some(v)
                .filter(|&x| current_min.map_or(true, |min| x < min))
                .or(current_min);
            assert_eq!(bst.min(), current_min.as_ref());
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
        let mut bst = BinarySearchTree::new();
        let mut current_max = None;

        for &v in &values {
            bst.insert(v);
            current_max = Some(v)
                .filter(|&x| current_max.map_or(true, |max| x > max))
                .or(current_max);
            assert_eq!(bst.max(), current_max.as_ref());
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
        let mut bst = BinarySearchTree::new();
        bst.insert(value);

        assert!(bst.min() == bst.max() && bst.min() == Some(&value));
    }
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 111,
        ..ProptestConfig::default()
    })]
    #[test]
    fn prop_height(values in prop::collection::vec(any::<i32>(), 1..111)) {
        let mut bst = BinarySearchTree::new();
        let mut bst_comparing = IterativeBSTOther::new();

        for &v in &values {
            bst.insert(v);
            bst_comparing.insert(v);
        }

        if values.is_empty() {
            assert_eq!(bst.height(), 0);
        } else {
            assert_eq!(bst.height(), bst_comparing.height().unwrap_or(0) as usize);
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
        let mut bst = BinarySearchTree::new();
        let mut bst_comparing = IterativeBSTOther::new();

        for &v in &values {
            bst.insert(v);
            bst_comparing.insert(v);
        }

        assert_eq!(bst.pre_order(), bst_comparing.pre_order_vec());
    }
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 111,
        ..ProptestConfig::default()
    })]
    #[test]
    fn prop_in_order(values in prop::collection::vec(any::<i32>(), 1..111)) {
        let mut bst = BinarySearchTree::new();
        let mut bst_comparing = IterativeBSTOther::new();

        for &v in &values {
            bst.insert(v);
            bst_comparing.insert(v);
        }

        assert_eq!(bst.in_order(), bst_comparing.in_order_vec());
    }
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 111,
        ..ProptestConfig::default()
    })]
    #[test]
    fn prop_post_order(values in prop::collection::vec(any::<i32>(), 1..111)) {
        let mut bst = BinarySearchTree::new();
        let mut bst_comparing = IterativeBSTOther::new();

        for &v in &values {
            bst.insert(v);
            bst_comparing.insert(v);
        }

        assert_eq!(bst.post_order(), bst_comparing.post_order_vec());
    }
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 111,
        ..ProptestConfig::default()
    })]
    #[test]
    fn prop_level_order(values in prop::collection::vec(any::<i32>(), 1..111)) {
        let mut bst = BinarySearchTree::new();
        let mut bst_comparing = IterativeBSTOther::new();

        for &v in &values {
            bst.insert(v);
            bst_comparing.insert(v);
        }

        assert_eq!(bst.level_order(), bst_comparing.level_order_vec());
    }
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 111,
        ..ProptestConfig::default()
    })]
    #[test]
    fn prop_number_of_elements(values in prop::collection::vec(any::<i32>(), 1..111)) {
        let mut bst = BinarySearchTree::new();

        for &v in &values {
            bst.insert(v);
        }

        assert_eq!(bst.number_of_elements(), values.iter().collect::<std::collections::HashSet<_>>().len());
    }
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 111,
        ..ProptestConfig::default()
    })]
    #[test]
    fn prop_ceil(values in prop::collection::vec(any::<i32>(), 1..111)) {
        let mut bst = BinarySearchTree::new();
        for &v in &values {
            bst.insert(v);
        }

        let unique_values: Vec<i32> = values.into_iter().collect::<HashSet<_>>().into_iter().collect();

        for &v in &unique_values {
            assert_eq!(bst.ceil(&v), Some(&v));
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
        assert_eq!(bst.ceil(&i), expected.as_ref());
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
        let mut bst = BinarySearchTree::new();
        for &v in &values {
            bst.insert(v);
        }

        let unique_values: Vec<i32> = values.into_iter().collect::<HashSet<_>>().into_iter().collect();

        for &v in &unique_values {
            assert_eq!(bst.floor(&v), Some(&v));
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
        assert_eq!(bst.floor(&i), expected.as_ref());
        }
    }
}
