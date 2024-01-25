// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dirs;
use lapin::{options::BasicPublishOptions, BasicProperties, Connection, ConnectionProperties};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, Error, Read};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use tauri::Manager;
use tokio::time::{sleep, Duration};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct AppState {
    rabbitmq_host: String,
    rabbitmq_username: String,
    rabbitmq_password: String,
    field_id: String,
    scorer_url: String,
    dark_statistic_url: String,
    light_statistic_url: String,
    man_of_the_match_url: String,
    top_player_url: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct ConfigState {
    file_path: String,
}

fn ensure_file_exists(file_path: &Path) -> Result<(), Error> {
    if !file_path.exists() {
        File::create(file_path)?;
        println!("File created: {}", file_path.display());
    } else {
        println!("File already exists: {}", file_path.display());
    }
    Ok(())
}

fn read_file(file_path: &Path) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn get_absolute_path_in_home(relative_path: &str) -> Result<PathBuf, String> {
    let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
    let absolute_path = home_dir.join(relative_path);
    Ok(absolute_path)
}

#[tauri::command]
async fn update_time(
    time: String,
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
) -> Result<(), String> {
    // println!("Publishing message: {}", time);
    let host = state.lock().unwrap().rabbitmq_host.to_string();
    let username = state.lock().unwrap().rabbitmq_username.to_string();
    let password = state.lock().unwrap().rabbitmq_password.to_string();

    //amqp://alifwr:alifsrabbit@alif.codes:5672
    let amqp_uri = format!("amqp://{}:{}@{}:5672", username, password, host); //state.lock().unwrap().rabbitmq_host.to_string(); // Replace with your URI

    // Connect to the AMQP server
    let conn = Connection::connect(&amqp_uri, ConnectionProperties::default())
        .await
        .expect("Failed to connect to AMQP server");

    let channel = conn
        .create_channel()
        .await
        .expect("Failed to create a channel");

    let queue_name = format!("basket.event.time.{}", state.lock().unwrap().field_id);

    // Publish a message
    channel
        .basic_publish(
            "amq.topic", // Exchange
            &queue_name, // Routing key (queue name)
            BasicPublishOptions::default(),
            &time.as_bytes().to_vec(),
            BasicProperties::default(),
        )
        .await
        .map_err(|e| e.to_string())?
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

// Tauri command to update the state
#[tauri::command]
fn save_config(
    rabbitmq_host: String,
    rabbitmq_username: String,
    rabbitmq_password: String,
    field_id: String,
    scorer_url: String,
    dark_statistic_url: String,
    light_statistic_url: String,
    man_of_the_match_url: String,
    top_player_url: String,
    app_state: tauri::State<'_, Arc<Mutex<AppState>>>,
    config_state: tauri::State<'_, Arc<Mutex<ConfigState>>>,
) -> Result<(), String> {
    let mut app_state = app_state.lock().unwrap();
    app_state.rabbitmq_host = rabbitmq_host;
    app_state.rabbitmq_username = rabbitmq_username;
    app_state.rabbitmq_password = rabbitmq_password;
    app_state.field_id = field_id;
    app_state.scorer_url = scorer_url;
    app_state.dark_statistic_url = dark_statistic_url;
    app_state.light_statistic_url = light_statistic_url;
    app_state.man_of_the_match_url = man_of_the_match_url;
    app_state.top_player_url = top_player_url;
    let relative_path = config_state.lock().unwrap().file_path.to_string();
    let file_path = get_absolute_path_in_home(&relative_path).unwrap();

    let _ = ensure_file_exists(&file_path);

    match serde_json::to_string(&*app_state) {
        Ok(serialized) => match std::fs::write(file_path, &serialized) {
            Ok(_) => {
                return Ok(());
            }
            Err(e) => {
                let err_msg = format!("Failed to write config file: {}", e);
                println!("{}", &err_msg);
                return Err(err_msg);
            }
        },
        Err(e) => {
            let err_msg = format!("Failed to serialize state: {}", e);
            println!("{}", &err_msg);
            return Err(err_msg);
        }
    };
    // Optionally, log or do something after updating the state
}

#[tauri::command]
fn get_config(state: tauri::State<'_, Arc<Mutex<AppState>>>) -> Result<AppState, String> {
    let state = state.lock().map_err(|e| e.to_string())?;
    Ok(state.clone())
}

#[tokio::main]
async fn main() {
    let config_state = ConfigState {
        file_path: "BasketScoreboard/settings.json".to_string(),
    };

    let file_path = get_absolute_path_in_home(&config_state.file_path).unwrap();

    let app_state = match read_file(&file_path) {
        Ok(contents) => match serde_json::from_str::<AppState>(&contents) {
            Ok(data) => {
                println!("Deserialized data: {:?}", data);
                data
            }
            Err(e) => {
                println!("Error parsing JSON: {}", e);
                AppState {
                    rabbitmq_host: "initial_host".to_string(),
                    rabbitmq_username: "initial_username".to_string(),
                    rabbitmq_password: "initial_password".to_string(),
                    field_id: "initial_field_id".to_string(),
                    scorer_url: "initial_scorer_url".to_string(),
                    dark_statistic_url: "initial_dark_statistic_url".to_string(),
                    light_statistic_url: "initial_light_statistic_url".to_string(),
                    man_of_the_match_url: "initial_man_of_the_match_url".to_string(),
                    top_player_url: "initial_top_player_url".to_string(),
                }
            }
        },
        Err(e) => {
            println!("Error reading file: {}", e);
            AppState {
                rabbitmq_host: "initial_host".to_string(),
                rabbitmq_username: "initial_username".to_string(),
                rabbitmq_password: "initial_password".to_string(),
                field_id: "initial_field_id".to_string(),
                scorer_url: "initial_scorer_url".to_string(),
                dark_statistic_url: "initial_dark_statistic_url".to_string(),
                light_statistic_url: "initial_light_statistic_url".to_string(),
                man_of_the_match_url: "initial_man_of_the_match_url".to_string(),
                top_player_url: "initial_top_player_url".to_string(),
            }
        }
    };

    tauri::Builder::default()
        .manage({
            let state = Arc::new(Mutex::new(app_state));
            state
        })
        .manage({
            let state = Arc::new(Mutex::new(config_state));
            state
        })
        .invoke_handler(tauri::generate_handler![
            update_time,
            save_config,
            get_config
        ])
        .setup(|app| {
            let splashscreen_window = app.get_window("splashscreen").unwrap();
            // let controller_window = app.get_window("controllerpage").unwrap();
            // let main_windows = app.get_window("indexpage").unwrap();
            let configuration_windows = app.get_window("configurationpage").unwrap();

            tauri::async_runtime::spawn(async move {
                sleep(Duration::from_secs(1)).await;

                splashscreen_window.close().unwrap();
                configuration_windows.show().unwrap();
                // main_windows.show().unwrap();
                // controller_window.show().unwrap();
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
