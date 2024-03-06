use color_eyre::eyre::eyre;
use crate::transpiler::{ChatGPT, Transpiler};

mod transpiler;
mod llm;

#[tokio::main]
async fn main() {
    match run().await {
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
        _ => {}
    }
}

async fn run() -> color_eyre::Result<()> {
    let gpt = ChatGPT::init()?;
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        return Err(eyre!("usage: {} <source>", args[0]));
    }
    let source_path = &args[1];

    let source_code = std::fs::read_to_string(source_path)?;
    let resp = gpt.transpile(&source_code).await?;
    println!("transpiled Rust code: ====\n {} \n====\n", resp);
    Ok(())
}
