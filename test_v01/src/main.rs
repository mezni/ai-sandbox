use std::collections::HashMap;

fn find_prefix_match<'a>(prefixes: &HashMap<&'a str, &'a str>, word: &str) -> Option<&'a str> {
    prefixes.keys().find_map(|&prefix| {
        if word.starts_with(prefix) {
            prefixes.get(prefix) // Returns Option<&&str>
        } else {
            None
        }
    }).copied() // Converts Option<&&str> to Option<&str>
}

fn main() {
    let prefixes: HashMap<&str, &str> = [
        ("ba", "Value for ba"),
        ("bb", "Value for bb"),
        ("ca", "Value for ca"),
        ("cb", "Value for cb"),
        ("daa", "Value for daa"),
        ("dab", "Value for dab"),
        ("dac", "Value for dac"),
    ].iter().cloned().collect();

    let strings = vec!["banana", "cabana", "dabble", "apple", "bat", "dazzle"];

    for s in &strings {
        match find_prefix_match(&prefixes, s) {
            Some(value) => println!("'{}' starts with a prefix, value: {}", s, value),
            None => println!("'{}' does not start with any of the given prefixes", s),
        }
    }
}
