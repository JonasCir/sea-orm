use std::fmt::Display;
use std::{fs, io};
use std::io::Write;
use std::path::{Path, PathBuf};
use proc_macro2::TokenStream;
use syn::File;

#[cfg(feature = "codegen")]
pub mod generate;
pub mod migrate;

#[cfg(feature = "codegen")]
pub use generate::*;
pub use migrate::*;

pub fn handle_error<E>(error: E)
where
    E: Display,
{
    eprintln!("{}", error);
    ::std::process::exit(1);
}

pub(self) fn read_dir<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> {
    Ok(fs::read_dir(path)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.file_type().ok()?.is_file() {
                Some(entry.file_name().to_string_lossy().into_owned())
            } else {
                None
            }
        })
        .collect())
}

pub(self) fn emit_generated_code(
    file_path: &Path,
    file_content_tokens: &TokenStream,
)  {
    // formatting and pretty printing
    let syntax_tree = syn::parse_str::<File>(&file_content_tokens.to_string()).unwrap();
    let formatted = prettyplease::unparse(&syntax_tree);
    dbg!(&formatted);

    let mut file = fs::File::create(&file_path).unwrap();

    file.write_all(formatted.as_bytes()).unwrap();

}