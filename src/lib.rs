mod ast;
mod error;
mod lexer;

pub use error::Error;

pub type Result<T> = std::result::Result<T, Error>;

///
pub fn from_str(content: impl AsRef<str>) -> Result<ast::Story> {
    fn inner(content: &str) -> Result<ast::Story> {
        todo!("Parser not implemented")
    }

    inner(content.as_ref())
}
