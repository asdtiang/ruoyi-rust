use std::path::{Path, PathBuf};

pub fn find_files_with_extension(dir: &Path, extension: &str) -> std::io::Result<Vec<PathBuf>> {
    let mut matching_files = Vec::new();

    if dir.is_dir() {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                // 递归处理子目录
                let mut subdir_files = find_files_with_extension(&path, extension)?;
                matching_files.append(&mut subdir_files);
            } else if let Some(ext) = path.extension() {
                // 检查文件扩展名是否匹配
                if ext == extension {
                    matching_files.push(path);
                }
            }
        }
    }

    Ok(matching_files)
}

#[test]

fn test_find_files_with_extension() {
    println!("{:?}", find_files_with_extension(Path::new("D:\\Project\\A-Ruoyi\\axum\\ruoyi-rust\\template\\rust"), "jinja"));
}