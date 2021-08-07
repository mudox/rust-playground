use std::error::Error;
use std::fs;

use time::{format_description, OffsetDateTime};

fn main() -> Result<(), Box<dyn Error>> {
    let meta = fs::metadata("/Users/mudox/.dotfiles/ap/actions/update-all-guru-pods.zsh")?;
    let ctime = meta.created()?;
    let time: OffsetDateTime = ctime.into();
    let format = format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]")?;
    println!("ctime: {}", time.format(&format).unwrap());
    Ok(())
}
