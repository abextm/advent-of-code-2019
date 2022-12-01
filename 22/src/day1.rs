#[aoc(day1, part1)]
fn day1_part1(input: &str) -> usize {
	input.trim().split("\n\n")
		.map(|x| x.split("\n")
			.map(|x| x.parse::<usize>().unwrap())
			.sum())
		.max().unwrap()
}

#[aoc(day1, part2)]
fn day1_part2(input: &str) -> usize {
	let mut elves = input.trim().split("\n\n")
		.map(|x| x.split("\n")
			.map(|x| x.parse::<usize>().unwrap())
			.sum())
	.collect::<Vec<_>>();
	elves.sort_by_key(|x| std::cmp::Reverse(*x));
	elves[0..3].iter().sum()
}