use qwiki::run;
//Make a wikipedia CLI app
fn main() {
    if let Err(e) = run(&std::env::args().collect()) {
        eprintln!("{e}\n Type qwiki -h for help");
        std::process::exit(1);
    }
}
