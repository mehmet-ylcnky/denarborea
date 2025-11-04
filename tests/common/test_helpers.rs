use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

pub struct TestFixture {
    pub temp_dir: TempDir,
    pub root_path: PathBuf,
}

impl TestFixture {
    pub fn new() -> Self {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let root_path = temp_dir.path().to_path_buf();
        Self {
            temp_dir,
            root_path,
        }
    }

    pub fn create_file(&self, path: &str, content: &str) -> PathBuf {
        let file_path = self.root_path.join(path);
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent).expect("Failed to create parent directories");
        }
        fs::write(&file_path, content).expect("Failed to write file");
        file_path
    }

    pub fn create_dir(&self, path: &str) -> PathBuf {
        let dir_path = self.root_path.join(path);
        fs::create_dir_all(&dir_path).expect("Failed to create directory");
        dir_path
    }

    pub fn create_symlink(&self, target: &str, link: &str) -> PathBuf {
        let link_path = self.root_path.join(link);
        let target_path = self.root_path.join(target);

        #[cfg(unix)]
        std::os::unix::fs::symlink(&target_path, &link_path).expect("Failed to create symlink");

        #[cfg(windows)]
        std::os::windows::fs::symlink_file(&target_path, &link_path)
            .expect("Failed to create symlink");

        link_path
    }

    pub fn path(&self) -> &Path {
        &self.root_path
    }
}

pub fn create_unicode_test_files(fixture: &TestFixture) {
    fixture.create_file("emoji_😀.txt", "content");
    fixture.create_file("unicode_ñáéíóú.txt", "content");
    fixture.create_file("chinese_中文.txt", "content");
    fixture.create_file("spaces in name.txt", "content");
    fixture.create_file("special!@#$%^&*().txt", "content");
}

pub fn create_deep_structure(fixture: &TestFixture, depth: usize) {
    let mut current_path = String::new();
    for i in 0..depth {
        current_path.push_str(&format!("level_{}/", i));
        fixture.create_dir(&current_path);
    }
    fixture.create_file(&format!("{}deep_file.txt", current_path), "deep content");
}

pub fn create_large_directory(fixture: &TestFixture, file_count: usize) {
    for i in 0..file_count {
        fixture.create_file(&format!("file_{:04}.txt", i), &format!("content {}", i));
    }
}
