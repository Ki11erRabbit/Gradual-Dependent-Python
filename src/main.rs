mod compiler;
mod type_checker;

use rustpython_ast::unparse;
use compiler::compile;


fn get_program1() -> &'static str {
    "print('hello world')"
}

fn get_program2() -> String {
    let file = std::fs::read_to_string("test.py").unwrap();
    file
}



fn main() {
    let ast = compile(&get_program2(), String::from("test.py")).unwrap();
    let ast = unparse(ast);
    println!("{}", ast);
}
