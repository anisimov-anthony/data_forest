use data_forest::avl_tree::AVLTree;

use proptest::prelude::*;
use std::collections::HashSet;

use rand::SeedableRng;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 111,
        ..ProptestConfig::default()
    })]
    #[test]
    fn prop_insert_contains(values in prop::collection::vec(any::<i32>(), 1..111)) {
        let mut avl = AVLTree::new();

        for &v in &values {
            avl.insert(v);
        }

        for &v in &values {
            assert!(avl.contains(&v));
        }
    }
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 111,
        ..ProptestConfig::default()
    })]
    #[test]
    fn prop_insert_maintains_avl_invariants(values in prop::collection::vec(any::<i32>(), 1..111)) {
        let mut avl = AVLTree::new();

        for (i, &value) in values.iter().enumerate() {
            avl.insert(value);

            assert!(
                avl.is_balanced(),
                "Tree unbalanced after inserting {} (operation #{})",
                value, i
            );

            assert!(
                avl.is_valid_bst(),
                "BST property violated after inserting {}",
                value
            );
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
        let mut avl = AVLTree::new();
        let unique_values: Vec<i32> = values.iter().cloned().collect::<HashSet<_>>().into_iter().collect();
        let mut remaining = unique_values.clone();

        for &v in &values {
            avl.insert(v);
        }

        for &v in &values {
            avl.remove(&v);
            remaining.retain(|&x| x != v);

            assert!(!avl.contains(&v));
            assert_eq!(avl.min(), remaining.iter().min());
            assert_eq!(avl.max(), remaining.iter().max());
        }
    }
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 111,
        ..ProptestConfig::default()
    })]
    #[test]
    fn prop_remove_maintains_avl_invariants(values in prop::collection::vec(any::<i32>(), 1..111)) {

        let mut avl = AVLTree::new();

        for &value in values.iter() {
            avl.insert(value);
        }

        let mut values = values;
        values.shuffle(&mut StdRng::seed_from_u64(111));

        for (i, &value) in values.iter().enumerate() {
            avl.remove(&value);

            assert!(
                avl.is_balanced(),
                "Tree unbalanced after removing {} (operation #{})",
                value, i
            );

            assert!(
                avl.is_valid_bst(),
                "BST property violated after removing {}",
                value
            );
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
        let mut avl = AVLTree::new();
        let mut current_min = None;

        for &v in &values {
            avl.insert(v);
            current_min = Some(v)
                .filter(|&x| current_min.is_none_or(|min| x < min))
                .or(current_min);
            assert_eq!(avl.min(), current_min.as_ref());
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
        let mut avl = AVLTree::new();
        let mut current_max = None;

        for &v in &values {
            avl.insert(v);
            current_max = Some(v)
                .filter(|&x| current_max.is_none_or(|max| x > max))
                .or(current_max);
            assert_eq!(avl.max(), current_max.as_ref());
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
        let mut avl = AVLTree::new();
        avl.insert(value);

        assert!(avl.min() == avl.max() && avl.min() == Some(&value));
    }
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 111,
        ..ProptestConfig::default()
    })]
    #[test]
    fn prop_number_of_elements(values in prop::collection::vec(any::<i32>(), 1..111)) {
        let mut avl = AVLTree::new();

        for &v in &values {
            avl.insert(v);
        }

        assert_eq!(avl.number_of_elements(), values.iter().collect::<std::collections::HashSet<_>>().len());
    }
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 111,
        ..ProptestConfig::default()
    })]
    #[test]
    fn prop_ceil(values in prop::collection::vec(any::<i32>(), 1..111)) {
        let mut avl = AVLTree::new();
        for &v in &values {
            avl.insert(v);
        }

        let unique_values: Vec<i32> = values.into_iter().collect::<HashSet<_>>().into_iter().collect();

        for &v in &unique_values {
            assert_eq!(avl.ceil(&v), Some(&v));
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
        assert_eq!(avl.ceil(&i), expected.as_ref());
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
        let mut avl = AVLTree::new();
        for &v in &values {
            avl.insert(v);
        }

        let unique_values: Vec<i32> = values.into_iter().collect::<HashSet<_>>().into_iter().collect();

        for &v in &unique_values {
            assert_eq!(avl.floor(&v), Some(&v));
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
        assert_eq!(avl.floor(&i), expected.as_ref());
        }
    }
}
