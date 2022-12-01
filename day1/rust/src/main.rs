const INPUT_FILE: &str = "../input.txt";

fn main() -> Result<(), std::io::Error> {
    part_1()?;
    part_2()?;
    Ok(())
}

fn part_1() -> Result<(), std::io::Error> {
    let input = std::fs::read_to_string(INPUT_FILE)?;
    let max_bag = input
        .split_terminator("\n\n")
        .map(|bag| {
            bag.split_terminator('\n')
                .map(|val| val.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max();
    println!("max bag: {max_bag:?}");
    Ok(())
}

fn part_2() -> Result<(), std::io::Error> {
    let input = std::fs::read_to_string(INPUT_FILE)?;
    let mut bags = input
        .split_terminator("\n\n")
        .map(|bag| {
            bag.split_terminator('\n')
                .map(|val| val.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<_>>();
    bags.sort_unstable();
    let sum_most_3 = bags[bags.len() - 3..].iter().sum::<u32>();
    println!("sum most 3: {sum_most_3}");
    Ok(())
}
