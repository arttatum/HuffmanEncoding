pub use clap::Parser;
#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Mode {
    Compress,
    Decompress,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum TokenType {
    Chars,
    Words,
}

/// A compression and decompression tool.
///
/// The default behaviour is to compress stdin to stdout. Optionally, input and output file paths may be provided.
///
/// During compression, the text is broken into 'tokens', either chars or words. Depending on the workload, compression ratio and speed may be better for one choice or the other. The default token type is 'chars'.
///
/// To decompress a file, set --mode=decompress and ensure the same token type is selected as was
/// used in compression.
///
#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
pub struct Args {
    #[arg(short, long, default_value_t = Mode::Compress)]
    #[clap(value_enum)]
    pub mode: Mode,

    #[arg(short, long, default_value_t = TokenType::Chars)]
    #[clap(value_enum)]
    pub token_type: TokenType,

    /// File path of input, otherwise the compressor reads from stdin.
    #[arg(short, long)]
    pub in_file: Option<String>,

    /// File path of output, otherwise the compressor writes to stdout.
    #[arg(short, long)]
    pub out_file: Option<String>,
}
