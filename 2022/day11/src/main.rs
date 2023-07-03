use std::fs;

use nom::{
    bytes::complete::tag,
    character::complete as cc,
    character::complete::{line_ending, multispace0, space1},
    combinator::opt,
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

#[derive(Debug, Clone)]
pub struct Monkey {
    pub index: u64,
    pub items: Vec<u64>,
    pub operation: Operation,
    pub divisor: u64,
    pub throw_to: (usize, usize), // first usize for true, second for false
    pub items_inspected: u64,
}

#[derive(Clone, Debug)]
pub enum Operation {
    Add(Term),
    Multiply(Term),
}

#[derive(Clone, Debug)]
pub enum Term {
    Old,
    Value(u64),
}

fn u64_parser(input: &str) -> IResult<&str, u64> {
    let (input, number_str) = cc::digit1(input)?;
    match number_str.parse::<u64>() {
        Ok(number) => Ok((input, number)),
        Err(_) => Err(nom::Err::Failure(nom::error::make_error(
            input,
            nom::error::ErrorKind::Char,
        ))),
    }
}

fn operation_parser(input: &str) -> IResult<&str, Operation> {
    let (input, _) = preceded(multispace0, tag("Operation: "))(input)?;
    let (input, _) = tag("new = old ")(input)?;
    let (input, operation_str) = nom::branch::alt((tag("+"), tag("*")))(input)?;
    let (input, _) = space1(input)?;
    match operation_str {
        "+" => {
            let (input, value) = opt(u64_parser)(input)?;
            match value {
                Some(val) => Ok((input, Operation::Add(Term::Value(val)))),
                None => {
                    let (input, _) = tag("old")(input)?;
                    Ok((input, Operation::Add(Term::Old)))
                }
            }
        }
        "*" => {
            let (input, value) = opt(u64_parser)(input)?;
            match value {
                Some(val) => Ok((input, Operation::Multiply(Term::Value(val)))),
                None => {
                    let (input, _) = tag("old")(input)?;
                    Ok((input, Operation::Multiply(Term::Old)))
                }
            }
        }
        _ => Err(nom::Err::Failure(nom::error::make_error(
            input,
            nom::error::ErrorKind::Char,
        ))),
    }
}

fn items_parser(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, _) = preceded(multispace0, tag("Starting items: "))(input)?;
    separated_list1(tag(", "), u64_parser)(input)
}

fn test_parser(input: &str) -> IResult<&str, u64> {
    let (input, _) = preceded(multispace0, tag("Test: divisible by "))(input)?;
    u64_parser(input)
}

fn receiver_parser(input: &str) -> IResult<&str, (usize, usize)> {
    let (input, _) = preceded(multispace0, tag("If true: throw to monkey "))(input)?;
    let (input, true_receiver) = u64_parser(input)?;
    let (input, _) = line_ending(input)?;
    let (input, _) = preceded(multispace0, tag("If false: throw to monkey "))(input)?;
    let (input, false_receiver) = u64_parser(input)?;
    Ok((input, (true_receiver as usize, false_receiver as usize)))
}

fn monkey_parser(input: &str) -> IResult<&str, Monkey> {
    let (input, _) = tag("Monkey ")(input)?;
    let (input, index) = u64_parser(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = line_ending(input)?;
    let (input, items) = items_parser(input)?;
    let (input, _) = line_ending(input)?;
    let (input, operation) = operation_parser(input)?;
    let (input, _) = line_ending(input)?;
    let (input, test) = test_parser(input)?;
    let (input, _) = line_ending(input)?;
    let (input, receivers) = receiver_parser(input)?;

    Ok((
        input,
        Monkey {
            index,
            items,
            operation,
            divisor: test,
            throw_to: receivers,
            items_inspected: 0,
        },
    ))
}

fn monkeys_parser(input: &str) -> IResult<&str, Vec<Monkey>> {
    let (input, monkeys) = nom::multi::separated_list0(multispace0, monkey_parser)(input)?;
    Ok((input, monkeys))
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let (_, mut monkeys) =
        monkeys_parser(&contents).unwrap_or_else(|err| panic!("Error parsing file: {:?}", err));

    for round in 0..20 {
        println!();
        println!("------------ Round {} -------------", round + 1);

        for monkey_idx in 0..monkeys.len() {
            monkeys[monkey_idx].items = monkeys[monkey_idx].items.iter().rev().cloned().collect();

            while !monkeys[monkey_idx].items.is_empty() {
                if let Some(item) = monkeys[monkey_idx].items.pop() {
                    monkeys[monkey_idx].items_inspected += 1;

                    let old_worry_level = item;

                    let worry_level = match &monkeys[monkey_idx].operation {
                        Operation::Add(term) => match term {
                            Term::Old => old_worry_level + old_worry_level,
                            Term::Value(val) => old_worry_level + val,
                        },
                        Operation::Multiply(term) => match term {
                            Term::Old => old_worry_level * old_worry_level,
                            Term::Value(val) => old_worry_level * val,
                        },
                    };

                    // bored worry level is the worry level divided by 3 and rounded down
                    let bored_worry_level = worry_level / 3;

                    let receiver_idx = if bored_worry_level % monkeys[monkey_idx].divisor == 0 {
                        monkeys[monkey_idx].throw_to.0
                    } else {
                        monkeys[monkey_idx].throw_to.1
                    };

                    monkeys[receiver_idx].items.push(bored_worry_level);

                    println!(
                        "Monkey: {}, Item: {}, Worry: {}, Bored: {}, Receiver: {}",
                        monkey_idx, item, worry_level, bored_worry_level, receiver_idx
                    )
                }
            }
        }

        println!("------------ Monkeys state -------------");
        (0..monkeys.len()).for_each(|monkey_idx| {
            println!(
                "Monkey: {}, Items: {:?}, Items Inspected: {}",
                monkey_idx, monkeys[monkey_idx].items, monkeys[monkey_idx].items_inspected
            )
        });
    }

    println!();
    println!("------------ Final Monkeys state -------------");
    monkeys.iter().for_each(|monkey| {
        println!(
            "Monkey: {}, Items: {}, Items Inspected: {}",
            monkey.index,
            monkey.items.len(),
            monkey.items_inspected
        )
    });

    // sort monkeys with items_inspected and get the top two
    monkeys.sort_by(|a, b| b.items_inspected.cmp(&a.items_inspected));

    // get top two monkeys
    let top_monkeys = &monkeys[0..2];

    println!();
    println!("------------ Top Monkeys -------------");
    top_monkeys.iter().for_each(|monkey| {
        println!(
            "Monkey: {}, Items: {}, Items Inspected: {}",
            monkey.index,
            monkey.items.len(),
            monkey.items_inspected
        )
    });

    // Monkey business is the multiplication of the top two monkeys' items_inspected
    let monkey_business = top_monkeys
        .iter()
        .fold(1, |acc, monkey| acc * monkey.items_inspected);

    println!();
    println!("------------ Monkey Business -------------");
    println!("Monkey Business: {}", monkey_business);
}
