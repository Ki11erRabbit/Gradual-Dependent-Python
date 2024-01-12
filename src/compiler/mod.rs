use rustpython_ast::Mod;
use rustpython_parser::ast::{self as ast, fold::Fold};

pub use rustpython_parser::{source_code::LinearLocator, Parse};

// these modules are out of repository. re-exporting them here for convenience.
pub use rustpython_parser as parser;
use rustpython_parser_core::Mode;
use rustpython_parser_core::source_code::SourceRange;

#[derive(Debug)]
pub enum CompileErrorType {
    Parse(parser::ParseErrorType),
}

impl std::error::Error for CompileErrorType {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CompileErrorType::Parse(e) => e.source(),
        }
    }
}
impl std::fmt::Display for CompileErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CompileErrorType::Parse(e) => e.fmt(f),
        }
    }
}
impl From<parser::ParseErrorType> for CompileErrorType {
    fn from(source: parser::ParseErrorType) -> Self {
        CompileErrorType::Parse(source)
    }
}

pub type CompileError = rustpython_parser::source_code::LocatedError<CompileErrorType>;

/// Compile a given source code into a normal Python script.
fn get_ast(
    source: &str,
    mode: Mode,
    source_path: String,
) -> Result<Mod<SourceRange>, CompileError> {
    let mut locator = LinearLocator::new(source);
    let ast = match parser::parse(source, mode.into(), &source_path) {
        Ok(x) => x,
        Err(e) => return Err(locator.locate_error(e)),
    };
    let ast = locator.fold_mod(ast).unwrap_or_else(|e| match e {});
    Ok(ast)
}

pub fn compile(
    source: &str,
    source_path: String,
) -> Result<Mod<SourceRange>, CompileError> {
    get_ast(source, Mode::Module, source_path)
}

