fn main() {
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=res/duck.ico");
    println!("cargo:rerun-if-changed=res/duck.rc");

    let _ = embed_resource::compile_for_cdylib(
        "res/duck.rc", embed_resource::NONE
    ).manifest_optional();
}