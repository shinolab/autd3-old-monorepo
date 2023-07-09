// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod options;

use options::Options;

use std::{path::PathBuf, process::Stdio};

use tauri::Manager;

use tokio::{
    fs::{File, OpenOptions},
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    process::Command,
    sync::mpsc::{channel, Sender},
};

const SETTINGS_PATH: &str = "settings.json";

fn get_settings_file_path(handle: &tauri::AppHandle) -> std::io::Result<PathBuf> {
    let mut path = handle
        .app_handle()
        .path_resolver()
        .app_config_dir()
        .unwrap_or_default();
    if !path.exists() {
        std::fs::create_dir_all(&path)?;
    }
    path.push(SETTINGS_PATH);
    Ok(path)
}

#[tauri::command]
async fn load_settings(handle: tauri::AppHandle) -> Result<Options, String> {
    let options: Options = if let Ok(mut file) =
        File::open(get_settings_file_path(&handle).map_err(|e| e.to_string())?).await
    {
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .await
            .map_err(|e| e.to_string())?;
        serde_json::from_str(&contents).map_err(|e| e.to_string())?
    } else {
        Default::default()
    };
    Ok(options)
}

#[tauri::command]
async fn save_settings(handle: tauri::AppHandle, options: &str) -> Result<(), String> {
    let options: Options = serde_json::from_str(options).map_err(|e| e.to_string())?;
    let json = serde_json::to_string_pretty(&options).map_err(|e| e.to_string())?;
    let mut file = match OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(get_settings_file_path(&handle).map_err(|e| e.to_string())?)
        .await
    {
        Ok(file) => file,
        Err(_) => File::create(get_settings_file_path(&handle).map_err(|e| e.to_string())?)
            .await
            .map_err(|e| e.to_string())?,
    };
    file.write_all(json.as_bytes())
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn fetch_ifnames() -> Vec<String> {
    autd3_link_soem::EthernetAdapters::new()
        .into_iter()
        .map(|adapter| adapter.to_string())
        .collect()
}

#[tauri::command]
async fn copy_autd_xml(handle: tauri::AppHandle) -> Result<(), String> {
    let dst = std::path::Path::new("C:/TwinCAT/3.1/Config/Io/EtherCAT/AUTD.xml");

    if dst.exists() {
        return Ok(());
    }

    if dst.parent().map_or(false, |p| !p.exists()) {
        return Err("TwinCAT is not installed".to_string());
    }

    let autd_xml_path = handle
        .path_resolver()
        .resolve_resource("TwinCATAUTDServer/AUTD.xml")
        .ok_or("Can't find AUTD.xml")?;

    tokio::fs::copy(autd_xml_path, dst)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
async fn run_twincat_server(
    twincat_options: &str,
    handle: tauri::AppHandle,
    console_emu_input_tx: tauri::State<'_, Sender<String>>,
) -> Result<(), String> {
    let twincat_autd_server_path = handle
        .path_resolver()
        .resolve_resource("TwinCATAUTDServer/TwinCATAUTDServer.exe")
        .ok_or("Can't find TwinCATAUTDServer.exe")?;

    let twincat_options: options::TwinCATOptions =
        serde_json::from_str(twincat_options).map_err(|e| e.to_string())?;

    let mut args = vec![
        "-c".to_string(),
        twincat_options.client,
        "-s".to_string(),
        twincat_options.sync0.to_string(),
        "-t".to_string(),
        twincat_options.task.to_string(),
        "-b".to_string(),
        twincat_options.base.to_string(),
        "-m".to_string(),
        match twincat_options.mode {
            autd3_link_soem::SyncMode::DC => "DC".to_string(),
            autd3_link_soem::SyncMode::FreeRun => "FreeRun".to_string(),
        },
    ];
    if twincat_options.keep {
        args.push("-k".to_string());
    }
    let mut child = Command::new(&twincat_autd_server_path)
        .args(args)
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;

    let stdout = child.stdout.take().ok_or("Failed to open stdout")?;
    let mut reader = BufReader::new(stdout);

    loop {
        let mut buf = String::new();
        if reader.read_line(&mut buf).await.unwrap() == 0 {
            break;
        }
        console_emu_input_tx
            .send(buf.trim().to_string())
            .await
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    tauri::async_runtime::set(tokio::runtime::Handle::current());

    let (console_emu_input_tx, mut console_emu_input_rx) = channel::<String>(32);

    tauri::Builder::default()
        .manage(console_emu_input_tx)
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }

            let app_handle = app.app_handle();
            tokio::spawn(async move {
                while let Some(s) = console_emu_input_rx.recv().await {
                    app_handle.emit_all("console-emu", s).unwrap();
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            load_settings,
            save_settings,
            fetch_ifnames,
            copy_autd_xml,
            run_twincat_server
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
