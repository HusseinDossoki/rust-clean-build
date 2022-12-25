use async_process::Command;
use futures::executor::block_on;
use std::env;
use std::fs::{self, metadata, DirEntry};
use std::path::Path;

fn main() {
    set_current_dir();

    block_on(run_cargo_build());

    create_bin_folder();

    move_binary_from_target();

    block_on(run_cargo_clean());
}

fn set_current_dir() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let working_dir = &args[1];
        std::env::set_current_dir(working_dir).expect("Invalid paramater");
    }
}

async fn run_cargo_build() {
    println!("\n'cargo build -r' is executing....\n");

    // Run "cargo build -r"
    Command::new("cargo")
        .arg("build")
        .arg("-r")
        .output()
        .await
        .expect("Erorr when run 'cargo build'");

    println!("\n'cargo build -r' has been completed\n");
}

async fn run_cargo_clean() {
    // Run "cargo clean"
    Command::new("cargo")
        .arg("clean")
        .output()
        .await
        .expect("Erorr when run 'cargo clean'");

    println!("\n'cargo clean' has been completed\n");
}

fn create_bin_folder() {
    if Path::new("bin").exists() {
        fs::remove_dir_all("bin").unwrap();
    }

    // Create the new dir
    // create_dir_all to skip if the dir is already exists
    fs::create_dir_all("bin").unwrap();
}

fn move_binary_from_target() {
    let release_folder = "./target/release";
    for path in fs::read_dir(release_folder).unwrap() {
        let entry = &path.unwrap();
        if is_binary_file(entry) {
            let full_path = entry.path().display().to_string();
            let file_name = entry.file_name().into_string().unwrap();
            fs::copy(full_path, format!("bin/{}", file_name))
                .expect("error when move the exeutable");
        }
    }
}

fn is_binary_file(entry: &DirEntry) -> bool {
    let full_path = entry.path().display().to_string();
    let file_name = entry.file_name().into_string().unwrap();
    let metadata = metadata(&full_path).unwrap();

    if metadata.is_file() && !file_name.starts_with('.') && !file_name.ends_with(".d") {
        return true;
    }
    return false;
}
