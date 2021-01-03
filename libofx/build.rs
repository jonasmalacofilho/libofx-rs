fn main() {
    println!("cargo:rerun-if-changed=src/ofxwrapper.cxx");
    cc::Build::new()
        .cpp(true)
        .file("src/ofxwrapper.cxx")
        .compile("ofxwrapper.a");
}
