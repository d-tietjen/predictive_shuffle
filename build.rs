// build.rs

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

const RADIX: u32 = 2;

fn main() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("byte_lookup.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    let mut builder: phf_codegen::Map<u8> = phf_codegen::Map::new();

    for key in 0..=255 {
        let binary = format!("{:08b}", key);
        builder.entry(key, format!("{:?}", binary).as_str());
    }

    write!(
        &mut file,
        "static BYTES: phf::Map<u8, &'static str> = {}",
        builder.build()
    )
    .unwrap();

    write!(&mut file, ";\n").unwrap();
}

// const RADIX: u32 = 10;

// let value: Vec<bool> = binary
//             .chars()
//             .map(|c| matches!(c.to_digit(RADIX).unwrap(), 1))
//             .collect();

// fn main() {
//     const RADIX: u32 = 10;

//     let mut map: HashMap<u8, Vec<bool>> = HashMap::new();
//     for i in 0..=255 {
//         let vec: Vec<bool> = format!("{:08b}", i)
//             .chars()
//             .map(|c| matches!(c.to_digit(RADIX).unwrap(), 1))
//             .collect();
//         map.insert(i, vec);
//     }

//     println!("{map:#?}")
// }
