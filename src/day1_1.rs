use std::fs;

fn main() {
    let file_path = "./input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines = contents.split("\n");
    let mut total = 0;

    for line in lines {
        let mut first = '0';
        let mut last = '0';

        for c in line.chars() {
            if c.is_digit(10) {
                first = c;
                break;
            }
        }

        for c in line.chars().rev() {
            if c.is_digit(10) {
                last = c;
                break;
            }
        }

        let mut val = String::from("");
        val.push(first);
        val.push(last);

        let num: i32 = val.parse().unwrap();
        total += num;
    }

    println!("Total: {total}");
}
