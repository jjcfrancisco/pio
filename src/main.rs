pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

mod pg;
mod process;
mod utils;

use utils::cli::run;

fn main() -> Result<()> {
    run()?;
    Ok(())
}
