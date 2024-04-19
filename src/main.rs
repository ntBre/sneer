use parser::parse;

mod parser;

#[derive(Debug)]
struct Atom {
    atomic_number: Option<usize>,
    connectivity: Option<usize>,
    index: Option<usize>,
}

impl From<Vec<&str>> for Atom {
    fn from(fields: Vec<&str>) -> Self {
        let mut atomic_number = None;
        let mut connectivity = None;
        let mut index = None;
        for f in fields {
            let mut chars = f.chars().peekable();
            match chars.next() {
                Some('#') => {
                    atomic_number =
                        Some(take_digits(&mut chars).parse().unwrap());
                }
                Some('X') => {
                    connectivity =
                        Some(take_digits(&mut chars).parse().unwrap());
                }
                Some(':') => {
                    index = Some(take_digits(&mut chars).parse().unwrap());
                }
                t => todo!("parse `{t:?}` into Atom"),
            }
        }
        Self {
            atomic_number,
            connectivity,
            index,
        }
    }
}

fn take_digits(chars: &mut impl Iterator<Item = char>) -> String {
    chars.by_ref().take_while(|c| c.is_numeric()).collect()
}

#[derive(Debug)]
struct Bond {
    order: usize,
    atom1: usize,
    atom2: usize,
}

#[derive(Debug, Default)]
struct Molecule {
    atoms: Vec<Atom>,
    bonds: Vec<Bond>,
}

// it's okay that atom doesn't handle nested structure currently because
// eventually ATOM will be defined to allow recursive atoms
fn main() {
    let input = "[#6X3:1]-[#16X4,#16X3+0:2]-[#7X4,#7X3:3]-[#6X4:4]";
    let (first, rest) = parse(input);
    dbg!(&first, &rest);
    let mut molecule = Molecule::default();
    molecule.atoms.push(Atom::from(first));

    dbg!(molecule);
}
