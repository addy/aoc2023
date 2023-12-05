use std::fs;

extern crate radix_trie;

use radix_trie::Trie;

fn main() {
    let mut first = Trie::new();
    let mut last = Trie::new();

    first.insert("o", 0);
    first.insert("on", 0);
    first.insert("one", 1);

    first.insert("first", 0);
    first.insert("tw", 0);
    first.insert("two", 2);

    first.insert("t", 0);
    first.insert("th", 0);
    first.insert("thr", 0);
    first.insert("thre", 0);
    first.insert("three", 3);

    first.insert("f", 0);
    first.insert("fo", 0);
    first.insert("fou", 0);
    first.insert("four", 4);

    first.insert("f", 0);
    first.insert("fi", 0);
    first.insert("fiv", 0);
    first.insert("five", 5);

    first.insert("s", 0);
    first.insert("si", 0);
    first.insert("six", 6);

    first.insert("s", 0);
    first.insert("se", 0);
    first.insert("sev", 0);
    first.insert("seve", 0);
    first.insert("seven", 7);

    first.insert("e", 0);
    first.insert("ei", 0);
    first.insert("eig", 0);
    first.insert("eigh", 0);
    first.insert("eight", 8);

    first.insert("n", 0);
    first.insert("ni", 0);
    first.insert("nin", 0);
    first.insert("nine", 9);

    last.insert("e", 0);
    last.insert("en", 0);
    last.insert("eno", 1);

    last.insert("o", 0);
    last.insert("ow", 0);
    last.insert("owt", 2);

    last.insert("e", 0);
    last.insert("ee", 0);
    last.insert("eer", 0);
    last.insert("eerh", 0);
    last.insert("eerht", 3);

    last.insert("r", 0);
    last.insert("ru", 0);
    last.insert("ruo", 0);
    last.insert("ruof", 4);

    last.insert("e", 0);
    last.insert("ev", 0);
    last.insert("evi", 0);
    last.insert("evif", 5);

    last.insert("x", 0);
    last.insert("xi", 0);
    last.insert("xis", 6);

    last.insert("n", 0);
    last.insert("ne", 0);
    last.insert("nev", 0);
    last.insert("neve", 0);
    last.insert("neves", 7);

    last.insert("t", 0);
    last.insert("th", 0);
    last.insert("thg", 0);
    last.insert("thgi", 0);
    last.insert("thgie", 8);

    last.insert("e", 0);
    last.insert("en", 0);
    last.insert("eni", 0);
    last.insert("enin", 9);

    let file_path = "./input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let lines = contents.split("\n");
    let mut total = 0;

    let mut row = 1;

    for line in lines {
        if line.len() == 0 {
            break;
        }

        let mut str_idx: usize = 0;
        let mut idx_set = false;
        let mut curr_str = String::from("");
        let mut total_str = String::from("");

        let mut i = 0;
        let line_chars: Vec<_> = line.chars().collect();
        while i < line_chars.len() {
            let c = line_chars[i];

            if c.is_digit(10) {
                total_str.push(c);
                break;
            }

            curr_str.push(c);

            match first.get(curr_str.as_str()) {
                Some(val) => {
                    if val.to_owned() == 0 {
                        if !idx_set {
                            str_idx = i;
                            idx_set = true;
                        }
                    } else {
                        total_str.push_str(&val.to_string());
                        break;
                    }
                }
                None => {
                    if idx_set {
                        i = str_idx;
                        str_idx = 0;
                        idx_set = false;
                    }

                    curr_str.clear();
                }
            }
            
            i += 1;
        }

        curr_str.clear();
        i = 0;
        str_idx = 0;

        let rev_line_chars: Vec<_> = line.chars().rev().collect();
        while i < rev_line_chars.len() {
            let c = rev_line_chars[i];

            if c.is_digit(10) {
                total_str.push(c);
                break;
            }

            curr_str.push(c);

            match last.get(curr_str.as_str()) {
                Some(val) => {
                    if val.to_owned() == 0 {
                        if !idx_set {
                            idx_set = true;
                            str_idx = i;
                        }
                    } else {
                        total_str.push_str(&val.to_string());
                        break;
                    }
                }
                None => {
                    if idx_set {
                        i = str_idx;
                        str_idx = 0;
                        idx_set = false;
                    }

                    curr_str.clear();
                }
            }

            i += 1;
        }

        let mut num: i32 = 0;
        match total_str.parse::<i32>() {
            Ok(val) => {
                num = val;
                println!("{} - {}", line, val);
            }
            Err(err) => {
                println!("{} - {}", row, err);
            }
        }

        total += num;
        row += 1;
    }

    println!("Total: {total}");
}
