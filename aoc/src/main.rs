use std::error::Error;
use std::fs;
use std::io;
use std::path::Path;

use clap::Parser as _;

#[derive(Debug, clap::Parser, serde::Serialize)]
struct Args {
    /// AOC current year
    #[clap(long)]
    year: usize,
    /// AOC current day
    #[clap(long)]
    day: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let dir = format!("{}/day/{}", args.year, args.day);

    fs::create_dir_all(format!("{}/benches", dir))?;
    fs::create_dir_all(format!("{}/src", dir))?;

    let mut tt = tinytemplate::TinyTemplate::new();

    tt.add_template(
        "benches/main.rs",
        include_str!("../template/benches/main.rs.hbs"),
    )?;
    tt.add_template("src/lib.rs", include_str!("../template/src/lib.rs.hbs"))?;
    tt.add_template("src/main.rs", include_str!("../template/src/main.rs.hbs"))?;
    tt.add_template("Cargo.toml", include_str!("../template/Cargo.toml.hbs"))?;
    tt.add_template("test.txt", include_str!("../template/test.txt.hbs"))?;

    for filename in [
        "benches/main.rs",
        "src/lib.rs",
        "src/main.rs",
        "Cargo.toml",
        "test.txt",
    ] {
        let path = format!("{}/{}", dir, filename);
        let path = Path::new(&path);

        if path.exists() {
            return Err(io::Error::from(io::ErrorKind::AlreadyExists).into());
        }

        fs::write(path, tt.render(filename, &args)?)?;
    }

    Ok(())
}
