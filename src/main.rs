extern crate sysinfo;
extern crate tokio;
extern crate reqwest;
extern crate zip;


mod component;
mod downloader;

use std::{path::PathBuf, io::{BufRead, Write}};

use sysinfo::{ProcessExt, SystemExt};

use console;

use crate::downloader::Downloader;

#[tokio::main]
async fn main() {
    let term = console::Term::stdout();

    if execute(&term).await {
        term.write_line("").unwrap();
        term.write_line("").unwrap();
        term.write_line(&console::Style::new().green().apply_to(" Success").to_string()).unwrap();
    } else {
        term.write_line("").unwrap();
        term.write_line("").unwrap();
        term.write_line(&console::Style::new().red().apply_to(" Faild").to_string()).unwrap();
    }

    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
}

const DOWNLOAD_URL: &str = "https://drive.google.com/uc?id=1t4_N2z03XWyeAn6IdmR3xfBE2qHuCzx-&confirm=t";

async fn execute(term: &console::Term) -> bool {
    term.write_line("This program is distributed under the MIT License.").unwrap();
    term.write_line("  Copyright (c) 2022 VSMPNR").unwrap();
    term.write_line("  https://opensource.org/licenses/mit-license.php").unwrap();
    term.write_line("").unwrap();

    term.write_line("This program used these Crates.").unwrap();
    term.write_line("  tokio [version=\"1.19.0\" license=\"MIT\"]").unwrap();
    term.write_line("  reqwest [version=\"1.11.10\" license=\"MIT / Apache-2.0\"]").unwrap();
    term.write_line("  console [version=\"0.15.0\" license=\"MIT\"]").unwrap();
    term.write_line("  sysinfo [version=\"0.23.13\" license=\"MIT\"]").unwrap();
    term.write_line("  regex [version = \"1.5.6\" license=\"MIT / Apache-2.0\"]").unwrap();
    term.write_line("  zip [version=\"0.6.2\" license=\"MIT\"]").unwrap();
    term.write_line("").unwrap();

    term.write_line("Contact").unwrap();
    term.write_line("  [Github]https://github.com/VSMPNR").unwrap();
    term.write_line("  [ Steam]https://steamcommunity.com/profiles/76561198825527396/").unwrap();
    term.write_line("    If you modify this program, please rewrite your own contact information as well.").unwrap();
    term.write_line("").unwrap();
    term.write_line("").unwrap();
    term.write_line("").unwrap();

    term.write_line("Start EnemyDown Server Resource Install? (Select).").unwrap();
    let mut menu = component::Menu::new();
    menu.add("Install");
    menu.add("Cancel");
    let select = menu.select(1);
    term.move_cursor_up(1).unwrap();
    term.clear_to_end_of_screen().unwrap();
    if select == 1 {
        term.write_line("Cancel Select").unwrap();
        return true;
    }

    let csgo_path = if let Some(csgo_path) = get_csgo_path(&term) {
        csgo_path
    } else {
        term.write_line("").unwrap();
        term.write_line("The csgo.exe process could not be found").unwrap();
        return false
    };

    term.write_line("").unwrap();
    term.write_line("Start downloading Resource files").unwrap();
    let mut d = Downloader::new();
    d.download(DOWNLOAD_URL, ".\\download.zip").await;
    unzip(&term, ".\\download.zip", csgo_path.to_str().unwrap()).unwrap();

    return true;
}

fn get_csgo_path_from_steam_library_folders(term: &console::Term, steam: &str) -> Vec<std::path::PathBuf> {
    let regex = regex::Regex::new("^\\s+\"path\"\\s+\"(?P<directory>.+)\"$").unwrap();
    let mut directorys = Vec::<std::path::PathBuf>::new();
    let steam_config = std::path::PathBuf::from(steam).join("config\\libraryfolders.vdf");
    term.write_line(&format!("  SteamLibraryDirectory")).unwrap();
    for line in std::io::BufReader::new(std::fs::File::open(steam_config).unwrap()).lines() {
        if let Ok(line) = line {
            if let Some(caps) = regex.captures(&line) {
                let directory = caps["directory"].to_string();
                let directory = directory.replace("\\\\", "\\");

                let path = std::path::PathBuf::from(&directory).join("steamapps\\common\\Counter-Strike Global Offensive\\csgo");
                if path.is_dir() {
                    term.write_line(&format!("    [  valid]{}", path.to_str().unwrap())).unwrap();
                    directorys.push(path);
                } else {
                    term.write_line(&format!("    [invaild]{}", directory)).unwrap();
                }
            }
        }
    }

    directorys
}

