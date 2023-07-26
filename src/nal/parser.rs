//! Non-Axiomatic Logic parser
//! sentence ::= <statement><punctuation> <tense> {frequency confidence}
//! judgment ::= <statement>. {<truth-value>}
//! question ::= <statement>? {<truth-value>}
//! goal ::= <statement>! {<desire-value>}
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until1},
    character::complete::{char, one_of},
    combinator::{map_res, opt, recognize},
    multi::{many0, many1},
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    IResult,
};

use super::{Sentence, Tense};

fn statement(input: &str) -> IResult<&str, (&str, &str)> {
    let j = take_until1(">.");
    let q = take_until1(">?");
    let g = take_until1(">!");
    let till_stend = alt((j, q, g));
    let punct = alt((tag(">."), tag(">?"), tag(">!")));

    let start = char('<');
    let mut pre = preceded(start, pair(till_stend, punct));
    pre(input)
    // map_res(pre, Sentence::from)(input)
}

fn tense(input: &str) -> IResult<&str, Tense> {
    // TODO: maybe explicitly fail on invalid tense strings?
    let opt_tense = opt(alt((tag(":|:"), tag(":\\:"), tag(":/:"))));

    map_res(preceded(char(' '), opt_tense), Tense::from)(input)
}

fn decimal(input: &str) -> IResult<&str, &str> {
    recognize(many1(terminated(one_of("0123456789"), many0(char('_')))))(input)
}

fn float(input: &str) -> IResult<&str, &str> {
    alt((
        // Case one: .42
        recognize(tuple((
            char('.'),
            decimal,
            opt(tuple((one_of("eE"), opt(one_of("+-")), decimal))),
        ))), // Case two: 42e42 and 42.42e42
        recognize(tuple((
            decimal,
            opt(preceded(char('.'), decimal)),
            one_of("eE"),
            opt(one_of("+-")),
            decimal,
        ))), // Case three: 42. and 42.42
        recognize(tuple((decimal, char('.'), opt(decimal)))),
    ))(input)
}

fn tv(input: &str) -> IResult<&str, Option<(&str, &str)>> {
    let two_floats = separated_pair(float, char(' '), float);
    let opt_tv = opt(delimited(char('{'), two_floats, char('}')));

    preceded(char(' '), opt_tv)(input)
}

pub fn sentence(input: &str) -> IResult<&str, Sentence> {
    map_res(tuple((statement, tense, tv)), Sentence::from)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_statement() {
        let test_statement = "<corridor --> location>. :|:";
        assert_eq!(
            statement(test_statement).unwrap(),
            (" :|:", ("corridor --> location", ">."))
        );
    }
}
