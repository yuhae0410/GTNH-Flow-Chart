use serde_json::Value;
use std::{fs, path::{Path, PathBuf}};
use tauri::{AppHandle, Manager};

fn safe_name(name: &str) -> Result<String, String> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return Err("프로젝트 이름이 비어 있습니다.".into());
    }
    if trimmed.len() > 100 {
        return Err("프로젝트 이름이 너무 깁니다.".into());
    }
    if trimmed.chars().any(|c| matches!(c, '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|')) {
        return Err("프로젝트 이름에 파일명으로 사용할 수 없는 문자가 있습니다.".into());
    }
    Ok(trimmed.to_string())
}

fn projects_dir(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app.path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join("projects");
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir)
}

fn project_path(app: &AppHandle, name: &str) -> Result<PathBuf, String> {
    Ok(projects_dir(app)?.join(format!("{}.json", safe_name(name)?)))
}

fn atomic_write(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let tmp = path.with_extension("json.tmp");
    fs::write(&tmp, bytes).map_err(|e| e.to_string())?;
    if path.exists() {
        fs::remove_file(path).map_err(|e| e.to_string())?;
    }
    fs::rename(tmp, path).map_err(|e| e.to_string())
}

#[tauri::command]
fn save_project(app: AppHandle, name: String, data: Value) -> Result<(), String> {
    let path = project_path(&app, &name)?;
    let bytes = serde_json::to_vec_pretty(&data).map_err(|e| e.to_string())?;
    atomic_write(&path, &bytes)
}

#[tauri::command]
fn load_project(app: AppHandle, name: String) -> Result<Value, String> {
    let path = project_path(&app, &name)?;
    let bytes = fs::read(path).map_err(|e| e.to_string())?;
    serde_json::from_slice(&bytes).map_err(|e| e.to_string())
}

#[tauri::command]
fn list_projects(app: AppHandle) -> Result<Vec<String>, String> {
    let dir = projects_dir(&app)?;
    let mut names = Vec::new();
    for entry in fs::read_dir(dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.extension().and_then(|x| x.to_str()) == Some("json") {
            if let Some(stem) = path.file_stem().and_then(|x| x.to_str()) {
                names.push(stem.to_string());
            }
        }
    }
    names.sort_by_key(|s| s.to_lowercase());
    Ok(names)
}

#[tauri::command]
fn delete_project(app: AppHandle, name: String) -> Result<(), String> {
    let path = project_path(&app, &name)?;
    if path.exists() {
        fs::remove_file(path).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn export_backup(app: AppHandle) -> Result<String, String> {
    // 뼈대 단계: 앱 데이터 위치를 반환한다.
    // 다음 단계에서 ZIP 백업 + 복원 UI를 연결할 예정.
    let dir = projects_dir(&app)?;
    Ok(dir.to_string_lossy().to_string())
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .invoke_handler(tauri::generate_handler![
            save_project,
            load_project,
            list_projects,
            delete_project,
            export_backup
        ])
        .run(tauri::generate_context!())
        .expect("error while running GTNH Flow Chart");
}
