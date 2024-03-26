use std::collections::HashMap;
use std::path::Path;

mod chatgpt;
mod code_interpreter;

pub use chatgpt::ChatGPT;
pub use code_interpreter::CodeInterpreter;

pub trait Transpiler {
    /// transpile from source code directly
    async fn transpile<T: AsRef<str>>(&self, source: T) -> color_eyre::Result<String>;
    /// transpile source file or files under a folder
    async fn transpile_from_path<P: AsRef<Path>>(&self, path: P) -> color_eyre::Result<Vec<TranspileInfo>>;
}

pub struct TranspileInfo {
    /// full path and transpiled code
    map: HashMap<String, String>,
}