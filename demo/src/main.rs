
fn main() {
    let a = "ihr alle".to_string();
    println!("{}", greeting(Some(&a)));
    println!("{}", greeting(Some(&a)));
    println!("{}", greeting(None));

    greeting(None);
}

#[must_use="greeting must be used"]
fn greeting(name: Option<&String>) -> String {
    format!("Hallo, {}", match name {
        None => "Fremder",
        Some(n) => n,
    })
}

#[cfg(test)]
mod test {
    use super::greeting;

    #[test]
    fn test_greeting() {
        assert_eq!("Hallo, Fremder".to_string(), greeting(None));
    }
}
