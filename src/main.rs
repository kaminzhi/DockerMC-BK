use chrono::prelude::*;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    // 設置備份目錄
    let backup_dir = "../backups";
    fs::create_dir_all(backup_dir).unwrap();

    // 獲取當前時間
    let now: DateTime<Local> = Local::now();
    let backup_filename = format!(
        "{}/backup_{}.tar.gz",
        backup_dir,
        now.format("%Y%m%d_%H%M%S")
    );

    // 壓縮並備份 Minecraft 地圖
    let status = Command::new("tar")
        .args(&["-czf", &backup_filename, "./data"])
        .status()
        .expect("Failed to backup Minecraft world");

    if status.success() {
        println!("Backup successful: {}", backup_filename);
    } else {
        eprintln!("Backup failed");
    }
}

