fn main() {
    let string1 = String::from("hello");
    let string2 = String::from(" world!");
    
    let concatenated_string = concatenate_strings(&string1, &string2);
    println!("{}", concatenated_string);
}

fn concatenate_strings(s1: &str, s2: &str) -> String {
    format!("{}{}", s1, s2)
}

#[cfg(test)]
mod tests {
    use crate::concatenate_strings;

    #[test]
    fn concatenate_strings_works() {
        let result = concatenate_strings("a", "b");
        assert_eq!(String::from("ab"), result);
    }
}
