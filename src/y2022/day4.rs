use std::{cmp::Ordering, ops::RangeInclusive, str::FromStr};

use crate::get_input;

type SectionSize = u32;

/// The sections are listed like "10-70" (10..=70)
struct Section {
	range: RangeInclusive<SectionSize>,
}

impl FromStr for Section {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut look = s.split("-").map(|s| s.parse::<SectionSize>());

		let range = look.next().transpose().map_err(|_| ())?.ok_or(())?
			..=look.next().transpose().map_err(|_| ())?.ok_or(())?;

		if look.next().is_some() {
			return Err(());
		}

		Ok(Self { range })
	}
}

struct Pair {
	first: Section,
	second: Section,
}

impl Pair {
	// For part 1
	fn has_full_range(&self) -> bool {
		let mut fr = self.first.range.clone().into_iter();
		let mut sr = self.second.range.clone().into_iter();

		match self.first.range.start().cmp(self.second.range.start()) {
			Ordering::Less => sr.all(|n| fr.contains(&n)),
			Ordering::Greater => fr.all(|n| sr.contains(&n)),
			Ordering::Equal => match self.first.range.end().cmp(self.second.range.end()) {
				Ordering::Less => fr.all(|n| sr.contains(&n)),
				Ordering::Greater => sr.all(|n| fr.contains(&n)),
				Ordering::Equal => true,
			},
		}
	}

	// For part 2
	fn has_overlap(&self) -> bool {
		let mut fr = self.first.range.clone().into_iter();
		let mut sr = self.second.range.clone().into_iter();

		match self.first.range.start().ge(self.second.range.start()) {
			true => fr.any(|n| sr.contains(&n)),
			false => sr.any(|n| fr.contains(&n)),
		}
	}
}

impl FromStr for Pair {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut pairs = s.split(",").map(|s| s.parse::<Section>());

		let first = pairs.next().transpose().map_err(|_| ())?.ok_or(())?;
		let second = pairs.next().transpose().map_err(|_| ())?.ok_or(())?;

		if pairs.next().is_some() {
			return Err(());
		}

		Ok(Self { first, second })
	}
}

#[test]
fn part1() {
	let total = get_input("day_4.txt")
		.lines()
		.map(|s| s.parse::<Pair>().unwrap().has_full_range() as SectionSize)
		.sum::<SectionSize>();
	println!("{total}");
}

#[test]
fn part2() {
	let total = get_input("day_4.txt")
		.lines()
		.map(|s| s.parse::<Pair>().unwrap().has_overlap() as SectionSize)
		.sum::<SectionSize>();
	println!("{total}");
}
