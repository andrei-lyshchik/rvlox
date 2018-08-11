use scanner::*;

pub fn compile(source: &str) {
    let mut scanner = Scanner::new(source);

    for t in scanner {
        println!("{:?}", t);
    }
}