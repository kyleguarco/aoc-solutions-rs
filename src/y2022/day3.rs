use std::str::FromStr;

use crate::get_input;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Priority(u32);

impl TryFrom<char> for Priority {
	type Error = ();

	fn try_from(value: char) -> Result<Self, Self::Error> {
		fn to_u32(c: char) -> u32 {
			u32::from(c as u8)
		}

		match value {
			// SAFETY: We know these values are always above 38
			_ if value.is_ascii_lowercase() => Ok(Self(to_u32(value).checked_sub(96).unwrap())),
			_ if value.is_ascii_uppercase() => Ok(Self(to_u32(value).checked_sub(38).unwrap())),
			_ => Err(()),
		}
	}
}

#[derive(Debug)]
struct Compartment {
	contents: Vec<Priority>,
}

impl Compartment {
	fn shares(&self, other: &Self) -> Result<&Priority, ()> {
		// The list is sorted, so we know this search works.
		for p in &self.contents {
			if other.contents.binary_search(p).is_ok() {
				return Ok(p);
			}
		}
		Err(())
	}
}

impl FromStr for Compartment {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut contents: Vec<Priority> = s
			.chars()
			.map(|c| c.try_into())
			.collect::<Result<Vec<Priority>, ()>>()?;
		contents.sort();
		Ok(Self { contents })
	}
}

#[derive(Debug)]
struct Rucksack {
	first: Compartment,
	second: Compartment,
}

impl Rucksack {
	fn get_duplicate(&self) -> Result<&Priority, ()> {
		self.first.shares(&self.second)
	}
}

impl FromStr for Rucksack {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (first, second) = s.split_at(s.len() / 2);
		let first = first.parse::<Compartment>()?;
		let second = second.parse::<Compartment>()?;
		Ok(Self { first, second })
	}
}

#[test]
fn part1() {
	let prio = get_input("day_3.txt")
		.lines()
		.map(|s| {
			s.parse::<Rucksack>()
				.expect("Not a sack")
				.get_duplicate()
				.expect("No duplicates")
				.0
		})
		.sum::<u32>();
	println!("{prio}")
}

#[derive(Debug)]
struct Group {
	first: Rucksack,
	second: Rucksack,
	third: Rucksack,
}

impl Group {
	// We consume the group here because we need to mutate and break the inner
	// structure of each Rucksack (therefore making it useless).
	fn badge(self) -> Result<Priority, ()> {
		fn merge(mut sack: Rucksack) -> Vec<Priority> {
			sack.first.contents.extend(sack.second.contents);
			sack.first.contents.sort();
			sack.first.contents
		}

		let first = merge(self.first);
		let second = merge(self.second);
		let third = merge(self.third);

		for p in first {
			if second
				.binary_search(&p)
				.and(third.binary_search(&p))
				.is_ok()
			{
				return Ok(p);
			}
		}

		Err(())
	}
}

fn into_group<I: Iterator<Item = Rucksack>>(mut iter: I) -> impl Iterator<Item = Group> {
	std::iter::from_fn(move || {
		let first = iter.next()?;
		let second = iter.next()?;
		let third = iter.next()?;

		Some(Group {
			first,
			second,
			third,
		})
	})
}

#[test]
fn part2() {
	let input = get_input("day_3.txt");
	let sacks = input
		.lines()
		.map(|s| s.parse::<Rucksack>().expect("Not a sack"));
	let groups = into_group(sacks).map(|g| g.badge().unwrap().0).sum::<u32>();
	println!("{groups:?}")
}
