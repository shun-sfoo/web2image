use clap::{AppSettings, Clap};
use std::ffi::OsStr;
use std::path::Path;
use url::Url;

mod web2image;
use web2image::web2image;

#[derive(Clap, Debug)]
#[clap(version = "0.1", author = "shun-sfoo <shun-sfoo@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// output file
    #[clap(short, long, default_value = "/tmp/snapshot.png", validator = valid_filename)]
    output: String,

    /// url to capture
    #[clap(validator = valid_url)]
    url: String,
}

fn get_file_extension(path: &Path) -> Option<String> {
    path.extension()
        .and_then(|p| OsStr::to_str(p))
        .and_then(|ext| {
            let ext = ext.to_lowercase();
            match ext.as_str() {
                "jpg" | "jpeg" | "png" => Some(ext),
                _ => None,
            }
        })
}

fn valid_url(url: &str) -> Result<(), String> {
    match Url::parse(url) {
        Ok(_) => Ok(()),
        Err(_) => Err("you must provide a valid url".into()),
    }
}

/// "tmp/abc.pdf" => "tmp" exists
///  pdf in (png | jpg | jpeg)
fn valid_filename(name: &str) -> Result<(), String> {
    let path = Path::new(name);
    let parent = path.parent().and_then(|p| p.is_dir().then(|| p));
    let ext = get_file_extension(path);

    if parent.is_none() || ext.is_none() {
        return Err("File path must be exists and file must be jpg, jpeg or png".into());
    }

    Ok(())
}

fn main() {
    let opts: Opts = Opts::parse();
    println!("{:?}", opts);
    web2image(&opts.url, &opts.output).unwrap();
}
