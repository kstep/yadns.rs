#[cfg(feature = "nightly")]
mod inner {
    pub fn main() {}
}

#[cfg(not(feature = "nightly"))]
mod inner {
    extern crate syntex;
    extern crate serde_codegen;

    use std::env;
    use std::path::Path;

    pub fn main() {
        let outdir = env::var_os("OUT_DIR").unwrap();
        let src = Path::new("src/dto.rs.in");
        let dst = Path::new(&outdir).join("dto.rs");

        let mut registry = syntex::Registry::new();
        serde_codegen::register(&mut registry);
        registry.expand("", &src, &dst).unwrap();
    }
}

fn main() {
    inner::main();
}
