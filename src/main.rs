use std::env::consts::OS;
use std::env;
use std::fs::{File, copy};
use std::io::{Write, Error, Read, Seek, SeekFrom};
use std::path::{PathBuf, Path};
use std::string::String;

use dirs;

const WINDOWS_PATH: &str = "C:\\Users\\Public\\mod.io\\2475\\metadata";
const LINUX_LOCAL_PATH: &str = ".local/share/Steam/steamapps/compatdata/548430/pfx/drive_c/users/Public/mod.io/2475/metadata";
const STAGE_FILE: &str = "state.json";

fn get_default_path() -> Option<PathBuf>{
    if OS == "windows" {
        return Some(PathBuf::from(WINDOWS_PATH));
    } else if OS == "linux" {
        match dirs::home_dir() {
            Some(mut homedir) => {
                homedir.push(LINUX_LOCAL_PATH);
                return Some(homedir);
            },
            None => return None,
        }
    } else {
        return None;
    }
}

fn read_file_to_string(path: &PathBuf) -> Result<String, Error> {
    let mut file = File::options()
        .read(true)
        .open(path.to_str().unwrap())?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    return Ok(data);
}

fn write_to_file(path: &PathBuf, data: String) -> Result<(), Error>{
    let file = File::options()
        .read(true)
        .write(true)
        .truncate(true)
        .open(&path);
    file?.write_all(data.as_bytes())?;
    return Ok(());
}

fn create_backup(target_file: &PathBuf) -> Result<u64, Error>{
    let target = target_file.to_str().unwrap();
    let backup = target.to_owned() + ".bak";
    return copy(target, backup);
}

fn main() -> Result<(), Error> {
    let x = env::args().collect::<Vec<String>>();
    let mut path: Option<PathBuf> = None;

    if x.len() == 2 {
        let temp = PathBuf::from(&x[1]);
        if !temp.exists() && temp.file_name().unwrap() == STAGE_FILE {
            panic!("Given path does not exist!")
        }
        println!("using provided path");
        path = Some(temp);
    }
    if !path.is_some() {
        path = Some(get_default_path().expect("unable to get default path"));
    }
    let path = path.unwrap();
    // panic!("meep");

    // let mut path = get_default_path().expect("Unable to get default path");
    // path.push(STAGE_FILE);
    let path = PathBuf::from(path);
    println!("{:?}", &path);
    create_backup(&path)?;
    let mut file_contents = read_file_to_string(&path)?;
    // file_contents = file_contents.replace("Approved", "Verified");
    file_contents = file_contents.replace("Approved", "Verified");
    write_to_file(&path, file_contents)?;
    Ok(())
}
