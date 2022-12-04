use crate::get_input;

#[test]
fn part1() {
	let res: u32 = get_input("day_1.txt")
		.split("\n\n")
		.map(|cal| cal.lines().map(|s| s.parse::<u32>().unwrap()).sum())
		.max()
		.unwrap();
	println!("{res}");
}

#[test]
fn part2() {
	let mut res = get_input("day_1.txt")
		.split("\n\n")
		.map(|cal| cal.lines().map(|s| s.parse::<u32>().unwrap()).sum::<u32>())
		.collect::<Vec<_>>();
	res.sort_by(|a, b| b.cmp(a));

	let top = res.iter().take(3).sum::<u32>();
	println!("{top}");
}
