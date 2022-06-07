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
    let success = match std::panic::catch_unwind(|| async { execute().await }) {
        Ok(fut) => fut.await,
        Err(_) => false,
    };

    if success {
        println!("");
        println!("");
        println!(" \x1b[32mSuccess\x1b[0m");
    } else {
        println!("");
        println!("");
        println!(" \x1b[31mFaild\x1b[0m");
    }

    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
}

const DOWNLOAD_URL: &str = "https://drive.google.com/uc?id=1t4_N2z03XWyeAn6IdmR3xfBE2qHuCzx-&confirm=t";
//const DOWNLOAD_URL: &str = "https://o16xyq.dm.files.1drv.com/y4mwX0rX0FZVKtuoUBu7lZtMJcirH9bKULQX1offK41a3Y4TgB8Ea5Ihgw6oeI_ykDXXmmhAUZi2OZ2Zts8tIyIs-UZ4h4xbZxZrpM19KNJrg_Df1ovZYHoUcCOqmrR1ML1ktnPxgPc87Hgh4P58QRNkay-pwwMqX-vJUnxqFRizITzAHUHqaJczX_QxYH1a0Rn8iR__Tq7m230uIrslQ9lvw";

async fn execute() -> bool {
    let term = console::Term::stdout();

    console::Style::new().black().apply_to("").to_string();
    println!("This program is distributed under the MIT License.");
    println!("  Copyright (c) 2022 VSMPNR");
    println!("  https://opensource.org/licenses/mit-license.php");
    println!("");

    println!("This program used these Crates.");
    println!("  tokio [version=\"1.19.0\" license=\"MIT\"]");
    println!("  reqwest [version=\"1.11.10\" license=\"MIT / Apache-2.0\"]");
    println!("  console [version=\"0.15.0\" license=\"MIT\"]");
    println!("  sysinfo [version=\"0.23.13\" license=\"MIT\"]");
    println!("  regex [version = \"1.5.6\" license=\"MIT / Apache-2.0\"]");
    println!("  zip [version=\"0.6.2\" license=\"MIT\"]");
    println!("");

    println!("Contact");
    println!("  [Github]https://github.com/VSMPNR");
    println!("  [ Steam]https://steamcommunity.com/profiles/76561198825527396/");
    println!("    If you modify this program, please rewrite your own contact information as well.");
    println!("");
    println!("");
    println!("");

    println!("Start EnemyDown Server Resource Install? (Left Right Enter).");
    let mut menu = component::Menu::new();
    //menu.add("ReInstall(overwrite)");
    menu.add("Install");
    menu.add("Cancel");
    let select = menu.select(1);
    term.move_cursor_up(1).unwrap();
    term.clear_to_end_of_screen().unwrap();

    if select == 1 {
        println!("Cancel Select");
        return true;
    }

    let csgo_path = if let Some(csgo_path) = get_csgo_path() {
        csgo_path
    } else {
        return false
    };

    println!("");
    println!("Start downloading Resource files");
    let mut d = Downloader::new();
    d.download(DOWNLOAD_URL, ".\\download.zip").await;
    unzip(".\\download.zip", csgo_path.to_str().unwrap(), false).unwrap();

    return true;
}

fn get_csgo_path_from_steam_library_folders(steam: &str) -> Vec<std::path::PathBuf> {
    let regex = regex::Regex::new("^\\s+\"path\"\\s+\"(?P<directory>.+)\"$").unwrap();
    let mut directorys = Vec::<std::path::PathBuf>::new();
    let steam_config = std::path::PathBuf::from(steam).join("config\\libraryfolders.vdf");
    println!("  Find SteamLibraryDirectory");
    for line in std::io::BufReader::new(std::fs::File::open(steam_config).unwrap()).lines() {
        if let Ok(line) = line {
            if let Some(caps) = regex.captures(&line) {
                let directory = caps["directory"].to_string();
                let directory = directory.replace("\\\\", "\\");

                let path = std::path::PathBuf::from(&directory).join("steamapps\\common\\Counter-Strike Global Offensive\\csgo");
                if path.is_dir() {
                    println!("    [\x1b[32m  valid\x1b[0m] {}", path.to_str().unwrap());
                    directorys.push(path);
                } else {
                    println!("    [\x1b[31minvaild\x1b[0m] {}", directory);
                }
            }
        }
    }

    directorys
}

fn get_csgo_path() -> Option<PathBuf> {
    println!("Start getting the directory of CSGO.");
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
            println!("  \x1b[32mFound path to CSGO. [{}]\x1b[0m", csgo_path.to_str().unwrap());
            return csgo;
        } else {
            println!("  \x1b[31mInvalid path. [{}]\x1b[0m", csgo_path.to_str().unwrap());
        }
    }

    if let Some(steam_path) = steam {
        println!("  \x1b[33mCSGO process not found.\x1b[0m");
        println!("  \x1b[33mLocate the CSGO directory in the steam library folder.\x1b[0m");
        let csgo_pathes = get_csgo_path_from_steam_library_folders(steam_path.to_str().unwrap());
        
        let pathes_amount = csgo_pathes.len();
        if pathes_amount == 1 {
            println!("    \x1b[32mFound path to CSGO. [{}]\x1b[0m", csgo_pathes[0].to_str().unwrap());
            return Some(csgo_pathes[0].clone());
        } else if pathes_amount > 1 {
            println!("    \x1b[31mMultiple csgo directories were found.\x1b[0m");
            for dir in csgo_pathes {
                println!("      {}", dir.to_str().unwrap());
            }
            return None;
        } else {
            println!("    \x1b[31mMCould not find the csgo directory.\x1b[0m");
            return None;
        }
    }
    println!("    \x1b[31mUnable to get csgo.exe process and steam.exe process.\x1b[0m");
    println!("    \x1b[31mPlease make sure that csgo or steam is running.\x1b[0m");
    None
}

fn unzip(path: &str, out_dir: &str, overwrite: bool) -> std::io::Result<()>{
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
            if let Ok(mut out_file) = std::fs::OpenOptions::new().write(true).create_new(!overwrite).open(&out_path) {
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

        println!("[{}] [{:>5}/{:>5}] {}", status, i + 1, len, zip_inside_path.to_str().unwrap());
    }
    
    println!("[Amount:{}, \x1b[32mCreate:{}\x1b[0m, \x1b[33mSkip:{}\x1b[0m, \x1b[31mError:{}\x1b[0m]", len, create_count, skip_count, error_count);

    Ok(())
}