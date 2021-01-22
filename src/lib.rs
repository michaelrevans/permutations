use std::format;


pub fn combinations(string: String) -> Vec<String> {
    if string.len() == 1 {
        return vec![string];
    }

    let letters = string
        .split("")
        .filter(|&s| s != "")
        .collect::<Vec<_>>();

    println!("{:?}", letters);

    let mut result: Vec<String> = vec![];

    for index in 0..letters.len() {
        // if letters[index] == "" {
        //     continue;
        // }

        println!("{}", index);

        let mut other_letters = letters.clone();

        let letter = other_letters.remove(index);

        let other_letters: String = other_letters
            .into_iter()
            .filter(|&s| s != "")
            .collect::<Vec<_>>()
            .join("");

        println!("other letters {}", other_letters);

        for res in combinations(other_letters) {
            result.push(format!("{}{}", letter, res));
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_does_one_letter() {
        let result = combinations(String::from("a"));

        assert_eq!(result, ["a"])
    }

    #[test]
    fn it_does_a_different_letter() {
        let result = combinations(String::from("b"));

        assert_eq!(result, ["b"]);
    }

    #[test]
    fn it_does_two_letters() {
        let result = combinations(String::from("ab"));

        assert_eq!(result, ["ab", "ba"])
    }
}
