use docx_rs::{Docx, Paragraph};
use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use tauri::State;
use std::sync::Mutex;
use std::fs::File;
use std::io::Read;
use std::io::prelude::*;
use zip::ZipArchive;
use zip::write::{FileOptions, ExtendedFileOptions}; 
use std::fs::metadata;

// 替换字典的结构
#[derive(Debug, Serialize, Deserialize)]
struct Replacement {
    id: i32,
    find: String,
    replace: String,
}

// 全局状态，用于管理字典
struct AppState {
    db: Mutex<Connection>,
}

impl AppState {
    fn new() -> Self {
        let conn = Connection::open("replacements.db").expect("Failed to open DB");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS replacements (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                find TEXT NOT NULL UNIQUE,
                replace TEXT NOT NULL
            )",
            [],
        ).expect("Failed to create table");
        Self { db: Mutex::new(conn) }
    }

    fn update_db(&self, file_path: &str) -> Result<(), String> {
        let conn = Connection::open(file_path).map_err(|e| e.to_string())?;
        *self.db.lock().unwrap() = conn;
        Ok(())
    }
}

// 增加替换词
#[tauri::command]
fn add_replacement(state: State<AppState>, find: String, replace: String) -> Result<String, String> {
    if find.chars().count() != replace.chars().count() {
        println!("find: {} unicode: {:?} length: {}", find, find.chars().map(|c| c.escape_unicode().to_string()).collect::<Vec<_>>(), find.chars().count());
        println!("replace: {} unicode: {:?} length: {}", replace, replace.chars().map(|c| c.escape_unicode().to_string()).collect::<Vec<_>>(), replace.chars().count());
        return Err("替换前后的字符串长度必须相等".to_string());

    }

    let db = state.db.lock().unwrap();

    // 检查包含关系
    let mut stmt = db.prepare("SELECT find FROM replacements WHERE ? LIKE '%' || find || '%' OR find LIKE '%' || ? || '%'").unwrap();
    let mut rows = stmt.query(params![find, find]).unwrap();

    if rows.next().unwrap().is_some() {
        return Err("不能添加含有现有字符串的替换项".to_string());
    }

    let mut stmtr = db.prepare("SELECT replace FROM replacements WHERE ? LIKE '%' || replace || '%' OR replace LIKE '%' || ? || '%'").unwrap();
    let mut rowsr = stmtr.query(params![replace, replace]).unwrap();

    if rowsr.next().unwrap().is_some() {
        return Ok("不能添加含有现有字符串的替换项".to_string());
    }

    db.execute("INSERT INTO replacements (find, replace) VALUES (?1, ?2)", params![find, replace])
        .map_err(|e| e.to_string())?;
    Ok("替换项添加成功".to_string())
}

// 删除替换词
#[tauri::command]
fn delete_replacement(state: State<AppState>, id: i32) -> Result<String, String> {
    let db = state.db.lock().unwrap();
    db.execute("DELETE FROM replacements WHERE id = ?", params![id])
        .map_err(|e| e.to_string())?;
    Ok("删除成功".to_string())
}

// 查询替换词
#[tauri::command]
fn get_replacements(state: State<AppState>) -> Result<Vec<Replacement>, String> {
    let db = state.db.lock().unwrap();
    let mut stmt = db.prepare("SELECT id, find, replace FROM replacements").unwrap();
    let replacement_iter = stmt.query_map([], |row| {
        Ok(Replacement {
            id: row.get(0)?,
            find: row.get(1)?,
            replace: row.get(2)?,
        })
    }).map_err(|e| e.to_string())?;

    let mut replacements = Vec::new();
    for replacement in replacement_iter {
        replacements.push(replacement.unwrap());
    }
    Ok(replacements)
}

