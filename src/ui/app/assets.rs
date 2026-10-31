use std::path::{Path, PathBuf};

/// 開発時はプロジェクト直下、配布時はexeの隣のassetsを使う。
/// 起動時のカレントディレクトリに依存させない。
fn image_directory(executable: &Path, project: &Path) -> PathBuf {
    if executable.starts_with(project.join("target")) {
        project.join("assets")
    } else {
        executable.parent().unwrap_or(Path::new(".")).join("assets")
    }
}

pub(super) fn pick_character_image() -> std::io::Result<Option<PathBuf>> {
    let directory = image_directory(
        &std::env::current_exe()?,
        Path::new(env!("CARGO_MANIFEST_DIR")),
    );
    std::fs::create_dir_all(&directory)?;
    Ok(rfd::FileDialog::new()
        .set_title("キャラクターの画像を選択")
        .set_directory(directory)
        .add_filter("画像", &["png", "jpg", "jpeg", "gif", "webp"])
        .pick_file())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn development_and_test_binaries_use_project_assets() {
        let project = Path::new("project");
        for executable in [
            "target/debug/app.exe",
            "target/release/app.exe",
            "target/debug/deps/test.exe",
        ] {
            assert_eq!(
                image_directory(&project.join(executable), project),
                project.join("assets")
            );
        }
    }

    #[test]
    fn distributed_app_uses_assets_next_to_executable() {
        assert_eq!(
            image_directory(Path::new("exhibition/app.exe"), Path::new("project")),
            Path::new("exhibition/assets")
        );
    }
}
