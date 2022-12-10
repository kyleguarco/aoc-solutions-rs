use crate::get_input;

fn detect_signal(packet: &str) -> usize {
	let packet: &[u8] = packet.as_bytes();
	let mut index = 5usize;
	let packet = packet.windows(4).skip(1);

	for sli in packet {
		// I cite my sources: https://stackoverflow.com/a/46766782/7200739
		if !(1..sli.len()).any(|i| sli[i..].contains(&sli[i - 1])) {
			break
		}
		index += 1;
	}

	index
}

fn detect_message(packet: &str) -> usize {
	let packet: &[u8] = packet.as_bytes();
	let mut index = 15usize;
	let packet = packet.windows(14).skip(1);

	for sli in packet {
		if !(1..sli.len()).any(|i| sli[i..].contains(&sli[i - 1])) {
			break
		}
		index += 1;
	}

	index
}

#[test]
fn test_part1() {
	let input = get_input("day_6_test.txt");
	println!("{:?}", input.lines().map(detect_signal).collect::<Vec<usize>>());
}

#[test]
fn part1() {
	let input = get_input("day_6.txt");
	println!("{}", detect_signal(&input));
}

#[test]
fn part2() {
	let input = get_input("day_6.txt");
	println!("{}", detect_message(&input));
}