// 替换并保存文件
#[tauri::command]
fn process_docx(state: State<AppState>, file_path: String, reverse: bool) -> Result<String, String> {
    let db = state.db.lock().unwrap();

    // 获取替换词典
    let mut stmt = db.prepare("SELECT find, replace FROM replacements").unwrap();
    let mut replacements = Vec::new();
    let mut rows = stmt.query([]).map_err(|e| e.to_string())?;
    while let Some(row) = rows.next().map_err(|e| e.to_string())? {
        let original_word: String = row.get(0).map_err(|e| e.to_string())?;
        let replacement_word: String = row.get(1).map_err(|e| e.to_string())?;
        if reverse {
            replacements.push((replacement_word, original_word)); // 反向替换
        } else {
            replacements.push((original_word, replacement_word)); // 正向替换
        }
    }

    // 读取 DOCX 文件
    let mut file = File::open(&file_path).map_err(|e| e.to_string())?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).map_err(|e| e.to_string())?;

    // 处理 DOCX 文件
    let suffix = if reverse { "_reversed" } else { "_processed" };
    let new_file_path = format!("{}{}.docx", file_path, suffix);
    let process_handle = {
        let new_file_path_cloned = new_file_path.clone();
        
        let mut zipm = zip::ZipArchive::new(std::io::Cursor::new(buffer)).map_err(|e| e.to_string())?;

        // 读取 document.xml
        let mut document_xml = {
            let mut file = zipm.by_name("word/document.xml").map_err(|e| e.to_string())?;
            let mut xml_content = String::new();
            file.read_to_string(&mut xml_content).map_err(|e| e.to_string())?;
            xml_content
        };

        // 替换词语
        for (original, replacement) in replacements {
            document_xml = document_xml.replace(&original, &replacement);
        }

        // 写回 document.xml
        let mut buffer = Vec::new();
        {
            let mut writer = zip::ZipWriter::new(std::io::Cursor::new(&mut buffer));
            for i in 0..zipm.len() {
                let mut file = zipm.by_index(i).map_err(|e| e.to_string())?;
                let compression_method = file.compression();
                let options = zip::write::FileOptions::<ExtendedFileOptions>::default()
                    .compression_method(compression_method);
                let name = file.name().to_string();

                if name == "word/document.xml" {
                    writer.start_file(name, options).map_err(|e| e.to_string())?;
                    writer.write_all(document_xml.as_bytes()).map_err(|e| e.to_string())?;
                } else {
                    writer.start_file(name, options).map_err(|e| e.to_string())?;
                    std::io::copy(&mut file, &mut writer).map_err(|e| e.to_string())?;
                }
            }
            writer.finish().map_err(|e| e.to_string())?;
        }

        // 保存处理后的文件
        let mut output_file = File::create(&new_file_path_cloned).map_err(|e| e.to_string())?;
        output_file.write_all(&buffer).map_err(|e| e.to_string())?;
        output_file.sync_all().map_err(|e| e.to_string())?;
        Ok::<(), String>(())
    };

    process_handle.map_err(|e| e.to_string())?; // 确保处理完成
    println!("new_file_path: {}", new_file_path);
    Ok(new_file_path)
}

#[tauri::command]
fn handle_dictionary_file(state: State<AppState>, file_path: String) -> Result<String, String> {
    let file_metadata = metadata(&file_path).map_err(|e| e.to_string())?;
    
    // 检查文件是否为空
    if file_metadata.len() == 0 {
        // 打开连接并创建表
        let conn = Connection::open(&file_path).map_err(|e| e.to_string())?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS replacements (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                find TEXT NOT NULL UNIQUE,
                replace TEXT NOT NULL
            )",
            [],
        ).map_err(|e| e.to_string())?;
    }

    // 更新 AppState 中的数据库连接
    state.update_db(&file_path)?;

    Ok("字典文件已加载".to_string())
}

#[tauri::command]
fn replace_text_with_db(state: State<AppState>, input: String, reverse: bool) -> Result<String, String> {
    let db = state.db.lock().unwrap();

    // 获取替换词典
    let mut stmt = db.prepare("SELECT find, replace FROM replacements").unwrap();
    let mut replacements = Vec::new();
    let mut rows = stmt.query([]).map_err(|e| e.to_string())?;
    while let Some(row) = rows.next().map_err(|e| e.to_string())? {
        let original_word: String = row.get(0).map_err(|e| e.to_string())?;
        let replacement_word: String = row.get(1).map_err(|e| e.to_string())?;
        if reverse {
            replacements.push((replacement_word, original_word)); // 反向替换
        } else {
            replacements.push((original_word, replacement_word)); // 正向替换
        }
    }

    // 替换文本
    let mut output = input;
    for (find, replace) in replacements {
        output = output.replace(&find, &replace);
    }

    Ok(output)
}

// fn main() {
//     tauri::Builder::default()



//         .manage(AppState::new())
//         .invoke_handler(tauri::generate_handler![
//             add_replacement,
//             delete_replacement,
//             get_replacements,
//             process_docx
//         ])
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");
// }

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
    .manage(AppState::new())
    .invoke_handler(tauri::generate_handler![
        add_replacement,
        delete_replacement,
        get_replacements,
        process_docx,
        handle_dictionary_file,
        replace_text_with_db
    ])
    .plugin(tauri_plugin_fs::init())
    .plugin(tauri_plugin_dialog::init())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
