extern crate phonetic;
extern crate itertools;
use itertools::Itertools;
use phonetic::phonetics::metaphone::double_metaphone::*;

const TEST_VALUES: &'static [&'static str] = &[
    &"leçon",

    &"mettre",
    &"maître",

    &"vin",
    &"vingt",

    &"vers",
    &"ver",
    &"verre",
    &"vert",
    &"vair"
];

fn main() {
    TEST_VALUES
        .iter()
        .foreach(|value| {
            let metaphone = match double_metaphone(&value) {
                Some(v) => v,
                None => panic!("hey")
            };
            println!("tuple: {:?} : (primary: {}, alternate: {}) ", value, metaphone.primary, metaphone.alternate);
        })
}
