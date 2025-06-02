// No requirements given, but assumed general purpose, small dataset with immutable input
//
// I only realised afterwards, I could have copied the 'before' placeholder code from
// https://github.com/LinkedInLearning/level-up-rust-3041143/blob/01_01b/src/main.rs
//
// Therefore my approach varies slightly 🤦

#[must_use]
pub fn median(list: &[f32]) -> Option<f32> {
    match list.len() {
        0 => None,
        1 => list.first().copied(),
        _ => {
            let mut sorted_list = list.to_vec();
            sorted_list.sort_unstable_by(f32::total_cmp);
            let mid_point = sorted_list.len() / 2;

            if sorted_list.len() % 2 == 0 {
                Some(sorted_list[mid_point].midpoint(sorted_list[mid_point - 1]))
            } else {
                sorted_list.get(mid_point).copied()
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::median;

    #[test]
    fn empty() {
        let sut = median(&[]);
        assert_eq!(sut, None);
    }

    #[test]
    fn single_entry() {
        let sut = median(&[5.0]);
        assert_eq!(sut, Some(5.0));
    }

    #[test]
    fn odd_length() {
        let sut = median(&[1.0, 3.0, 2.0]);
        assert_eq!(sut, Some(2.0));
    }

    #[test]
    fn even_length() {
        let sut = median(&[2.0, 3.0, 1.0, 4.0]);
        assert_eq!(sut, Some(2.5));
    }
}
