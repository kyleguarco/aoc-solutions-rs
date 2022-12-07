use std::str::FromStr;

use crate::get_input;

type NumSize = usize;

struct Instruction {
	quantity: NumSize,
	from: NumSize,
	to: NumSize,
}

impl FromStr for Instruction {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut s = s.split_whitespace();
		let mut next = || s.next().ok_or(());
		let mut word = |expected| {
			if !next()?.eq(expected) {
				return Err(());
			}

			next()?.parse::<NumSize>().map_err(|_| ())
		};

		let quantity = word("move")?;
		// Convert these to indicies
		let from = word("from")? - 1;
		let to = word("to")? - 1;

		Ok(Self { quantity, from, to })
	}
}

// It's basically just an ID.
type Crate = char;

fn crate_line_iter<I: Iterator<Item = char>>(
	mut iter: I,
) -> impl Iterator<Item = Result<Option<Crate>, ()>> {
	std::iter::from_fn(move || {
		let (opbr, name, clbr) = (iter.next()?, iter.next()?, iter.next()?);

		let ctype = match (opbr, clbr) {
			// Implies stack `idx` has a crate
			('[', ']') => Ok(Some(name)),
			// Implies stack `idx` has no crate on it at this level.
			(' ', ' ') => Ok(None),
			_ => return Some(Err(())),
		};

		// There must be a space after the crate definition, but that error
		// will be caught on the next iteration.
		iter.next();

		Some(ctype)
	})
}

// A collection of "crates"
type Stack = Vec<Crate>;

#[derive(Debug)]
struct Yard(Vec<Stack>);

impl Yard {
	// For part 1
	fn apply_noorder_inst<I: Iterator<Item = Instruction>>(&mut self, iter: I) {
		for inst in iter {
			for _ in 0..inst.quantity {
				// SAFETY: Thanks to the FromIterator impl on Yard, we know these indicies exist.
				let from = self.0.get_mut(inst.from).unwrap();
				let from = from.pop().unwrap();

				let to = self.0.get_mut(inst.to).unwrap();
				to.push(from);
			}
		}
	}

	// For part 2
	fn apply_inorder_inst<I: Iterator<Item = Instruction>>(&mut self, iter: I) {
		for inst in iter {
			// SAFETY: Thanks to the FromIterator impl on Yard, we know these indicies exist.
			let top_to = self.0.get(inst.to).unwrap().len();
			for _ in 0..inst.quantity {
				let from = self.0.get_mut(inst.from).unwrap();
				let from = from.pop().unwrap();

				let to = self.0.get_mut(inst.to).unwrap();
				// This preserves the order of elements on `from`
				to.insert(top_to, from);
			}
		}
	}

	fn get_tops(&self) -> String {
		let mut tops = String::new();
		for stack in &self.0 {
			tops.push(stack.get(stack.len() - 1).unwrap().clone());
		}
		tops
	}
}

impl FromIterator<(usize, Option<Crate>)> for Yard {
	fn from_iter<T: IntoIterator<Item = (usize, Option<Crate>)>>(iter: T) -> Self {
		let mut yard = vec![];

		for (index, name) in iter {
			if yard.len() < index + 1 {
				yard.resize(index + 1, Stack::new());
			}

			let Some(name) = name else {
				continue;
			};

			// SAFETY: The above line guarantees that a stack exists at this index
			yard.get_mut(index).unwrap().insert(0, name);
		}

		Self(yard)
	}
}

fn parse_input<'a>(input: &'a str) -> (Yard, impl Iterator<Item = Instruction> + 'a) {
	let mut input = input.split("\n\n");

	let mut crates = input.next().unwrap().lines();
	// Skip the last line for crate input
	crates.next_back();
	let crates = crates
		.flat_map(|s| crate_line_iter(s.chars()).enumerate())
		.map(|c| {
			// Unwrap the result to make sure `collect` stops if `crate_line_iter` fails.
			// The index must be preserved.
			match c {
				(index, Ok(Some(name))) => Ok((index, Some(name))),
				(index, Ok(None)) => Ok((index, None)),
				_ => Err(()),
			}
		})
		.flatten()
		.collect::<Yard>();

	let instructions = input
		.next()
		// SAFETY: Instructions always follow input
		.unwrap()
		.lines()
		.map(|s| s.parse::<Instruction>().unwrap());

	(crates, instructions)
}

#[test]
fn part1() {
	// Split the crate input and instructions
	let input = get_input("day_5.txt");
	let (mut crates, instructions) = parse_input(&input);

	println!("{:?} TOPS {}", crates, crates.get_tops());
	crates.apply_noorder_inst(instructions);
	println!("{:?} TOPS {}", crates, crates.get_tops());
}

#[test]
fn part2() {
	let input = get_input("day_5.txt");
	let (mut crates, instructions) = parse_input(&input);

	println!("{:?} TOPS {}", crates, crates.get_tops());
	crates.apply_inorder_inst(instructions);
	println!("{:?} TOPS {}", crates, crates.get_tops());
}
