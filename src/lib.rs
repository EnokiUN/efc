mod centre;
mod outdent;

use std::{fs, io::stdin, path::PathBuf};

use anyhow::{anyhow, Context, Result};
use atty::Stream;

macro_rules! formatters {
    ($last:ident, $($name:ident),+) => {
        #[derive(Debug)]
        pub enum Formatter {
            $($name,)+
            $last,
        }
        pub const FORMATTER_NAMES: [&str; (Formatter::$last as u32 + 1) as usize] = [$(stringify!($name),)+ stringify!($last)];
    };
}

formatters!(Centre, Outdent);

#[derive(Debug)]
pub struct Params {
    pub formatter: Formatter,
    pub files: Vec<PathBuf>,
    pub write: bool,
}

pub fn format(params: Params) -> Result<()> {
    let formatter = match params.formatter {
        Formatter::Centre => centre::centre,
        Formatter::Outdent => outdent::outdent,
    };

    if params.files.is_empty() {
        if !atty::is(Stream::Stdin) {
            if params.write {
                Err(anyhow!("Cant write if input is received from Stdin"))?;
            }
            println!(
                "{}",
                formatter(
                    stdin()
                        .lines()
                        .filter(|l| l.is_ok())
                        .map(|l| l.unwrap())
                        .collect::<Vec<String>>()
                        .join("\n")
                )
            );
            return Ok(());
        }
        Err(anyhow!("You must specify files to format"))?
    }

    for file in params.files.into_iter() {
        let code = fs::read_to_string(&file)
            .with_context(|| format!("Could not read file {}", file.display()))?;
        let code = formatter(code);
        match params.write {
            false => println!("{}", code),
            true => fs::write(&file, code)
                .with_context(|| format!("Could not write to file {}", file.display()))?,
        }
    }

    Ok(())
}
