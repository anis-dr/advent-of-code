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
            let (input, value) = u64_parser(input)?;
            Ok((input, Operation::Add(Term::Value(value))))
        }
        "*" => {
            let (input, value) = opt(u64_parser)(input)?;
            match value {
                Some(val) => Ok((input, Operation::Multiply(Term::Value(val)))),
                None => Ok((input, Operation::Multiply(Term::Old))),
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
    let (input, _) = cc::digit1(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = line_ending(input)?;
    let (input, items) = items_parser(input)?;
    let (input, _) = line_ending(input)?;
    let (input, operation) = operation_parser(input)?;
    let (input, _) = line_ending(input)?;
    let (input, test) = test_parser(input)?;
    let (input, _) = line_ending(input)?;
    let (input, receivers) = receiver_parser(input)?;
    let (input, _) = opt(line_ending)(input)?;
    Ok((
        input,
        Monkey {
            items,
            operation,
            divisor: test,
            throw_to: receivers,
            items_inspected: 0,
        },
    ))
}

fn monkeys_parser(input: &str) -> IResult<&str, Vec<Monkey>> {
    let (input, _) = multispace0(input)?;
    let (input, monkeys) = nom::multi::separated_list1(multispace0, monkey_parser)(input)?;
    Ok((input, monkeys))
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let (_, monkeys) =
        monkeys_parser(&contents).unwrap_or_else(|err| panic!("Error parsing file: {:?}", err));
}
