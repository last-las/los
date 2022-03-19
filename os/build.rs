use std::io::{Result, Write};
use std::fs::{self, File};

static TARGET_PATH: &str = "./user/target/riscv64gc-unknown-none-elf/release/";

fn main() {
    insert_app_data().unwrap();
}

fn insert_app_data() -> Result<()> {
    let mut f = File::create("src/link_app.asm").unwrap();
    let mut apps: Vec<_> = fs::read_to_string("application.txt").unwrap()
        .split_whitespace()
        .map(|a| {
            String::from(a)
        })
        .collect();
    apps.sort();

    writeln!(f, r#"
    .align 3
    .section .data
    .global _num_app
_num_app:
    .quad {}"#, apps.len())?;

    for i in 0..apps.len() {
        writeln!(f, r#" .quad app_{}_start"#, i)?;
    }
    writeln!(f, r#" .quad app_{}_end"#, apps.len() -1)?;

    for (idx, app) in apps.iter().enumerate() {
        println!("app_{}: {}", idx, app);
        writeln!(f, r#"
        .section .data
        .global app_{0}_start
        .global app_{0}_end
    app_{0}_start:
        .incbin "{2}{1}"
    app_{0}_end:"#, idx, app, TARGET_PATH)?;
    }

    Ok(())
}