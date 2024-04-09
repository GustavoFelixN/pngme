use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt)]
pub enum Options {
    Encode {
        #[structopt(parse(from_os_str))]
        /// Path to file
        file_path: PathBuf,

        #[structopt()]
        /// Chunk type
        chunk_type: String,

        #[structopt()]
        /// Message to encode
        message: String,

        #[structopt(parse(from_os_str))]
        /// Path to output file (optional)
        output: Option<PathBuf>,
    },
    Decode {
        #[structopt(parse(from_os_str))]
        /// Path to file
        file_path: PathBuf,

        #[structopt()]
        /// Chunk type
        chunk_type: String,
    },
    Remove {
        #[structopt(parse(from_os_str))]
        /// Path to file
        file_path: PathBuf,

        #[structopt()]
        /// Chunk type
        chunk_type: String,
    },
    Print {
        #[structopt(parse(from_os_str))]
        /// Path to file
        file_path: PathBuf,
    },
}
