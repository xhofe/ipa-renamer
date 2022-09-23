use std::{
    fs::{self, File},
    path::{Path, PathBuf},
    str::FromStr,
};

use clap::Parser;
use colored::*;

/// A command line tool for renaming your ipa files quickly
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Renamer {
    /// file path pattern to rename
    glob: String,

    /// Name template for the new file
    #[clap(default_value_t = String::from("$raw@$CFBundleIdentifier"), value_parser)]
    template: String,

    /// The dir to save the renamed files
    #[clap(short, long, default_value = "renamed")]
    out: String,

    /// The temp dir for the extracted ipa file
    #[clap(short, long, default_value_t = String::from("./temp"), value_parser)]
    temp: String,
}

impl Renamer {
    pub fn run(&self) -> anyhow::Result<()> {
        // create the temp dir
        fs::create_dir_all(&self.temp)?;
        fs::create_dir_all(&self.out)?;
        let files = glob::glob(&self.glob)?.collect::<Vec<_>>();
        files.into_iter().for_each(|v| {
            if let Ok(filename) = v {
                if let Ok(file) = File::open(&filename) {
                    let file_ext = filename
                        .extension()
                        .unwrap_or_default()
                        .to_str()
                        .unwrap_or_default();
                    if file_ext != "ipa" {
                        println!(
                            "[{}] {} is not a ipa file",
                            "skipped".yellow(),
                            filename.display()
                        );
                        return;
                    }
                    if let Err(e) = self.rename(&filename, file) {
                        eprintln!("Error: {:?}", e);
                    }
                }
            }
        });
        println!("[{}]", "Done!".green());
        Ok(())
    }
    fn rename(&self, path: &Path, file: File) -> anyhow::Result<()> {
        let plist = self.get_info_plist(file)?;
        let info = plist::Value::from_file(&plist)?;
        let cfbundle_identifier = info
            .as_dictionary()
            .and_then(|dict| dict.get("CFBundleIdentifier"))
            .and_then(|x| x.as_string())
            .ok_or(anyhow::anyhow!("CFBundleIdentifier not found"))?;
        fs::remove_file(&plist)?;
        let mut new_name = self
            .template
            .replace("$CFBundleIdentifier", cfbundle_identifier);
        new_name = new_name.replace(
            "$raw",
            path.file_stem()
                .ok_or(anyhow::anyhow!("Can't get raw name"))?
                .to_str()
                .ok_or(anyhow::anyhow!("Can't get raw name"))?,
        );
        if !new_name.ends_with(".ipa") {
            new_name.push_str(".ipa");
        }
        let new_path = Path::new(&self.out).join(new_name);
        fs::copy(path, &new_path)?;
        println!(
            "[{}] {} to {}",
            "renamed".green(),
            path.display().to_string().blue(),
            new_path.display().to_string().purple()
        );
        Ok(())
    }
    fn get_info_plist(&self, file: File) -> anyhow::Result<PathBuf> {
        let mut archive = zip::ZipArchive::new(file)?;
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let path = match file.enclosed_name() {
                Some(path) => path.to_owned(),
                None => continue,
            };

            if !(*file.name()).ends_with('/') && match_plist(&path) {
                let outpath = PathBuf::from_str(&self.temp)?.join("Info.plist");
                let mut outfile = fs::File::create(&outpath)?;
                std::io::copy(&mut file, &mut outfile)?;
                return Ok(outpath);
            }
        }
        Err(anyhow::anyhow!("Not found"))
    }
}

fn match_plist(path: &Path) -> bool {
    let path_str = path.display().to_string();
    path_str.chars().into_iter().filter(|c| c == &'/').count() == 2
        && path_str.ends_with("/Info.plist")
}
