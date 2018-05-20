extern crate cc;

use std::fs::File;
use std::io::{Write, Result};

fn main() {
    cc::Build::new()
		.file("src/arch/x86_64/driver/apic/lapic.c")
		.file("src/arch/x86_64/driver/keyboard/keyboard.c")
		.flag("-mcmodel=large")
		.compile("cobj");
	gen_vector_asm().unwrap();
}

fn gen_vector_asm() -> Result<()> {
	let mut f = File::create("src/arch/x86_64/boot/vector.asm").unwrap();

	writeln!(f, "# generated by build.rs - do not edit")?;
	writeln!(f, "section .text")?;
	writeln!(f, "extern __alltraps")?;
	for i in 0..256 {
		writeln!(f, "vector{}:", i)?;
		if !(i == 8 || (i >= 10 && i <= 14) || i == 17) {
			writeln!(f, "\tpush 0")?;
		}
		writeln!(f, "\tpush {}", i)?;
		writeln!(f, "\tjmp __alltraps")?;
	}

	writeln!(f, "\nsection .rodata")?;
	writeln!(f, "global __vectors")?;
	writeln!(f, "__vectors:")?;
	for i in 0..256 {
		writeln!(f, "\tdq vector{}", i)?;
	}
	Ok(())
}