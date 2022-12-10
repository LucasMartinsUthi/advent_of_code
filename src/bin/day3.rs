fn find_common_item(line: &str) -> char {
    // splite the line in half
    let (first, second) = line.split_at(line.len() / 2);

    first
        .chars()
        .filter(|c| second.contains(*c))
        .next().unwrap()
}

// a group is 3 lines
fn find_common_item_between_group(lines: [&str; 3]) -> char {
    lines[0]
        .chars()
        .filter(|c| lines[1].contains(*c) && lines[2].contains(*c))
        .next().unwrap()
}

// Lowercase item types a through z have priorities 1 through 26.
// Uppercase item types A through Z have priorities 27 through 52.

fn get_priority(item: char) -> usize {
    let mut priority = 0;

    if item.is_lowercase() {
        priority = item as usize - 96;
    } else if item.is_uppercase() {
        priority = item as usize - 64 + 26;
    }

    return priority
}

fn main() {
    // Part 1
    let sum = include_str!("../inputs/input3.txt")
        .lines()
        .map(|line| {
            let common_item = find_common_item(line);
            let priority = get_priority(common_item);

            return priority
        })
        .sum::<usize>();

    println!("Sum: {}", sum);

    // Part 2
    let mut group = vec![];

    let mut sum_badges = include_str!("../inputs/input3.txt")
        .lines()
        .enumerate()
        .filter_map(|(i, line)| {
            if i % 3 != 0 || i == 0{ 
                group.push(line);
                return None 
            }
            

            let arr_group = [group[0], group[1], group[2]];

            // println!("Group: {:?}", arr_group);
            let badge = find_common_item_between_group(arr_group);
            println!("Badge: {}", badge);

            group.clear();
            group.push(line);
            

            return Some(get_priority(badge))
        })
        .sum::<usize>();

    sum_badges += get_priority(find_common_item_between_group([group[0], group[1], group[2]]));

    println!("Sum of badges: {}", sum_badges);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        // vJrwpWtwJgWrhcsFMMfFFhFp = p
        // jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL = L
        // PmmdzqPrVvPwwTWBwg = P
        // wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn = v
        // ttgJtRGJQctTZtZT = t
        // CrZsJsPPZsGzwwsLwLmpwMDw = s
        
        let line = "vJrwpWtwJgWrhcsFMMfFFhFp";
        assert_eq!(find_common_item(line).to_string(), "p");

        let line = "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL";
        assert_eq!(find_common_item(line).to_string(), "L");

        let line = "PmmdzqPrVvPwwTWBwg";
        assert_eq!(find_common_item(line).to_string(), "P");

        let line = "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn";
        assert_eq!(find_common_item(line).to_string(), "v");

        let line = "ttgJtRGJQctTZtZT";
        assert_eq!(find_common_item(line).to_string(), "t");

        let line = "CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(find_common_item(line).to_string(), "s");
    }

    #[test]
    fn test_get_priority() {
        assert_eq!(get_priority('a'), 1);
        assert_eq!(get_priority('z'), 26);
        assert_eq!(get_priority('A'), 27);
        assert_eq!(get_priority('Z'), 52);

        assert_eq!(get_priority('p'), 16);
        assert_eq!(get_priority('L'), 38);
        assert_eq!(get_priority('P'), 42);
        assert_eq!(get_priority('v'), 22);
        assert_eq!(get_priority('t'), 20);
        assert_eq!(get_priority('s'), 19);
    }

    #[test]
    fn test_badge() {
        let lines = ["vJrwpWtwJgWrhcsFMMfFFhFp", "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", "PmmdzqPrVvPwwTWBwg"];
        assert_eq!(find_common_item_between_group(lines).to_string(), "r");

        let lines = ["wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn", "ttgJtRGJQctTZtZT", "CrZsJsPPZsGzwwsLwLmpwMDw"];
        assert_eq!(find_common_item_between_group(lines).to_string(), "Z");

        let mut sum = 0;
        sum += get_priority('r');
        sum += get_priority('Z');

        assert_eq!(sum, 70);
    }
}