use std::fs::read_to_string;

fn read_input_file(file_name: &str) -> Vec<String> {
    // let mut lines  = Vec::new();
    // for line in read_to_string(file_name).unwrap().lines() {
    //     lines.push(line.to_string());
    // }
    // lines
    read_to_string(file_name).unwrap().lines().map(String::from).collect()
}



fn part1() -> u32 {
    let input = read_input_file("/home/rama/Desktop/rustTest/AoCRust/aoc/inputs/Input_day1_final.txt");
    
    
    let mut sum: u32 = 0;

    for line in input.iter() {
        let mut first_digit: u32 = 0;
        let mut last_digit: u32 = 0;
        for i in line.chars() {
            if i.is_digit(10) {
                first_digit = i.to_digit(10).unwrap();
                break;
            }
        }
        for i in line.chars().rev() {
            if i.is_digit(10) {
                last_digit = i.to_digit(10).unwrap();
                break;
            }
        }
        sum += first_digit*10 + last_digit;
        // println!("{}",  );

    }
    
    sum
}

fn main() {
    let res: u32 = part1();
    dbg!(res);
}

#[cfg(test)]
mod test {
    use crate::part1;
    #[test]
    fn test_function () {
        let res: u32 = part1();
        assert_eq!( res, 142);
    }

}