use std::{fs::OpenOptions, io::{BufWriter, Write}};

fn main() {
    println!("cargo:rerun-if-changed={}", PATH);
    generate_kinds().unwrap();
}

const PATH: &str = "src/datafixers/kinds/generated.rs";

fn generate_kinds() -> std::io::Result<()> {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(PATH)?;
    let mut file = BufWriter::new(file);
    let cnt = 50;

    file.write_all(r#"#![allow(dead_code)]
#![allow(unused_macros)]
//! This is an auto-generated file.
//! Edits would be invalidated after a refresh.
    "#.as_bytes())?;

    for i in 1..=cnt {
        let tn = format!("Kind{}", i);
        let mn  = format!("kind{}", i);

        let args = (1..=i).into_iter()
            .map(|a| format!("T{}", a))
            .collect::<Vec<_>>().join(", ");

        let ignored = (1..=i).into_iter()
            .map(|a| format!("_T{}", a))
            .collect::<Vec<_>>().join(", ");
        
        file.write_all(format!(
            r#"
/// Represents a **type constructor** that takes {i} arguments to construct a type.
pub trait {tn} {{
    type This<{args}>;
}}

#[macro_export]
macro_rules! {mn} {{
    ($t:ident) => {{
        impl<{ignored}> $crate::datafixers::kinds::generated::{tn} for $t<{ignored}> {{
            type This<{args}> = $t<{args}>;
        }}
    }};
}}
            "#,
            i = i, tn = tn, mn = mn, args = args, ignored = ignored
        ).as_bytes())?;
    }
    file.flush()?;

    Ok(())
}