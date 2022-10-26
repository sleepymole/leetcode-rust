#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let mut res = Vec::new();
        let mut min_sum = usize::MAX;
        for i in 0..list1.len() {
            if i > min_sum {
                break;
            }
            for j in 0..list2.len() {
                if i + j > min_sum {
                    break;
                }
                if list1[i] == list2[j] {
                    if i + j < min_sum {
                        min_sum = i + j;
                        res.clear();
                    }
                    res.push(list1[i].clone());
                    break;
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_restaurant() {
        assert_eq!(
            Solution::find_restaurant(
                vec![
                    "Shogun".to_owned(),
                    "Tapioca Express".to_owned(),
                    "Burger King".to_owned(),
                    "KFC".to_owned()
                ],
                vec![
                    "Piatti".to_owned(),
                    "The Grill at Torrey Pines".to_owned(),
                    "Hungry Hunter Steakhouse".to_owned(),
                    "Shogun".to_owned()
                ]
            ),
            vec!["Shogun"]
        );
        assert_eq!(
            Solution::find_restaurant(
                vec![
                    "Shogun".to_owned(),
                    "Tapioca Express".to_owned(),
                    "Burger King".to_owned(),
                    "KFC".to_owned()
                ],
                vec![
                    "KFC".to_owned(),
                    "Shogun".to_owned(),
                    "Burger King".to_owned()
                ]
            ),
            vec!["Shogun"]
        );
        assert_eq!(
            Solution::find_restaurant(
                vec![
                    "Shogun".to_owned(),
                    "Tapioca Express".to_owned(),
                    "Burger King".to_owned(),
                    "KFC".to_owned()
                ],
                vec![
                    "KFC".to_owned(),
                    "Burger King".to_owned(),
                    "Tapioca Express".to_owned(),
                    "Shogun".to_owned()
                ]
            ),
            vec![
                "Shogun".to_owned(),
                "Tapioca Express".to_owned(),
                "Burger King".to_owned(),
                "KFC".to_owned()
            ],
        );
        assert_eq!(
            Solution::find_restaurant(
                vec![
                    "Shogun".to_owned(),
                    "Tapioca Express".to_owned(),
                    "Burger King".to_owned(),
                    "KFC".to_owned()
                ],
                vec![
                    "KNN".to_owned(),
                    "KFC".to_owned(),
                    "Burger King".to_owned(),
                    "Tapioca Express".to_owned(),
                    "Shogun".to_owned()
                ]
            ),
            vec![
                "Shogun".to_owned(),
                "Tapioca Express".to_owned(),
                "Burger King".to_owned(),
                "KFC".to_owned()
            ],
        );
        assert_eq!(
            Solution::find_restaurant(vec!["KFC".to_owned()], vec!["KFC".to_owned()]),
            vec!["KFC".to_owned()]
        );
    }
}
