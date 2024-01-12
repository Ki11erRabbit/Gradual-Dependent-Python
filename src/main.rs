mod compiler;

use compiler::compile;


fn get_program1() -> &'static str {
    "print('hello world')"
}



fn main() {
    let ast = compile(get_program1(), String::from("test.py")).unwrap();
    println!("{:?}",ast);
}
