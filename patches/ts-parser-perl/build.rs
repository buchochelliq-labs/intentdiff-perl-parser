fn main() {
    let manifest = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let target = std::env::var("TARGET").unwrap_or_default();
    if target == "wasm32-wasip2" {
        println!("cargo:rustc-link-lib=static=ts_parser_perl");
        println!("cargo:rustc-link-search=native={}/lib", manifest);
        println!("cargo:rerun-if-changed=lib/libts_parser_perl.a");
        return;
    }

    let mut cfg = cc::Build::new();
    cfg.std("c11").include("src");
    if std::env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default() == "msvc" {
        cfg.flag("-utf-8");
    }
    cfg
        .file("src/parser.c")
        .file("src/scanner.c")
        .compile("ts_parser_perl");
}
