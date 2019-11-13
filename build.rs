fn main() {
    cc::Build::new()
        .compiler("arm-none-eabi-gcc")
        .archiver("arm-none-eabi-ar")
        .no_default_flags(true)
        .warnings_into_errors(true)
        .flag("-mcpu=arm7tdmi")
        .out_dir("target")
        .file("src/arch/entry.S")
        .compile("entry.o");

    println!("cargo:rerun-if-changed=entry.S");
}
