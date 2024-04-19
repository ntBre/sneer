use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1};
use nom::combinator::recognize;
use nom::multi::{many0, many1};
use nom::sequence::{pair, tuple};
use nom::{sequence::delimited, IResult};

/// ATOM := "[" stuff+ "]"
fn atom_primitive(s: &str) -> IResult<&str, &str> {
    alt((
        tag("*"),
        tag("a"),
        tag("A"),
        recognize(pair(tag("D"), digit1)),
        recognize(pair(tag("H"), digit1)),
        recognize(pair(tag("h"), digit1)),
        recognize(pair(tag("R"), digit1)),
        recognize(pair(tag("r"), digit1)),
        recognize(pair(tag("v"), digit1)),
        recognize(pair(tag("X"), digit1)),
        recognize(pair(tag("x"), digit1)),
        recognize(pair(tag("-"), digit1)),
        recognize(pair(tag("+"), digit1)),
        recognize(pair(tag("#"), digit1)),
        recognize(pair(tag(":"), digit1)),
        tag("@"),
        tag("@@"), // NOTE skipping weird chiral options @<c><n> and @<c><n>?
        delimited(char('<'), digit1, char('>')),
    ))(s)
}

fn atom(s: &str) -> IResult<&str, Atom> {
    delimited(
        char('['),
        many1(alt((atom_primitive, logical_operator))),
        char(']'),
    )(s)
}

// TODO bonds can also include logical operators and additional bonds
/// BOND := "-" | "/" | "\" | "/?" | "\?" | "=" | "#" | ":" | "~" | "@"
fn bond_primitive(s: &str) -> IResult<&str, &str> {
    alt((
        tag("-"),
        tag("/"),
        tag("\\"),
        tag("/?"),
        tag("\\?"),
        tag("="),
        tag("#"),
        tag(":"),
        tag("~"),
        tag("@"),
    ))(s)
}

fn logical_operator(s: &str) -> IResult<&str, &str> {
    alt((tag("!"), tag("&"), tag(","), tag(";")))(s)
}

fn bond(s: &str) -> IResult<&str, Bond> {
    many0(alt((bond_primitive, logical_operator)))(s)
}

type Atom<'a> = Vec<&'a str>;
type Bond<'a> = Vec<&'a str>;

/// MOLECULE := ATOM [BOND ATOM]*
fn molecule(s: &str) -> IResult<&str, (Atom, Vec<(Bond, Atom)>)> {
    tuple((atom, many0(pair(bond, atom))))(s)
}

// it's okay that atom doesn't handle nested structure currently because
// eventually ATOM will be defined to allow recursive atoms
fn main() {
    let input = "[#6X3:1]-[#16X4,#16X3+0:2]-[#7X4,#7X3:3]-[#6X4:4]";
    dbg!(molecule(input).unwrap());
}
