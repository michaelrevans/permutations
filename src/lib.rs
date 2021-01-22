use std::format;


pub fn permutations(string: String) -> Vec<String> {
    if string.len() == 1 {
        return vec![string];
    }

    let letters = string
        .split("")
        .filter(|&s| s != "")
        .collect::<Vec<_>>();

    let mut result: Vec<String> = vec![];

    for index in 0..letters.len() {
        let mut other_letters = letters.clone();

        let letter = other_letters.remove(index);

        let other_letters: String = other_letters
            .into_iter()
            .filter(|&s| s != "")
            .collect::<Vec<_>>()
            .join("");

        for res in permutations(other_letters) {
            result.push(format!("{}{}", letter, res));
        }
    }

    result.sort();
    result.dedup();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_does_one_letter() {
        let result = permutations(String::from("a"));

        assert_eq!(result, ["a"]);
    }

    #[test]
    fn it_does_a_different_letter() {
        let result = permutations(String::from("b"));

        assert_eq!(result, ["b"]);
    }

    #[test]
    fn it_does_two_letters() {
        let result = permutations(String::from("ab"));

        assert_eq!(result, ["ab", "ba"]);
    }

    #[test]
    fn it_does_many_letters() {
        let mut result = permutations(String::from("abc"));
        result.sort();

        let mut expected_result = ["abc", "acb", "bac", "bca", "cab", "cba"];
        expected_result.sort();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn it_removes_duplicates() {
        let mut result = permutations(String::from("aabb"));
        result.sort();

        let mut expected_result = ["aabb", "abba", "abab", "baab", "baba", "bbaa"];
        expected_result.sort();

        assert_eq!(result, expected_result);
    }
}
