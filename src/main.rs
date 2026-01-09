#![cfg_attr(windows, windows_subsystem = "windows")]

use walkdir::{WalkDir, DirEntry};
use std::{error::Error, fs::{File, read_to_string, remove_file}, path::{Path, PathBuf}};
use regex::{ RegexSet};
use std::io::{copy};
use zip::{ZipWriter, write::FileOptions,CompressionMethod::Deflated};
use std::env;
use std::io::stdin;

#[cfg(windows)]
use winrt_notification::{Toast,Sound,Duration};

#[cfg(unix)]
use notify_rust::Notification;

struct Blacklists {
    dirs:RegexSet,
    general: RegexSet
}

fn main() {

    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty(){
        show_error("No path was provided to create a zip!");
        pause_console();
        return;
    }
    let raw_arg = args.get(0).unwrap();

    let target_path = if raw_arg == "." {
       env::current_dir().expect("Could not get current dir")
    } else {
        PathBuf::from(raw_arg).canonicalize().unwrap_or(PathBuf::from(raw_arg))
    };


    let exe_path = env::current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();
    let ignores_path = exe_dir.join("ignores.txt");


    show_toast("Initializing Compression", &format!("You'll be notified when process is finished"));


    match blacklist_generator(ignores_path.to_str().unwrap_or("")) {
        Ok(blacklist) => {
            match entry_iterator(target_path.to_str().unwrap_or(""), blacklist) {
                Ok(_) => show_toast("Compression Finished", &format!("Zip can be found at following path: {}", target_path.to_str().unwrap())),
                Err(error) => show_error(&format!("There has been a problem while compressing the selected file(s):\n\n{}", error))
            }
        },
        Err(error) => eprintln!("{}", error)
    }
}


fn blacklist_generator(ignores_path:&str) -> Result<Blacklists, Box<dyn Error>>{
    let ignores_content = read_to_string(ignores_path)?;

    let mut dir_rules = Vec::new();
    let mut general_rules = Vec::new();

     for line in ignores_content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("#"){
           continue; 
        }
        if trimmed.ends_with("/") {
            let rule_without_slash = &trimmed[..trimmed.len()-1];
            dir_rules.push(rule_without_slash);
        }else {
            general_rules.push(trimmed);
        }
        
      
    }
    
    Ok(Blacklists {
        dirs: RegexSet::new(dir_rules)?,
        general: RegexSet::new(general_rules)?,
    })
}

fn entry_iterator(dir_path:&str, blacklist: Blacklists) -> Result<(), Box<dyn Error>> {
    
    let dir_path = Path::new(dir_path);

    let walker = WalkDir::new(dir_path).into_iter();
    let it_filtered = walker.filter_entry(|e| !is_ignored(e, &blacklist));


    let file_options = FileOptions::<()>::default().compression_method(Deflated).unix_permissions(0o755);


    let name_zip = format!("{}.zip", dir_path.display());

    let file = File::create(&name_zip)?;
    let mut zip = ZipWriter::new(file);


    let base_prefix = if dir_path.is_file() {
        dir_path.parent().unwrap_or(Path::new("."))
    }else {
        dir_path
    };


    let zip_copression = || -> Result<(), Box<dyn Error>> {
        for entry in it_filtered {
        let entry = entry?;
        let path = entry.path();
        
        let relative_path = path.strip_prefix(base_prefix).unwrap_or(path);
        let name_string = relative_path.to_str().unwrap_or("").to_string();
        let name = name_string.replace('\\', "/");

    
        if path.is_file() {
            zip.start_file(name, file_options)?;
            let mut f = File::open(path)?;
            copy(&mut f, &mut zip)?;
        } else if !name.is_empty(){
            zip.add_directory(name, file_options)?;
        }
        }
    Ok(())
    };

    if let Err(e) = zip_copression() {
        drop(zip);
        remove_file(&name_zip)?;
        return Err(e);
    }

    zip.finish()?;

    Ok(())
}

fn is_ignored(entry: &DirEntry, blacklist: &Blacklists) -> bool {
    let file_name = entry.file_name();
    let name = file_name.to_str().unwrap_or("");
    let is_dir = entry.file_type().is_dir();

    if blacklist.general.is_match(name) {
        return true;
    }

    if is_dir && blacklist.dirs.is_match(name) {
        return true;
    }
    false
}


fn pause_console(){
    println!("\nPress Enter to escape the program...");
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
}

fn show_error(msg:&str) {
    let _ = msgbox::create("Compression Error", msg, msgbox::IconType::Error);
}

#[cfg(windows)]
fn show_toast(title: &str, msg: &str) {
    Toast::new(Toast::POWERSHELL_APP_ID)
        .title(title)
        .text1(msg)
        .sound(Some(Sound::SMS))
        .duration(Duration::Short)
        .show()
        .ok(); // Ignoramos errores de notificación (no son críticos)
}

#[cfg(unix)]
fn show_toast(title: &str, msg: &str) {
    let _ = Notification::new().summary(title).body(msg).show();
}