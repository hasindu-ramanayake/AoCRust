use std::fs::read_to_string;
// use std::collections::HashMap;

fn read_input_file(file_name: &str) -> Vec<String> {
    // let mut lines  = Vec::new();
    // for line in read_to_string(file_name).unwrap().lines() {
    //     lines.push(line.to_string());
    // }
    // lines
    read_to_string(file_name).unwrap().lines().map(String::from).collect()
}


// for second task cannot use this -> ex: sx oneight one ckk 9 ldctxxnffqnzmjqvj
// fn refactor_word_to_numbers(mut line: String ) -> (u32, u32) {
//     let words_to_number_mp = HashMap::from([
//         ("one", '1'),
//         ("two",'2'),
//         ("three",'3'),
//         ("four", '4'),
//         ("five", '5'),
//         ("six", '6'),
//         ("seven",'7'),
//         ("eight", '8'),
//         ("nine", '9')
//     ]);
//         for (key, value) in words_to_number_mp {
//              final_line = line.replace( key, &value.to_string);
//         }
//         index +=1;
//         Some(final_line)
//     });
// }

fn part1() -> u32 {
    let input = read_input_file("/home/rama/Desktop/rustTest/AoCRust/aoc/inputs/Input_day1_final.txt");
    
    let mut sum: u32 = 0;

    for line in input.into_iter() {
        let mut index = 0;
        let line_iter = std::iter::from_fn( move || {
            let reduce_line = &line[index..];
            let result = if reduce_line.starts_with("one") {
                Some('1')
            } else if reduce_line.starts_with("two") {
                Some('2')
            }  else if reduce_line.starts_with("three") {
                Some('3')
            }  else if reduce_line.starts_with("four") {
                Some('4')
            }  else if reduce_line.starts_with("five") {
                Some('5')
            }  else if reduce_line.starts_with("six") {
                Some('6')
            }  else if reduce_line.starts_with("seven") {
                Some('7')
            }  else if reduce_line.starts_with("eight") {
                Some('8')
            }  else if reduce_line.starts_with("nine") {
                Some('9')
            }  else {
                let result = reduce_line.chars().next();
                result
            };
            index += 1;
            result
        });

        let mut it = line_iter.filter_map( |c| c.to_digit(10));
        let first = it.next().expect("number");
        sum += match it.last() { 
            Some(num) => format!("{first}{num}"),
            None => format!("{first}{first}"),
        }.parse::<u32>().expect("number");
        
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