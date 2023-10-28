use std::collections::HashSet;
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

use rand::Rng;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    // anime_root is the root directory where all the anime pictures are stored
    anime_root: String,

    // extensions is the list of extensions that will be considered as anime pictures
    extensions: HashSet<String>,
}

fn dfs_visit(dir: &Path, entries: &mut Vec<PathBuf>) {
    if !dir.is_dir() {
        return;
    }

    for entry in dir.read_dir().unwrap() {
        let path = entry.unwrap().path();

        match path.is_dir() {
            true => dfs_visit(&path, entries),
            false => entries.push(path),
        }
    }
}

fn get_all_anime_pictures(anime_root: &Path, extensions: &HashSet<String>) -> Vec<PathBuf> {
    let mut entries = Vec::new();
    dfs_visit(anime_root, &mut entries);

    entries.retain(|e| {
        let ext = e.extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
            .to_lowercase();

        extensions.contains(&ext)
    });

    entries.iter()
        .map(|e| e.canonicalize().unwrap())
        .collect()
}

fn select_random_anime_picture(anime_pictures: &Vec<PathBuf>) -> Option<PathBuf> {
    if anime_pictures.is_empty() {
        return None;
    }

    let index = rand::thread_rng().
        gen_range(0..anime_pictures.len());

    Some(anime_pictures[index].clone())
}


trait Cmd {
    fn run(&self, anime_picture: &Path);
}

struct MacOsCommand {}

impl MacOsCommand {
    fn new() -> MacOsCommand {
        MacOsCommand {}
    }
}

impl Cmd for MacOsCommand {
    fn run(&self, anime_picture: &Path) {
        let command = format!(
            "on run image_path
               tell application \"iTerm2\"
                  tell current session of current window
                    set background image to \"{}\"
                  end tell
               end tell
            end run", anime_picture.to_str().unwrap()
        );

        Command::new("osascript")
            .arg("-e")
            .arg(&command)
            .output()
            .expect("failed to execute process");
    }
}

fn new_cmd() -> Option<Box<dyn Cmd>> {
    match env::consts::OS {
        "macos" => { Some(Box::new(MacOsCommand::new())) }
        _ => None
    }
}

// This is an script, that will be executed when the iterm2 terminal is opened
//
// Goal is to choose a random anime picture and set it as the background of the terminal
//
// Put this script to .zshrc/.bashrc/.fishrc to run it
fn main() {
    let cfg: Config = serde_yaml::from_str(include_str!("../config/config.yaml"))
        .unwrap_or_else(|e| panic!("Failed to parse config.yaml: {}", e));

    let cmd = new_cmd()
        .unwrap_or_else(|| panic!("{} is not supported", env::consts::OS));

    let anime_root = Path::new(&cfg.anime_root);
    let anime_pictures = get_all_anime_pictures(anime_root, &cfg.extensions);
    let rnd = select_random_anime_picture(&anime_pictures)
        .unwrap_or_else(|| panic!("No anime pictures found in {}", cfg.anime_root));

    cmd.run(&rnd);
}
