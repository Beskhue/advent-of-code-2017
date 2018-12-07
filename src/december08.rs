use std::collections::HashMap;
mod utils;

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

fn interpret(lines: &[String]) -> Result<(i32, i32)> {
	let mut max = 0;
	let mut registers: HashMap<String, i32> = HashMap::new();
	for line in lines {
		let instrs: Vec<&str> = line.split(" ").collect();
		
		let test_register = registers.entry(instrs[4].to_string()).or_insert(0).clone();
		let register = registers.entry(instrs[0].to_string()).or_insert(0);

		let pass = match (instrs[5], instrs[6].parse::<i32>()?) {
			("==", n) => test_register == n,
			("<", n) => test_register < n,
			("<=", n) => test_register <= n,
			(">", n) => test_register > n,
			(">=", n) => test_register >= n,
			("!=", n) => test_register != n,
			(o, _) => return Err(From::from(format!("Unkown operand: `{}`", o)))
		};
		
		if pass {
			match (instrs[1], instrs[2].parse::<i32>()?) {
				("inc", n) => *register += n,
				("dec", n) => *register -= n,
				_ => ()
			}
		}

		max = std::cmp::max(*register, max);
	}

	Ok((*registers.iter().map(|(_, v)| v).max().unwrap_or(&0), max))
}

fn main() -> Result<()> {
	let lines = utils::lines_from_file("input/december08.txt")?;

    println!("Part 1 & 2: {:#?}", interpret(&lines)?);

    Ok(())
}
