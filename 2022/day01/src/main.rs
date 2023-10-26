use ::std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut calories: Vec<u32> = input
        .split("\n\n")
        .map(|rows| {
            rows.split("\n")
                .filter_map(|num| num.parse::<u32>().ok())
                .sum()
        })
        .collect();

    calories.sort_by(|a, b| b.cmp(a));
    let total: u32 = calories.iter().take(3).sum();
    println!("{:?}", total)
}
