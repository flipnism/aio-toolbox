use crate::settings::get_setting_for;
use crate::strct::{ConfigEnum, LoggingPayload, Thumbnail};
use crate::utils::{self, random_frame, random_str};
use serde_urlencoded;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::os::windows::process::CommandExt;
use std::path::Path;
use std::process::{Command, Stdio};
use tauri::AppHandle;
use tauri::Manager;

pub fn write_thumbnail(path: &str, content: Vec<Thumbnail>) {
    let json_string = serde_json::to_string_pretty(&content).expect("Failed writing content");
    let mut file = File::create(&path).expect("failed to write");
    file.write_all(json_string.as_bytes())
        .expect("failed to write into json file");
}

pub fn read_thumbnail(path: &str) -> Vec<Thumbnail> {
    let thumbnail_file = File::open(&path).expect("can't read the file");
    let val: Vec<Thumbnail> = serde_json::from_reader(thumbnail_file).expect("Error");
    val
}

pub async fn delete_thumbnail(psd_file: &str) {
    let path = Path::new(&psd_file);
    if let Some(parent) = &path.parent() {
        let thumbnail = Path::join(&parent, "thumbnail.json");
        let thumbnail = thumbnail.to_str().unwrap();
        let content = read_thumbnail(&thumbnail);
        let new_content: Vec<Thumbnail> = content
            .iter()
            .filter(|obj| obj.name != path.file_stem().unwrap().to_str().unwrap())
            .cloned()
            .collect();
        write_thumbnail(&thumbnail, new_content);
        match fs::remove_file(&path) {
            Ok(_) => println!("{:?} deleted!", &path),
            Err(_) => println!("fail to delete :{:?}", &path),
        }
    }
}

pub async fn add_thumbnail(psd_file: &str, app_handle: &AppHandle) {
    let path = Path::new(&psd_file);
    if let Some(parent) = path.parent() {
        let thumbnail = Path::join(&parent, "thumbnail.json");
        let thumbnail = thumbnail.to_str().unwrap();
        let mut content = read_thumbnail(&thumbnail);
        let filename = &path.file_stem().and_then(|stem| stem.to_str());
        let filename = filename.unwrap().to_string();
        content.push(Thumbnail {
            name: filename.clone(),
            thumbfile: format!("{}.png", filename),
        });
        convert_psd_to_png(&psd_file, &parent.to_str().unwrap(), app_handle).await;
        write_thumbnail(&thumbnail, content)
    }
}

pub async fn convert_psd_to_png(psd_file: &str, parent_dir: &str, app_handle: &AppHandle) {
    let psd_path = format!("\"{psd_file}[0]\"");
    let png_path = format!(
        "\"{}/thumbnails/{}.png\"",
        &parent_dir,
        Path::new(psd_file).file_stem().unwrap().to_str().unwrap()
    );
    let command = Command::new("pwsh")
        .args(&["-c", "magick", &psd_path, "-scale", "120", &png_path])
        .stdout(Stdio::piped())
        .creation_flags(0x08000000)
        .spawn()
        .expect("Error");
    if let Some(stdout) = command.stdout {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            let _ = app_handle.emit_all(
                "logger",
                &LoggingPayload {
                    message: line.unwrap(),
                },
            );
        }
    }
}

pub async fn create_emblem(new_data: &str, app_handle: &AppHandle) -> String {
    let py_cmd = r#""import random; import bpy; mfont = bpy.data.node_groups['textNode'].nodes['fontface']; mfont.font = bpy.data.fonts.load('"#.to_string()
        + get_setting_for(ConfigEnum::EmblemFont, app_handle).as_str()
        + r#"'); bpy.context.object.modifiers['textNodes']['Input_2'] = ('"#
        + new_data.to_string().replace('"', "'").as_str()
        + r#"').upper(); bpy.context.object.modifiers['textNodes']['Input_3'] = random.uniform(0.0,360.0); bpy.data.objects['text'].update_tag(); bpy.data.materials['textMat'].node_tree.nodes['textMat'].inputs['Base Color'].default_value=(1,0,0.02,1)""#;

    println!("{}", &py_cmd);
    let template = "text-node-eevee4.blend";
    let random_string = utils::random_str(6);
    let random_frame = utils::random_frame();
    let output_path = get_setting_for(ConfigEnum::BlenderOutputPath, &app_handle);
    let blender_path = get_setting_for(ConfigEnum::BlenderAssetPath, &app_handle);
    let file_name = format!("{}\\00001{}", &output_path, &random_string).to_string();

    let cmd = Command::new("pwsh")
        .current_dir(&blender_path)
        .args(&[
            "-c",
            "blender",
            "-b",
            template,
            "--python-expr",
            py_cmd.as_str(),
            "-o",
            &file_name,
            "-f",
            &random_frame,
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .creation_flags(0x08000000)
        .spawn()
        .expect("Error reading");
    if let Some(stdout) = cmd.stdout {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            println!("{:?}", &line);
            let _ = app_handle.emit_all(
                "logger",
                &LoggingPayload {
                    message: line.unwrap(),
                },
            );
        }
    }

    format!(
        "{}{:04}.png",
        file_name,
        random_frame.parse::<i32>().unwrap()
    )
    .to_string()
}

pub async fn send_image_to_whatsapp(new_data: &str, channel: &str, app_handle: &AppHandle) {
    let base_url = format!("{}/send", &get_setting_for(ConfigEnum::ApiUrl, app_handle));
    let params = [
        ("filepath", new_data),
        ("channel", channel),
        ("textdata", &"".to_string()),
    ];
    let url = format!(
        "{}?{}",
        base_url,
        serde_urlencoded::to_string(params).unwrap()
    );
    let _ = reqwest::get(&url).await;
}