fn get_csgo_path(term: &console::Term) -> Option<PathBuf> {
    term.write_line("Start getting the directory of CSGO.").unwrap();
    let sys = sysinfo::System::new_all();
    let processes = sys.processes();

    let mut csgo: Option<std::path::PathBuf> = None;
    let mut steam: Option<std::path::PathBuf> = None;

    for (_, process) in processes {
        match process.name() {
            "csgo.exe" => {
                csgo = Some(process.cwd().to_path_buf());
            },
            "steam.exe" => {
                steam = Some(process.cwd().to_path_buf());
            },
            _ => {}
        }
    }

    if let Some(csgo_path) = &mut csgo {
        csgo_path.push("csgo");
        if csgo_path.is_dir() {
            term.write_line(&format!("  Found path to CSGO. [{}]", csgo_path.to_str().unwrap())).unwrap();
            return csgo;
        } else {
            term.write_line(&format!("  Invalid path. [{}]", csgo_path.to_str().unwrap())).unwrap();
        }
    } else if let Some(steam_path) = steam {
        term.write_line(&format!("")).unwrap();
        term.write_line(&format!("CSGO process not found.")).unwrap();
        term.write_line(&format!("Locate the CSGO directory in the steam library folder.")).unwrap();
        let csgo_pathes = get_csgo_path_from_steam_library_folders(&term, steam_path.to_str().unwrap());
        
        let pathes_amount = csgo_pathes.len();
        if pathes_amount == 1 {
            return Some(csgo_pathes[0].clone());
        } else if pathes_amount > 1 {
            return None;
            todo!();
        } else {
            return None;
        }
    }
    term.write_line("  Unable to retrieve csgo.exe process.").unwrap();
    term.write_line("  Please make sure csgo is running!").unwrap();
    None
}

fn unzip(term: &console::Term, path: &str, out_dir: &str) -> std::io::Result<()>{
    let mut zip = zip::ZipArchive::new(std::io::BufReader::new(std::fs::File::open(path)?))?;
    let path_buf = std::path::PathBuf::from(out_dir);
    
    std::fs::create_dir_all(&path_buf)?;

    let skip  = console::Style::new().yellow().apply_to(" Skip").to_string();
    let done  = console::Style::new().green() .apply_to(" Done").to_string();
    let error = console::Style::new().red()   .apply_to("Error").to_string();

    let len = zip.len();

    let mut create_count = 0;
    let mut skip_count = 0;
    let mut error_count = 0;

    for i in 0..len {
        let mut file = zip.by_index(i)?;
        let zip_inside_path = file.enclosed_name().unwrap().to_path_buf();
        let out_path = path_buf.join(&zip_inside_path);

        let mut status;

        if file.is_dir() {
            if let Ok(_) = std::fs::create_dir(&out_path) {
                status = &done;
                create_count += 1;
            } else {
                status = &skip;
                skip_count += 1;
            }
        } else {
            if let Ok(mut out_file) = std::fs::OpenOptions::new().write(true).create_new(true).open(&out_path) {
                match std::io::copy(&mut file, &mut out_file) {
                    Ok(_) => {
                        status = &done;
                        if let Err(_) = out_file.flush() {
                            status = &error;
                            error_count += 1;
                        } else {
                            create_count += 1;
                        }
                    },
                    Err(_) => {
                        status = &error;
                        error_count += 1;
                    },
                }
            } else {
                status = &skip;
                skip_count += 1;
            }
        }

        term.write_line(&format!("[{}] [{:>5}/{:>5}] {}", status, i + 1, len, zip_inside_path.to_str().unwrap())).unwrap();
    }
    
    term.write_line(&format!("[Amount:{}, Create:{}, Skip:{}, Error:{}]", len, create_count, skip_count, error_count)).unwrap();

    Ok(())
}