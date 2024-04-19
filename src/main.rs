mod parser;

// it's okay that atom doesn't handle nested structure currently because
// eventually ATOM will be defined to allow recursive atoms
fn main() {
    let input = "[#6X3:1]-[#16X4,#16X3+0:2]-[#7X4,#7X3:3]-[#6X4:4]";
    let (first, rest) = parser::parse(input).unwrap();
    dbg!(first, rest);
}
