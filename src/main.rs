use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::char;
use nom::multi::many0;
use nom::sequence::{pair, tuple};
use nom::{sequence::delimited, IResult};

/// ATOM := "[" stuff+ "]"
fn atom(s: &str) -> IResult<&str, &str> {
    delimited(char('['), is_not("]"), char(']'))(s)
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

type Bond<'a> = Vec<&'a str>;

/// MOLECULE := ATOM [BOND ATOM]*
fn molecule(s: &str) -> IResult<&str, (&str, Vec<(Bond, &str)>)> {
    tuple((atom, many0(pair(bond, atom))))(s)
}

// it's okay that atom doesn't handle nested structure currently because
// eventually ATOM will be defined to allow recursive atoms
fn main() {
    let input = "[#6X3:1]-[#16X4,#16X3+0:2]-[#7X4,#7X3:3]-[#6X4:4]";
    dbg!(molecule(input).unwrap());
}
