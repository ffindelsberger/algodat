#[allow(dead_code)]

fn get_string() -> String {
    String::from("Hi")
}

#[cfg(test)]
mod test {
    use super::get_string;

    #[test]
    fn string_works() {
        let result = get_string();
        assert_eq!("Hi", result);
    }
}
