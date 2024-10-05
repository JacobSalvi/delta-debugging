use regex::Regex;
use std::fs;

pub fn split_cf(n: usize, cf: &str) -> (Vec<String>, Vec<String>) {
    let mut deltas = Vec::new();
    let mut nambas = Vec::new();
    let split_size = cf.len() / n;
    let chunks: Vec<&str> = cf
        .as_bytes()
        .chunks(split_size)
        .map(|chunk| std::str::from_utf8(chunk).unwrap())
        .collect();
    for (i, &el) in chunks.iter().enumerate() {
        deltas.push(el.to_string());
        let namba = chunks
            .iter()
            .enumerate()
            .filter_map(|(j, &other)| if i != j { Some(other) } else { None })
            .collect::<Vec<&str>>()
            .join("");
        nambas.push(namba);
    }
    (deltas, nambas)
}

pub fn string_fails(test: &str) -> bool {
    let pattern = ".*<SELECT.*>";
    let reg = Regex::new(pattern).unwrap();
    if reg.is_match(test) {
        return true;
    }
    false
}

pub fn find_min_recursive(n: usize, cf: String, test_fn: &dyn Fn(&str) -> bool) -> String {
    let (deltas, nambas) = split_cf(n, &cf);
    for delta in deltas {
        if test_fn(&delta) {
            return find_min_recursive(2, delta, test_fn);
        }
    }
    for namba in nambas {
        if test_fn(&namba) {
            return find_min_recursive(n - 1, namba, test_fn);
        }
    }
    if n * 2 <= cf.len() {
        return find_min_recursive(n * 2, cf, test_fn);
    }
    cf
}

pub fn find_min(cf: String, test_fn: &dyn Fn(&str) -> bool) -> String {
    find_min_recursive(2, cf, test_fn)
}

fn main() {
    let matches = clap::Command::new("Performance test")
        .version("1.0")
        .about("")
        .arg(clap::arg!(-f --file <INPUT_FILE> "Input file to test.").required(true))
        .get_matches();

    let input_file = matches.get_one::<String>("file").unwrap();

    let content = fs::read_to_string(input_file).expect("Input file does not exist");
    let minimum_input = find_min(content, &string_fails);
    println!("{minimum_input}");
}
