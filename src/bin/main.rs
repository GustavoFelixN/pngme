pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

use pngme::args::Options;
use structopt::StructOpt;

use pngme::commands::{decode_message, encode_message, print_file, remove_chunk, save_to_file};

fn main() -> Result<()> {
    let opts = Options::from_args();
    match opts {
        Options::Encode {
            file_path,
            chunk_type,
            message,
            output,
        } => {
            let encoded_png = encode_message(&file_path, chunk_type, message)?;
            if let Some(output_path) = output {
                save_to_file(output_path, encoded_png)?;
            } else {
                save_to_file(file_path, encoded_png)?;
            }
        }
        Options::Decode {
            file_path,
            chunk_type,
        } => {
            let message = decode_message(&file_path, chunk_type)?;
            println!("{}", message);
        }
        Options::Remove {
            file_path,
            chunk_type,
        } => {
            let message = remove_chunk(file_path, chunk_type)?;
            println!("Message removed: {}", message);
        }
        Options::Print { file_path } => {
            print_file(file_path)?;
        }
    };
    Ok(())
}
