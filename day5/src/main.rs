use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, digit1, multispace1, newline, space1},
    multi::{many1, separated_list1},
    sequence::{delimited, preceded},
    *,
};

#[derive(Debug)]
struct Cargo<'a> {
    stacks: Vec<Stack<'a>>,
    moves: Vec<Move>,
}

#[derive(Debug, Clone)]
struct Move {
    from: u32,
    to: u32,
    count: u32,
}

#[derive(Debug)]
struct Stack<'a> {
    crates: Vec<&'a str>,
}

impl Cargo<'_> {
    fn move_crate(&mut self) {
        for Move { count, from, to } in self.moves.iter() {
            let len = self.stacks[*from as usize].crates.len();
            let drained = self.stacks[*from as usize]
                .crates
                .drain((len - *count as usize)..)
                .rev()
                .collect::<Vec<&str>>();
            for c in drained.iter() {
                self.stacks[*to as usize].crates.push(c);
            }
        }
    }
}

fn parse_crate(input: &str) -> IResult<&str, Option<&str>> {
    let (input, c) = alt((
        tag("   "),
        delimited(complete::char('['), alpha1, complete::char(']')),
    ))(input)?;

    let result = match c {
        "   " => None,
        value => Some(value),
    };
    Ok((input, result))
}

fn line(input: &str) -> IResult<&str, Vec<Option<&str>>> {
    let (input, result) = separated_list1(tag(" "), parse_crate)(input)?;

    Ok((input, result))
}

fn parse_cargo(input: &str) -> IResult<&str, Cargo> {
    let (input, crates_horizontal) = separated_list1(newline, line)(input)?;
    let (input, _) = newline(input)?;
    let (input, _numbers) = many1(preceded(space1, digit1))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, moves) = separated_list1(newline, parse_move)(input)?;

    let mut stacks: Vec<Stack> = vec![];
    for _ in 0..crates_horizontal[0].len() {
        stacks.push(Stack { crates: vec![] });
    }

    for vec in crates_horizontal.iter().rev() {
        for (i, c) in vec.iter().enumerate() {
            if let Some(c) = c {
                stacks[i].crates.push(c);
            }
        }
    }

    let cargo = Cargo { stacks, moves };

    Ok((input, cargo))
}

fn parse_move(input: &str) -> IResult<&str, Move> {
    let (input, _) = tag("move ")(input)?;
    let (input, number) = complete::u32(input)?;
    let (input, _) = tag(" from ")(input)?;
    let (input, from) = complete::u32(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, to) = complete::u32(input)?;

    Ok((
        input,
        Move {
            count: number,
            from: from - 1,
            to: to - 1,
        },
    ))
}

fn main() {
    // Read input from file = "input.txt"
    let input = std::fs::read_to_string("input.txt").unwrap();
    let (_, mut cargo) = parse_cargo(input.as_str()).unwrap();

    cargo.move_crate();

    // get the top crate of each stack
    let mut top_crates = vec![];
    for stack in cargo.stacks.iter() {
        if let Some(c) = stack.crates.last() {
            top_crates.push(c);
        }
    }

    let top_crates = top_crates
        .iter()
        .map(|c| c.to_string())
        .collect::<Vec<String>>();

    println!("{:?}", top_crates.join(""));
}
