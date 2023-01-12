use qwiki::run;
//Make a wikipedia CLI app
fn main() {
    if let Err(e) = run(&std::env::args().collect()) {
        eprintln!("{e}\nType qwiki -h for help");
        std::process::exit(1);
    }
}
