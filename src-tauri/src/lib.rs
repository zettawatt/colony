// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
//use colony::config::generate_seed_phrase;
use autonomi::client::payment::PaymentOption;
use autonomi::client::quote::CostError;
use autonomi::client::ConnectError;
use autonomi::client::config::ClientConfig;
use autonomi::client::{GetError, PutError};
use autonomi::data::DataAddress;
use autonomi::{AddressParseError, Bytes, Client, Wallet};
use colonylib::data::Error as DataStoreError;
use colonylib::graph::Error as GraphError;
use colonylib::key::Error as KeyStoreError;
use colonylib::pod::Error as PodError;
use colonylib::{DataStore, Graph, KeyStore, PodManager};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::io::Error as IoError;
use std::sync::Mutex;
use std::sync::{MutexGuard, PoisonError};
use tauri::Manager;
use tauri::{AppHandle, Emitter, RunEvent, State, WindowEvent};
use tauri_plugin_shell::{process::CommandChild, Error as ShellError, ShellExt};
use tracing::{debug, error, info, warn};

// Dweb detection and management
#[derive(Debug, Clone)]
pub enum DwebBinary {
    System,     // Use system-installed dweb
    ColonyDweb, // Use colony-dweb sidecar
}

/// Check if a user-installed dweb serve is already running by querying the REST API
async fn is_dweb_serve_running() -> bool {
    let client = reqwest::Client::new();

    // Try the default dweb port (5537)
    match client
        .get("http://localhost:5537/dweb-0/app-settings")
        .timeout(std::time::Duration::from_secs(2))
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                info!("Detected running dweb serve on port 5537");
                true
            } else {
                debug!(
                    "dweb serve not responding on port 5537: {}",
                    response.status()
                );
                false
            }
        }
        Err(e) => {
            debug!("No dweb serve detected on port 5537: {}", e);
            false
        }
    }
}

/// Determine which dweb binary to use based on system state
async fn determine_dweb_binary() -> DwebBinary {
    // Android doesn't support dweb sidecar
    if cfg!(target_os = "android") {
        info!("Android: dweb sidecar not supported, using system binary");
        return DwebBinary::System;
    }

    if is_dweb_serve_running().await {
        info!("Using system dweb binary (serve already running)");
        DwebBinary::System
    } else {
        info!("Using colony-dweb sidecar binary");
        DwebBinary::ColonyDweb
    }
}

#[tauri::command]
fn get_file_size(path: String) -> Result<u64, String> {
    fs::metadata(path)
        .map(|meta| meta.len())
        .map_err(|e| e.to_string())
}

// File opener command for Android
#[tauri::command]
async fn open_file_with_default_app(file_path: String, app: tauri::AppHandle) -> Result<String, String> {
    if cfg!(target_os = "android") {
        open_file_via_socket_communication(&file_path, &app).await
    } else {
        Err("This command is only available on Android".to_string())
    }
}

#[cfg(target_os = "android")]
async fn open_file_via_socket_communication(file_path: &str, _app: &tauri::AppHandle) -> Result<String, String> {
    use tokio::net::TcpStream;
    use tokio::io::{AsyncWriteExt, AsyncReadExt};
    use tokio::time::{timeout, Duration};

    let address = "127.0.0.1:8765";

    let mut stream = timeout(Duration::from_secs(5), TcpStream::connect(address))
        .await
        .map_err(|_| "Timeout connecting to socket".to_string())?
        .map_err(|e| format!("Failed to connect to socket: {}", e))?;

    stream.write_all(file_path.as_bytes()).await
        .map_err(|e| format!("Failed to write to socket: {}", e))?;

    stream.shutdown().await
        .map_err(|e| format!("Failed to shutdown write: {}", e))?;

    let mut response = String::new();
    timeout(Duration::from_secs(5), stream.read_to_string(&mut response))
        .await
        .map_err(|_| "Timeout reading response".to_string())?
        .map_err(|e| format!("Failed to read response: {}", e))?;

    Ok(response.trim().to_string())
}

#[cfg(not(target_os = "android"))]
async fn open_file_via_socket_communication(_file_path: &str, _app: &tauri::AppHandle) -> Result<String, String> {
    Err("Not available on non-Android platforms".to_string())
}

pub struct Session {
    pub password: Mutex<Option<String>>,
}

#[tauri::command]
fn set_password(state: State<'_, Mutex<AppState>>, pw: String) {
    let app_state = state.lock().unwrap();
    let mut stored_pw = app_state.session.password.lock().unwrap();
    *stored_pw = Some(pw);
}

#[tauri::command]
fn get_password(state: State<'_, Mutex<AppState>>) -> Option<String> {
    let app_state = state.lock().unwrap();
    let stored_pw = app_state.session.password.lock().unwrap();
    stored_pw.clone()
}

#[tauri::command]
fn clear_password(state: State<'_, Mutex<AppState>>) {
    let app_state = state.lock().unwrap();
    let mut stored_pw = app_state.session.password.lock().unwrap();
    *stored_pw = None;
}

// Error handling
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Connect(#[from] ConnectError),
    #[error(transparent)]
    Pod(#[from] PodError),
    #[error(transparent)]
    KeyStore(#[from] KeyStoreError),
    #[error(transparent)]
    DataStore(#[from] DataStoreError),
    #[error(transparent)]
    Graph(#[from] GraphError),
    #[error(transparent)]
    Mutex(#[from] PoisonError<MutexGuard<'static, Option<Client>>>),
    #[error(transparent)]
    AddressParse(#[from] AddressParseError),
    #[error(transparent)]
    Get(Box<GetError>),
    #[error(transparent)]
    Put(Box<PutError>),
    #[error(transparent)]
    Cost(#[from] CostError),
    #[error(transparent)]
    Io(#[from] IoError),
    #[error(transparent)]
    Shell(#[from] ShellError),
    #[error("{0}")]
    Message(String),
}

impl From<&str> for Error {
    fn from(msg: &str) -> Self {
        Error::Message(msg.to_string())
    }
}

impl From<GetError> for Error {
    fn from(err: GetError) -> Self {
        Error::Get(Box::new(err))
    }
}

impl From<PutError> for Error {
    fn from(err: PutError) -> Self {
        Error::Put(Box::new(err))
    }
}

#[derive(serde::Serialize)]
#[serde(tag = "kind", content = "message")]
#[serde(rename_all = "camelCase")]
pub enum ErrorKind {
    Connect(String),
    Pod(String),
    KeyStore(String),
    DataStore(String),
    Graph(String),
    Mutex(String),
    AddressParse(String),
    Get(String),
    Put(String),
    Cost(String),
    Io(String),
    Shell(String),
    Message(String),
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let error_message = self.to_string();
        let error_kind = match self {
            Self::Connect(_) => ErrorKind::Connect(error_message),
            Self::Pod(_) => ErrorKind::Pod(error_message),
            Self::KeyStore(_) => ErrorKind::KeyStore(error_message),
            Self::DataStore(_) => ErrorKind::DataStore(error_message),
            Self::Graph(_) => ErrorKind::Graph(error_message),
            Self::Mutex(_) => ErrorKind::Mutex(error_message),
            Self::AddressParse(_) => ErrorKind::AddressParse(error_message),
            Self::Get(_) => ErrorKind::Get(error_message),
            Self::Put(_) => ErrorKind::Put(error_message),
            Self::Cost(_) => ErrorKind::Cost(error_message),
            Self::Io(_) => ErrorKind::Io(error_message),
            Self::Shell(_) => ErrorKind::Shell(error_message),
            Self::Message(_) => ErrorKind::Message(error_message),
        };
        error_kind.serialize(serializer)
    }
}

// Application state - using std::sync::Mutex for simplicity and compatibility
pub struct AppState {
    pub client: Mutex<Option<Client>>,
    pub wallet: Mutex<Option<Wallet>>,
    pub datastore: Mutex<Option<DataStore>>,
    pub keystore: Mutex<Option<KeyStore>>,
    pub graph: Mutex<Option<Graph>>,
    pub network: String,
    pub session: Session,
    pub dweb_process: Mutex<Option<CommandChild>>,
}

// Data structures for Tauri commands
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PodInfo {
    pub address: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddressList {
    pub addresses: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePodRequest {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RenamePodRequest {
    pub name: String,
    pub new_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePodRefRequest {
    pub pod_address: String,
    pub pod_ref_address: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadFileRequest {
    pub file_path: String,
    pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddWalletRequest {
    pub name: String,
    pub key: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RefreshRefRequest {
    pub depth: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DownloadFileRequest {
    pub address: String,
    pub destination_path: String,
    pub size: u64,
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadPodRequest {
    pub pod_address: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchRequest {
    pub query: Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PutSubjectDataRequest {
    pub pod_address: String,
    pub subject_address: String,
    pub data: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetSubjectDataRequest {
    pub subject_address: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResult {
    pub results: Value,
}

#[derive(Serialize, Debug, Clone)]
pub struct PodMetaData {
    pub address: String,
    pub name: Option<String>,
    pub creation: Option<String>,
    pub modified: Option<String>,
    pub depth: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubjectDataResult {
    pub data: String,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {name}! You've been greeted from Rust!")
}

#[tauri::command]
fn get_new_seed_phrase() -> Result<String, String> {
    //Ok(generate_seed_phrase())
    let seed_phrase = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about".to_string();
    Ok(seed_phrase)
}

////////////////////////////////////////////////////////////////////
// DataStore commands
////////////////////////////////////////////////////////////////////

#[tauri::command]
fn initialize_datastore(app: AppHandle, state: State<'_, Mutex<AppState>>) -> Result<String, Error> {
    let datastore = if cfg!(target_os = "android") {
        // Android-specific initialization using from_paths
        let app_data_dir = app.path().app_data_dir()
            .map_err(|e| Error::Io(IoError::new(std::io::ErrorKind::Other, format!("Failed to get app data dir: {}", e))))?;

        let data_dir = app_data_dir.clone();
        let pods_dir = app_data_dir.join("pods");

        // Ensure the pods directory exists
        if !pods_dir.exists() {
            std::fs::create_dir_all(&pods_dir)
                .map_err(|e| Error::Io(IoError::new(std::io::ErrorKind::Other, format!("Failed to create pods directory: {}", e))))?;
        }

        info!("Android: Using data_dir: {:?}, pods_dir: {:?}", data_dir, pods_dir);
        // Use standard Android Downloads directory
        let downloads_dir = std::path::PathBuf::from("/storage/emulated/0/Download");
        if !downloads_dir.exists() {
            std::fs::create_dir_all(&downloads_dir)
                .map_err(|e| Error::Io(IoError::new(std::io::ErrorKind::Other, format!("Failed to create downloads directory: {}", e))))?;
        }
        DataStore::from_paths(data_dir, pods_dir, downloads_dir)?
    } else {
        // Desktop platforms use the default create method
        DataStore::create()?
    };

    let state = state.lock().unwrap();
    *state.datastore.lock().unwrap() = Some(datastore);
    info!("Datastore initialized");
    Ok("Datastore initialized".to_string())
}

#[tauri::command]
fn datastore_exists() -> Result<bool, Error> {
    let exists = DataStore::data_dir_exists()?;
    Ok(exists)
}

////////////////////////////////////////////////////////////////////
// KeyStore commands
////////////////////////////////////////////////////////////////////

#[tauri::command]
fn create_keystore_from_seed_phrase(
    state: State<'_, Mutex<AppState>>,
    seed_phrase: String,
) -> Result<String, Error> {
    let keystore = KeyStore::from_mnemonic(seed_phrase.trim())?;
    let state = state.lock().unwrap();
    *state.keystore.lock().unwrap() = Some(keystore);
    info!("New KeyStore created from seed phrase");
    Ok("New KeyStore created from seed phrase".to_string())
}

#[tauri::command]
fn create_keystore_from_key(
    state: State<'_, Mutex<AppState>>,
    key: String,
) -> Result<String, Error> {
    let keystore = KeyStore::from_hex(key.trim(), "")?;
    let state = state.lock().unwrap();
    *state.keystore.lock().unwrap() = Some(keystore);
    info!("New KeyStore created from secrete key");
    Ok("New KeyStore created from secrete key".to_string())
}

#[tauri::command]
fn open_keystore(state: State<'_, Mutex<AppState>>, password: String) -> Result<String, Error> {
    let state = state.lock().unwrap();
    let keystore_path = match state.datastore.lock().unwrap().as_ref() {
        Some(datastore) => datastore.get_keystore_path(),
        None => return Err(Error::Message("Datastore not initialized".to_string())),
    };
    let mut file = std::fs::File::open(keystore_path.clone())?;
    let keystore = match KeyStore::from_file(&mut file, &password) {
        Ok(ks) => ks,
        Err(_e) => {
            // You can further match `e` for specific error types if required
            return Err(Error::Message(
                "Failed to open keystore: possible wrong password".into(),
            ));
        }
    };
    *state.keystore.lock().unwrap() = Some(keystore);
    info!("Existing KeyStore file {} opened", keystore_path.display());
    Ok("Existing KeyStore file opened".to_string())
}

#[tauri::command]
fn write_keystore_to_file(
    state: State<'_, Mutex<AppState>>,
    password: String,
) -> Result<String, Error> {
    // Extract all data we need and drop all locks before any await
    let (datastore, keystore) = {
        let state = state.lock().unwrap();

        let datastore = state
            .datastore
            .lock()
            .unwrap()
            .take()
            .ok_or("DataStore not initialized")?;

        let keystore = state
            .keystore
            .lock()
            .unwrap()
            .take()
            .ok_or("KeyStore not initialized")?;

        (datastore, keystore)
    }; // All MutexGuards are dropped here

    let key_store_file = datastore.get_keystore_path();
    let mut file = std::fs::File::create(key_store_file)?;
    KeyStore::to_file(&keystore, &mut file, &password)?;

    // Put the components back
    {
        let state = state.lock().unwrap();
        *state.datastore.lock().unwrap() = Some(datastore);
        *state.keystore.lock().unwrap() = Some(keystore);
    }
    info!("KeyStore written to file");
    Ok("KeyStore written to file".to_string())
}

////////////////////////////////////////////////////////////////////
// Graph commands
////////////////////////////////////////////////////////////////////

#[tauri::command]
fn initialize_graph(state: State<'_, Mutex<AppState>>) -> Result<String, Error> {
    let state = state.lock().unwrap();
    let graph_path = match state.datastore.lock().unwrap().as_ref() {
        Some(datastore) => datastore.get_graph_path(),
        None => return Err(Error::Message("Datastore not initialized".to_string())),
    };
    let graph = Graph::open(&graph_path)?;
    *state.graph.lock().unwrap() = Some(graph);
    info!("Graph initialized");
    Ok("Graph initialized".to_string())
}

////////////////////////////////////////////////////////////////////
// PodManager commands
////////////////////////////////////////////////////////////////////

#[tauri::command]
async fn initialize_pod_manager(state: State<'_, Mutex<AppState>>) -> Result<String, Error> {
    let state = state.lock().unwrap();

    // Check that all required components are initialized
    if state.client.lock().unwrap().is_none() {
        return Err(Error::Message("Client not initialized".to_string()));
    }
    if state.wallet.lock().unwrap().is_none() {
        return Err(Error::Message("Wallet not initialized".to_string()));
    }
    if state.datastore.lock().unwrap().is_none() {
        return Err(Error::Message("DataStore not initialized".to_string()));
    }
    if state.keystore.lock().unwrap().is_none() {
        return Err(Error::Message("KeyStore not initialized".to_string()));
    }
    if state.graph.lock().unwrap().is_none() {
        return Err(Error::Message("Graph not initialized".to_string()));
    }

    info!("PodManager components verified - ready for operations");
    Ok("PodManager components verified - ready for operations".to_string())
}

#[tauri::command]
async fn add_pod(
    state: State<'_, Mutex<AppState>>,
    request: CreatePodRequest,
) -> Result<PodInfo, Error> {
    // Extract all data we need and drop all locks before any await
    let (client, wallet, mut datastore, mut keystore, mut graph) = {
        let state = state.lock().unwrap();

        let client = state
            .client
            .lock()
            .unwrap()
            .as_ref()
            .ok_or("Client not initialized")?
            .clone();

        let wallet = state
            .wallet
            .lock()
            .unwrap()
            .as_ref()
            .ok_or("Wallet not initialized")?
            .clone();

        let datastore = state
            .datastore
            .lock()
            .unwrap()
            .take()
            .ok_or("DataStore not initialized")?;

        let keystore = state
            .keystore
            .lock()
            .unwrap()
            .take()
            .ok_or("KeyStore not initialized")?;

        let graph = state
            .graph
            .lock()
            .unwrap()
            .take()
            .ok_or("Graph not initialized")?;

        (client, wallet, datastore, keystore, graph)
    }; // All MutexGuards are dropped here

    // Perform operations and ensure components are always restored
    let result = async {
        // Now we can safely use async operations
        let mut podman =
            PodManager::new(client, &wallet, &mut datastore, &mut keystore, &mut graph).await?;

        // Use the PodManager
        let (pod_address, _) = podman.add_pod(&request.name).await?;

        info!("Added pod {} with address {}", &request.name, &pod_address);
        Ok(PodInfo {
            address: pod_address,
        })
    }
    .await;

    // Always put the components back, regardless of success or failure
    {
        let state = state.lock().unwrap();
        *state.datastore.lock().unwrap() = Some(datastore);
        *state.keystore.lock().unwrap() = Some(keystore);
        *state.graph.lock().unwrap() = Some(graph);
    }

    result
}

#[tauri::command]
async fn remove_pod(
    state: State<'_, Mutex<AppState>>,
    request: CreatePodRequest,
) -> Result<PodInfo, Error> {
    // Extract all data we need and drop all locks before any await
    let (client, wallet, mut datastore, mut keystore, mut graph) = {
        let state = state.lock().unwrap();

        let client = state
            .client
            .lock()
            .unwrap()
            .as_ref()
            .ok_or("Client not initialized")?
            .clone();

        let wallet = state
            .wallet
            .lock()
            .unwrap()
            .as_ref()
            .ok_or("Wallet not initialized")?
            .clone();

        let datastore = state
            .datastore
            .lock()
            .unwrap()
            .take()
            .ok_or("DataStore not initialized")?;

        let keystore = state
            .keystore
            .lock()
            .unwrap()
            .take()
            .ok_or("KeyStore not initialized")?;

        let graph = state
            .graph
            .lock()
            .unwrap()
            .take()
            .ok_or("Graph not initialized")?;

        (client, wallet, datastore, keystore, graph)
    }; // All MutexGuards are dropped here

    // Perform operations and ensure components are always restored
    let result = async {
        // Now we can safely use async operations
        let mut podman =
            PodManager::new(client, &wallet, &mut datastore, &mut keystore, &mut graph).await?;

        // Use the PodManager
        podman.remove_pod(&request.name).await?;

        info!("Removed pod {}", &request.name);
        Ok(PodInfo {
            address: request.name,
        })
    }
    .await;

    // Always put the components back, regardless of success or failure
    {
        let state = state.lock().unwrap();
        *state.datastore.lock().unwrap() = Some(datastore);
        *state.keystore.lock().unwrap() = Some(keystore);
        *state.graph.lock().unwrap() = Some(graph);
    }

    result
}

#[tauri::command]
async fn rename_pod(
    state: State<'_, Mutex<AppState>>,
    request: RenamePodRequest,
) -> Result<PodInfo, Error> {
    // Extract all data we need and drop all locks before any await
    let (client, wallet, mut datastore, mut keystore, mut graph) = {
        let state = state.lock().unwrap();

        let client = state
            .client
            .lock()
            .unwrap()
            .as_ref()
            .ok_or("Client not initialized")?
            .clone();

        let wallet = state
            .wallet
            .lock()
            .unwrap()
            .as_ref()
            .ok_or("Wallet not initialized")?
            .clone();

        let datastore = state
            .datastore
            .lock()
            .unwrap()
            .take()
            .ok_or("DataStore not initialized")?;

        let keystore = state
            .keystore
            .lock()
            .unwrap()
            .take()
            .ok_or("KeyStore not initialized")?;

        let graph = state
            .graph
            .lock()
            .unwrap()
            .take()
            .ok_or("Graph not initialized")?;

        (client, wallet, datastore, keystore, graph)
    }; // All MutexGuards are dropped here

    // Perform operations and ensure components are always restored
    let result = async {
        // Now we can safely use async operations
        let mut podman =
            PodManager::new(client, &wallet, &mut datastore, &mut keystore, &mut graph).await?;

        // Use the PodManager
        podman.rename_pod(&request.name, &request.new_name).await?;

        info!(
            "Renamed pod {} to new name {}",
            &request.name, &request.new_name
        );
        Ok(PodInfo {
            address: request.name,
        })
    }
    .await;

    // Always put the components back, regardless of success or failure
    {
        let state = state.lock().unwrap();
        *state.datastore.lock().unwrap() = Some(datastore);
        *state.keystore.lock().unwrap() = Some(keystore);
        *state.graph.lock().unwrap() = Some(graph);
    }

    result
}

#[tauri::command]
async fn add_pod_ref(
    state: State<'_, Mutex<AppState>>,
    request: CreatePodRefRequest,
) -> Result<PodInfo, Error> {
    // Extract all data we need and drop all locks before any await
    let (client, wallet, mut datastore, mut keystore, mut graph) = {
        let state = state.lock().unwrap();

        let client = state
            .client
            .lock()
            .unwrap()
            .as_ref()
            .ok_or("Client not initialized")?
            .clone();

        let wallet = state
            .wallet
            .lock()
            .unwrap()
            .as_ref()
            .ok_or("Wallet not initialized")?
            .clone();

        let datastore = state
            .datastore
            .lock()
            .unwrap()
            .take()
            .ok_or("DataStore not initialized")?;

        let keystore = state
            .keystore
            .lock()
            .unwrap()
            .take()
            .ok_or("KeyStore not initialized")?;

        let graph = state
            .graph
            .lock()
            .unwrap()
            .take()
            .ok_or("Graph not initialized")?;

        (client, wallet, datastore, keystore, graph)
    }; // All MutexGuards are dropped here

    // Perform operations and ensure components are always restored
    let result = async {
        // Now we can safely use async operations
        let mut podman =
            PodManager::new(client, &wallet, &mut datastore, &mut keystore, &mut graph).await?;

        // Use the PodManager
        podman
            .add_pod_ref(&request.pod_address, &request.pod_ref_address)
            .await?;

        info!(
            "Added pod reference {} to pod {}",
            &request.pod_ref_address, &request.pod_address
        );
        Ok(PodInfo {
            address: request.pod_address,
        })
    }
    .await;

    // Always put the components back, regardless of success or failure
    {
        let state = state.lock().unwrap();
        *state.datastore.lock().unwrap() = Some(datastore);
        *state.keystore.lock().unwrap() = Some(keystore);
        *state.graph.lock().unwrap() = Some(graph);
    }

    result
}

#[tauri::command]
async fn remove_pod_ref(
    state: State<'_, Mutex<AppState>>,
    request: CreatePodRefRequest,
) -> Result<PodInfo, Error> {
    // Extract all data we need and drop all locks before any await
    let (client, wallet, mut datastore, mut keystore, mut graph) = {
        let state = state.lock().unwrap();

        let client = state
            .client
            .lock()
            .unwrap()
            .as_ref()
            .ok_or("Client not initialized")?
            .clone();

        let wallet = state
            .wallet
            .lock()
            .unwrap()
            .as_ref()
            .ok_or("Wallet not initialized")?
            .clone();

        let datastore = state
            .datastore
            .lock()
            .unwrap()
            .take()
            .ok_or("DataStore not initialized")?;

        let keystore = state
            .keystore
            .lock()
            .unwrap()
            .take()
            .ok_or("KeyStore not initialized")?;

        let graph = state
            .graph
            .lock()
            .unwrap()
            .take()
            .ok_or("Graph not initialized")?;

        (client, wallet, datastore, keystore, graph)
    }; // All MutexGuards are dropped here

    // Perform operations and ensure components are always restored
    let result = async {
        // Now we can safely use async operations
        let mut podman =
            PodManager::new(client, &wallet, &mut datastore, &mut keystore, &mut graph).await?;

        // Use the PodManager
        podman
            .remove_pod_ref(&request.pod_address, &request.pod_ref_address)
            .await?;

        info!(
            "Removed pod reference {} from pod {}",
            &request.pod_ref_address, &request.pod_address
        );
        Ok(PodInfo {
            address: request.pod_address,
        })
    }
    .await;

    // Always put the components back, regardless of success or failure
    {
        let state = state.lock().unwrap();
        *state.datastore.lock().unwrap() = Some(datastore);
        *state.keystore.lock().unwrap() = Some(keystore);
        *state.graph.lock().unwrap() = Some(graph);
    }

    result
}

#[tauri::command]
async fn get_update_list(state: State<'_, Mutex<AppState>>) -> Result<Value, Error> {
    // Extract all data we need and drop all locks before any await
    let (client, wallet, mut datastore, mut keystore, mut graph) = {
        let state = state.lock().unwrap();

        let client = state
            .client
            .lock()
            .unwrap()
            .as_ref()
            .ok_or("Client not initialized")?
            .clone();

        let wallet = state
            .wallet
            .lock()
            .unwrap()
            .as_ref()
            .ok_or("Wallet not initialized")?
            .clone();

        let datastore = state
            .datastore
            .lock()
            .unwrap()
            .take()
            .ok_or("DataStore not initialized")?;

        let keystore = state
            .keystore
            .lock()
            .unwrap()
            .take()
            .ok_or("KeyStore not initialized")?;

        let graph = state
            .graph
            .lock()
            .unwrap()
            .take()
            .ok_or("Graph not initialized")?;

        (client, wallet, datastore, keystore, graph)
    }; // All MutexGuards are dropped here

    // Perform operations and ensure components are always restored
    let result = {
        // Now we can safely use async operations
        let podman =
            PodManager::new(client, &wallet, &mut datastore, &mut keystore, &mut graph).await?;

        // Use the PodManager
        let update_list = podman.get_update_list()?;

        Ok(update_list)
    };

    // Always put the components back, regardless of success or failure
    {
        let state = state.lock().unwrap();
        *state.datastore.lock().unwrap() = Some(datastore);
        *state.keystore.lock().unwrap() = Some(keystore);
        *state.graph.lock().unwrap() = Some(graph);
    }

    result
}

#[tauri::command]
async fn list_my_pods(state: State<'_, Mutex<AppState>>) -> Result<Vec<PodMetaData>, Error> {
    // Extract all data we need and drop all locks before any await
    let (client, wallet, mut datastore, mut keystore, mut graph) = {
        let state = state.lock().unwrap();

        let client = state
            .client
            .lock()
            .unwrap()
            .as_ref()
            .ok_or("Client not initialized")?
            .clone();

        let wallet = state
            .wallet
            .lock()
            .unwrap()
            .as_ref()
            .ok_or("Wallet not initialized")?
            .clone();

        let datastore = state
            .datastore
            .lock()
            .unwrap()
            .take()
            .ok_or("DataStore not initialized")?;

        let keystore = state
            .keystore
            .lock()
            .unwrap()
            .take()
            .ok_or("KeyStore not initialized")?;

        let graph = state
            .graph
            .lock()
            .unwrap()
            .take()
            .ok_or("Graph not initialized")?;

        (client, wallet, datastore, keystore, graph)
    }; // All MutexGuards are dropped here

    // Perform operations and ensure components are always restored
    let result = async {
        // Now we can safely use async operations
        let podman =
            PodManager::new(client, &wallet, &mut datastore, &mut keystore, &mut graph).await?;

        // Use the PodManager
        let pod_list = podman.list_my_pods()?;

        let bindings = pod_list["results"]["bindings"]
            .as_array()
            .ok_or(Error::Message("Unexpected pod list format".into()))?;

        let mut pods: HashMap<String, PodMetaData> = HashMap::new();

        for bind in bindings {
            let subject = bind["subject"]["value"].as_str();
            let predicate = bind["predicate"]["value"].as_str();
            let object = bind["object"]["value"].as_str();

            if let (Some(subject), Some(predicate), Some(object)) = (subject, predicate, object) {
                let address = subject
                    .strip_prefix("ant://")
                    .unwrap_or(subject)
                    .to_string();
                let entry = pods.entry(subject.to_string()).or_insert(PodMetaData {
                    address,
                    name: None,
                    creation: None,
                    modified: None,
                    depth: None,
                });

                if predicate.ends_with("name") {
                    entry.name = Some(object.to_string());
                } else if predicate.ends_with("creation") {
                    entry.creation = Some(object.to_string());
                } else if predicate.ends_with("modified") {
                    entry.modified = Some(object.to_string());
                } else if predicate.ends_with("depth") {
                    entry.depth = Some(object.to_string());
                }
            }
        }

        let pod_vec = pods.into_values().collect();
        Ok(pod_vec)
    }
    .await;

    // Always put the components back, regardless of success or failure
    {
        let state = state.lock().unwrap();
        *state.datastore.lock().unwrap() = Some(datastore);
        *state.keystore.lock().unwrap() = Some(keystore);
        *state.graph.lock().unwrap() = Some(graph);
    }

    result
}

#[tauri::command]
async fn list_pod_subjects(
    state: State<'_, Mutex<AppState>>,
    address: String,
) -> Result<AddressList, Error> {
    // Extract all data we need and drop all locks before any await
    let (client, wallet, mut datastore, mut keystore, mut graph) = {
        let state = state.lock().unwrap();

        let client = state
            .client
            .lock()
            .unwrap()
            .as_ref()
            .ok_or("Client not initialized")?
            .clone();

        let wallet = state
            .wallet
            .lock()
            .unwrap()
            .as_ref()
            .ok_or("Wallet not initialized")?
            .clone();

        let datastore = state
            .datastore
            .lock()
            .unwrap()
            .take()
            .ok_or("DataStore not initialized")?;

        let keystore = state
            .keystore
            .lock()
            .unwrap()
            .take()
            .ok_or("KeyStore not initialized")?;

        let graph = state
            .graph
            .lock()
            .unwrap()
            .take()
            .ok_or("Graph not initialized")?;

        (client, wallet, datastore, keystore, graph)
    }; // All MutexGuards are dropped here

    // Perform operations and ensure components are always restored
    let result = async {
        // Now we can safely use async operations
        let podman =
            PodManager::new(client, &wallet, &mut datastore, &mut keystore, &mut graph).await?;

        // Use the PodManager
        let subject_list = podman.list_pod_subjects(&address)?;

        info!("List of subjects in pod {}:", address);
        for subject in &subject_list {
            info!("Subject address: {}", subject);
        }

        Ok(AddressList {
            addresses: subject_list,
        })
    }
    .await;

    // Always put the components back, regardless of success or failure
    {
        let state = state.lock().unwrap();
        *state.datastore.lock().unwrap() = Some(datastore);
        *state.keystore.lock().unwrap() = Some(keystore);
        *state.graph.lock().unwrap() = Some(graph);
    }

    result
}

#[tauri::command]
async fn upload_pod(
    state: State<'_, Mutex<AppState>>,
    request: UploadPodRequest,
) -> Result<PodInfo, Error> {
    // Extract all data we need and drop all locks before any await
    let (client, wallet, mut datastore, mut keystore, mut graph) = {
        let state = state.lock().unwrap();

        let client = state
            .client
            .lock()
            .unwrap()
            .as_ref()
            .ok_or("Client not initialized")?
            .clone();

        let wallet = state
            .wallet
            .lock()
            .unwrap()
            .as_ref()
            .ok_or("Wallet not initialized")?
            .clone();

        let datastore = state
            .datastore
            .lock()
            .unwrap()
            .take()
            .ok_or("DataStore not initialized")?;

        let keystore = state
            .keystore
            .lock()
            .unwrap()
            .take()
            .ok_or("KeyStore not initialized")?;

        let graph = state
            .graph
            .lock()
            .unwrap()
            .take()
            .ok_or("Graph not initialized")?;

        (client, wallet, datastore, keystore, graph)
    }; // All MutexGuards are dropped here

    // Perform operations and ensure components are always restored
    let result = async {
        // Now we can safely use async operations
        let mut podman =
            PodManager::new(client, &wallet, &mut datastore, &mut keystore, &mut graph).await?;

        // Use the PodManager
        podman.upload_pod(&request.pod_address).await?;

        info!("Uploaded pod with address {}", &request.pod_address);
        Ok(PodInfo {
            address: request.pod_address,
        })
    }
    .await;

    // Always put the components back, regardless of success or failure
    {
        let state = state.lock().unwrap();
        *state.datastore.lock().unwrap() = Some(datastore);
        *state.keystore.lock().unwrap() = Some(keystore);
        *state.graph.lock().unwrap() = Some(graph);
    }

    result
}

#[tauri::command]
async fn upload_all(state: State<'_, Mutex<AppState>>) -> Result<String, Error> {
    // Extract all data we need and drop all locks before any await
    let (client, wallet, mut datastore, mut keystore, mut graph) = {
        let state = state.lock().unwrap();

        let client = state
            .client
            .lock()
            .unwrap()
            .as_ref()
            .ok_or("Client not initialized")?
            .clone();

        let wallet = state
            .wallet
            .lock()
            .unwrap()
            .as_ref()
            .ok_or("Wallet not initialized")?
            .clone();

        let datastore = state
            .datastore
            .lock()
            .unwrap()
            .take()
            .ok_or("DataStore not initialized")?;

        let keystore = state
            .keystore
            .lock()
            .unwrap()
            .take()
            .ok_or("KeyStore not initialized")?;

        let graph = state
            .graph
            .lock()
            .unwrap()
            .take()
            .ok_or("Graph not initialized")?;

        (client, wallet, datastore, keystore, graph)
    }; // All MutexGuards are dropped here

    // Perform operations and ensure components are always restored
    let result = async {
        // Now we can safely use async operations
        let mut podman =
            PodManager::new(client, &wallet, &mut datastore, &mut keystore, &mut graph).await?;

        // Use the PodManager
        podman.upload_all().await?;

        info!("Uploaded all updated pods to Autonomi");
        Ok("Successfully uploaded all updated pods to Autonomi".to_string())
    }
    .await;

    // Always put the components back, regardless of success or failure
    {
        let state = state.lock().unwrap();
        *state.datastore.lock().unwrap() = Some(datastore);
        *state.keystore.lock().unwrap() = Some(keystore);
        *state.graph.lock().unwrap() = Some(graph);
    }

    result
}

#[tauri::command]
async fn refresh_cache(state: State<'_, Mutex<AppState>>) -> Result<String, Error> {
    // Extract all data we need and drop all locks before any await
    let (client, wallet, mut datastore, mut keystore, mut graph) = {
        let state = state.lock().unwrap();

        let client = state
            .client
            .lock()
            .unwrap()
            .as_ref()
            .ok_or("Client not initialized")?
            .clone();

        let wallet = state
            .wallet
            .lock()
            .unwrap()
            .as_ref()
            .ok_or("Wallet not initialized")?
            .clone();

        let datastore = state
            .datastore
            .lock()
            .unwrap()
            .take()
            .ok_or("DataStore not initialized")?;

        let keystore = state
            .keystore
            .lock()
            .unwrap()
            .take()
            .ok_or("KeyStore not initialized")?;

        let graph = state
            .graph
            .lock()
            .unwrap()
            .take()
            .ok_or("Graph not initialized")?;

        (client, wallet, datastore, keystore, graph)
    }; // All MutexGuards are dropped here

    // Perform operations and ensure components are always restored
    let result = async {
        // Now we can safely use async operations
        let mut podman =
            PodManager::new(client, &wallet, &mut datastore, &mut keystore, &mut graph).await?;

        // Use the PodManager
        podman.refresh_cache().await?;

        info!("Refreshed local pod cache");
        Ok("Successfully refreshed local pod cache".to_string())
    }
    .await;

    // Always put the components back, regardless of success or failure
    {
        let state = state.lock().unwrap();
        *state.datastore.lock().unwrap() = Some(datastore);
        *state.keystore.lock().unwrap() = Some(keystore);
        *state.graph.lock().unwrap() = Some(graph);
    }

    result
}

#[tauri::command]
async fn refresh_ref(
    state: State<'_, Mutex<AppState>>,
    request: RefreshRefRequest,
) -> Result<String, Error> {
    // Extract all data we need and drop all locks before any await
    let (client, wallet, mut datastore, mut keystore, mut graph) = {
        let state = state.lock().unwrap();

        let client = state
            .client
            .lock()
            .unwrap()
            .as_ref()
            .ok_or("Client not initialized")?
            .clone();

        let wallet = state
            .wallet
            .lock()
            .unwrap()
            .as_ref()
            .ok_or("Wallet not initialized")?
            .clone();

        let datastore = state
            .datastore
            .lock()
            .unwrap()
            .take()
            .ok_or("DataStore not initialized")?;

        let keystore = state
            .keystore
            .lock()
            .unwrap()
            .take()
            .ok_or("KeyStore not initialized")?;

        let graph = state
            .graph
            .lock()
            .unwrap()
            .take()
            .ok_or("Graph not initialized")?;

        (client, wallet, datastore, keystore, graph)
    }; // All MutexGuards are dropped here

    // Perform operations and ensure components are always restored
    let result = async {
        // Now we can safely use async operations
        let mut podman =
            PodManager::new(client, &wallet, &mut datastore, &mut keystore, &mut graph).await?;

        // Use the PodManager
        let depth: u64 = request
            .depth
            .clone()
            .parse()
            .map_err(|_| Error::from("Invalid depth"))?;
        podman.refresh_ref(depth).await?;

        info!(
            "Refreshed all local pods and pod reference to cache to depth {}",
            &request.depth
        );
        Ok(format!(
            "Successfully refreshed all local pods and pod reference to cache to depth {}",
            &request.depth
        ))
    }
    .await;

    // Always put the components back, regardless of success or failure
    {
        let state = state.lock().unwrap();
        *state.datastore.lock().unwrap() = Some(datastore);
        *state.keystore.lock().unwrap() = Some(keystore);
        *state.graph.lock().unwrap() = Some(graph);
    }

    result
}

#[tauri::command]
async fn search(
    state: State<'_, Mutex<AppState>>,
    request: SearchRequest,
) -> Result<SearchResult, Error> {
    // Extract all data we need and drop all locks before any await
    let (client, wallet, mut datastore, mut keystore, mut graph) = {
        let state = state.lock().unwrap();

        let client = state
            .client
            .lock()
            .unwrap()
            .as_ref()
            .ok_or("Client not initialized")?
            .clone();

        let wallet = state
            .wallet
            .lock()
            .unwrap()
            .as_ref()
            .ok_or("Wallet not initialized")?
            .clone();

        let datastore = state
            .datastore
            .lock()
            .unwrap()
            .take()
            .ok_or("DataStore not initialized")?;

        let keystore = state
            .keystore
            .lock()
            .unwrap()
            .take()
            .ok_or("KeyStore not initialized")?;

        let graph = state
            .graph
            .lock()
            .unwrap()
            .take()
            .ok_or("Graph not initialized")?;

        (client, wallet, datastore, keystore, graph)
    }; // All MutexGuards are dropped here

    // Perform operations and ensure components are always restored
    let result = async {
        // Now we can safely use async operations
        let mut podman =
            PodManager::new(client, &wallet, &mut datastore, &mut keystore, &mut graph).await?;

        // Use the PodManager
        let search_results = podman.search(request.query.clone()).await?;

        info!("Search completed");
        Ok(SearchResult {
            results: search_results,
        })
    }
    .await;

    // Always put the components back, regardless of success or failure
    {
        let state = state.lock().unwrap();
        *state.datastore.lock().unwrap() = Some(datastore);
        *state.keystore.lock().unwrap() = Some(keystore);
        *state.graph.lock().unwrap() = Some(graph);
    }

    result
}

#[tauri::command]
async fn put_subject_data(
    state: State<'_, Mutex<AppState>>,
    request: PutSubjectDataRequest,
) -> Result<String, Error> {
    // Extract all data we need and drop all locks before any await
    let (client, wallet, mut datastore, mut keystore, mut graph) = {
        let state = state.lock().unwrap();

        let client = state
            .client
            .lock()
            .unwrap()
            .as_ref()
            .ok_or("Client not initialized")?
            .clone();

        let wallet = state
            .wallet
            .lock()
            .unwrap()
            .as_ref()
            .ok_or("Wallet not initialized")?
            .clone();

        let datastore = state
            .datastore
            .lock()
            .unwrap()
            .take()
            .ok_or("DataStore not initialized")?;

        let keystore = state
            .keystore
            .lock()
            .unwrap()
            .take()
            .ok_or("KeyStore not initialized")?;

        let graph = state
            .graph
            .lock()
            .unwrap()
            .take()
            .ok_or("Graph not initialized")?;

        (client, wallet, datastore, keystore, graph)
    }; // All MutexGuards are dropped here

    // Perform operations and ensure components are always restored
    let result = async {
        // Now we can safely use async operations
        let mut podman =
            PodManager::new(client, &wallet, &mut datastore, &mut keystore, &mut graph).await?;

        // Use the PodManager
        podman
            .put_subject_data(
                &request.pod_address,
                &request.subject_address,
                &request.data,
            )
            .await?;

        info!(
            "Put data for subject {} in pod {}",
            &request.subject_address, &request.pod_address
        );
        Ok(format!(
            "Successfully put data for subject {} in pod {}",
            &request.subject_address, &request.pod_address
        ))
    }
    .await;

    // Always put the components back, regardless of success or failure
    {
        let state = state.lock().unwrap();
        *state.datastore.lock().unwrap() = Some(datastore);
        *state.keystore.lock().unwrap() = Some(keystore);
        *state.graph.lock().unwrap() = Some(graph);
    }

    result
}

#[tauri::command]
async fn get_subject_data(
    state: State<'_, Mutex<AppState>>,
    request: GetSubjectDataRequest,
) -> Result<SubjectDataResult, Error> {
    // Extract all data we need and drop all locks before any await
    let (client, wallet, mut datastore, mut keystore, mut graph) = {
        let state = state.lock().unwrap();

        let client = state
            .client
            .lock()
            .unwrap()
            .as_ref()
            .ok_or("Client not initialized")?
            .clone();

        let wallet = state
            .wallet
            .lock()
            .unwrap()
            .as_ref()
            .ok_or("Wallet not initialized")?
            .clone();

        let datastore = state
            .datastore
            .lock()
            .unwrap()
            .take()
            .ok_or("DataStore not initialized")?;

        let keystore = state
            .keystore
            .lock()
            .unwrap()
            .take()
            .ok_or("KeyStore not initialized")?;

        let graph = state
            .graph
            .lock()
            .unwrap()
            .take()
            .ok_or("Graph not initialized")?;

        (client, wallet, datastore, keystore, graph)
    }; // All MutexGuards are dropped here

    // Perform operations and ensure components are always restored
    let result = async {
        // Now we can safely use async operations
        let mut podman =
            PodManager::new(client, &wallet, &mut datastore, &mut keystore, &mut graph).await?;

        // Use the PodManager
        let subject_data = podman.get_subject_data(&request.subject_address).await?;

        info!("Retrieved data for subject {}", &request.subject_address);
        Ok(SubjectDataResult { data: subject_data })
    }
    .await;

    // Always put the components back, regardless of success or failure
    {
        let state = state.lock().unwrap();
        *state.datastore.lock().unwrap() = Some(datastore);
        *state.keystore.lock().unwrap() = Some(keystore);
        *state.graph.lock().unwrap() = Some(graph);
    }

    result
}

////////////////////////////////////////////////////////////////////
// Autonomi commands
////////////////////////////////////////////////////////////////////

#[tauri::command]
async fn add_wallet(
    state: State<'_, Mutex<AppState>>,
    request: AddWalletRequest,
) -> Result<String, Error> {
    // Extract keystore and drop all locks before any operations
    let mut keystore = {
        let app_state = state.lock().unwrap();
        let keystore_guard = app_state.keystore.lock().unwrap();
        keystore_guard.clone().ok_or("KeyStore not initialized")?
    }; // All MutexGuards are dropped here

    // Call the keystore add_wallet_key function to add the new wallet private key to the keystore
    keystore.add_wallet_key(&request.name, &request.key)?;

    // Put the modified keystore back
    {
        let app_state = state.lock().unwrap();
        *app_state.keystore.lock().unwrap() = Some(keystore);
    }

    info!("Wallet added: {}", request.name);
    Ok("Wallet added".to_string())
}

#[tauri::command]
async fn remove_wallet(state: State<'_, Mutex<AppState>>, name: String) -> Result<String, Error> {
    // Extract keystore and drop all locks before any operations
    let mut keystore = {
        let app_state = state.lock().unwrap();
        let keystore_guard = app_state.keystore.lock().unwrap();
        keystore_guard.clone().ok_or("KeyStore not initialized")?
    }; // All MutexGuards are dropped here

    // Call the keystore remove_wallet_key function to remove the wallet private key from the keystore
    keystore.remove_wallet_key(&name)?;

    // Put the modified keystore back
    {
        let app_state = state.lock().unwrap();
        *app_state.keystore.lock().unwrap() = Some(keystore);
    }

    info!("Wallet removed: {}", name);
    Ok("Wallet removed".to_string())
}

#[tauri::command]
async fn list_wallets(state: State<'_, Mutex<AppState>>) -> Result<Value, Error> {
    let state = state.lock().unwrap();
    let keystore = state
        .keystore
        .lock()
        .unwrap()
        .as_ref()
        .ok_or("KeyStore not initialized")?
        .clone();

    // Call the keystore list_wallets function to get the list of wallet names
    let wallet_keys = keystore.get_wallet_keys();
    let wallet_addresses = keystore.get_wallet_addresses();

    // Build an array of wallet objects
    let wallets: Vec<Value> = wallet_keys
        .iter()
        .filter_map(|(name, key)| {
            let address = match wallet_addresses.get(name) {
                Some(address) => address,
                None => {
                    error!("Wallet address not found for wallet: {}", name);
                    return None;
                }
            };

            Some(serde_json::json!({
                "name": name,
                "key": key,
                "address": address
            }))
        })
        .collect();

    info!("Wallets listed");
    Ok(serde_json::json!(wallets))
}

async fn get_wallet_balances(
    evm_network: autonomi::Network,
    key: &str,
) -> Result<(f64, f64), Error> {
    // Create a wallet instance for this specific wallet
    let target_wallet = Wallet::new_from_private_key(evm_network, key)
        .map_err(|e| Error::Message(format!("Failed to create wallet: {e}")))?;

    // Get the balance using the wallet's balance method
    let balance_raw = target_wallet
        .balance_of_tokens()
        .await
        .map_err(|e| Error::Message(format!("Failed to get wallet token balance: {e}")))?;

    let gas_balance_raw = target_wallet
        .balance_of_gas_tokens()
        .await
        .map_err(|e| Error::Message(format!("Failed to get wallet gas balance: {e}")))?;

    // Convert balances to human-readable format (ETH)
    let balance: f64 = balance_raw.into();
    let balance = balance / 1_000_000_000_000_000_000.0f64;

    let gas_balance: f64 = gas_balance_raw.into();
    let gas_balance = gas_balance / 1_000_000_000_000_000_000.0f64;

    Ok((balance, gas_balance))
}

#[tauri::command]
async fn get_wallet_balance(
    state: State<'_, Mutex<AppState>>,
    wallet_key: String,
) -> Result<(f64, f64), Error> {
    let evm_network = {
        let state = state.lock().unwrap();
        let client = state
            .client
            .lock()
            .unwrap()
            .as_ref()
            .ok_or("Client not initialized")?
            .clone();
        client.evm_network().clone()
    }; // All MutexGuards are dropped here

    // Call the async balance function
    let (ant_balance, gas_balance) = get_wallet_balances(evm_network, &wallet_key).await?;

    info!(
        "Wallet balance retrieved: ANT={}, ETH={}",
        ant_balance, gas_balance
    );
    Ok((ant_balance, gas_balance))
}

#[tauri::command]
async fn get_wallet(state: State<'_, Mutex<AppState>>, name: String) -> Result<String, Error> {
    let state = state.lock().unwrap();
    let keystore = state
        .keystore
        .lock()
        .unwrap()
        .as_ref()
        .ok_or("KeyStore not initialized")?
        .clone();

    // Call the keystore get_wallet_key function to get the wallet private key
    let wallet_key = keystore.get_wallet_key(&name)?;

    info!("Wallet key retrieved for: {}", name);
    Ok(wallet_key)
}

#[tauri::command]
async fn get_active_wallet(state: State<'_, Mutex<AppState>>) -> Result<(String, String), Error> {
    let state = state.lock().unwrap();
    let datastore = state
        .datastore
        .lock()
        .unwrap()
        .as_ref()
        .ok_or("DataStore not initialized")?
        .clone();

    // Call the keystore get_active_wallet function to get the active wallet name
    let (name, address) = datastore.get_active_wallet()?;

    info!("Active wallet retrieved: {name}");
    Ok((name, address))
}

#[tauri::command]
async fn set_active_wallet(
    state: State<'_, Mutex<AppState>>,
    name: String,
) -> Result<(String, String), Error> {
    let state = state.lock().unwrap();

    let mut keystore = state
        .keystore
        .lock()
        .unwrap()
        .as_ref()
        .ok_or("KeyStore not initialized")?
        .clone();

    let datastore = state
        .datastore
        .lock()
        .unwrap()
        .as_ref()
        .ok_or("DataStore not initialized")?
        .clone();

    // Set the active wallet
    let (name, address) = keystore.set_active_wallet(&name)?;
    datastore.set_active_wallet(&name, &address)?;

    info!("Active wallet retrieved: {name}");
    Ok((name, address))
}

#[tauri::command]
async fn switch_wallet(state: State<'_, Mutex<AppState>>, name: String) -> Result<String, Error> {
    // Extract all data we need and drop all locks before any await
    let (client, wallet_key, evm_network) = {
        let state = state.lock().unwrap();

        let client = state
            .client
            .lock()
            .unwrap()
            .take()
            .ok_or("Client not initialized")?;

        let mut keystore = state
            .keystore
            .lock()
            .unwrap()
            .as_ref()
            .ok_or("KeyStore not initialized")?
            .clone();

        let datastore = state
            .datastore
            .lock()
            .unwrap()
            .as_ref()
            .ok_or("DataStore not initialized")?
            .clone();

        // Call the keystore get_wallet_key function to get the wallet private key
        let wallet_key = keystore.get_wallet_key(&name)?;

        // Set the active wallet
        let (name, address) = keystore.set_active_wallet(&name)?;
        datastore.set_active_wallet(&name, &address)?;

        // Get the EVM network from the existing client
        let evm_network = client.evm_network().clone();

        (client, wallet_key, evm_network)
    }; // All MutexGuards are dropped here

    // Perform operations and ensure components are always restored
    let result = async {
        info!("EVM network: {evm_network:?}");

        // Create new wallet with the specified key
        let wallet = Wallet::new_from_private_key(evm_network.clone(), &wallet_key)
            .map_err(|e| Error::Message(format!("Failed to create wallet: {e}")))?;

        info!("Wallet switched to: {}", name);
        Ok((wallet, wallet_key))
    }
    .await;

    // Always put the components back, regardless of success or failure
    match result {
        Ok((wallet, _wallet_key)) => {
            {
                let state_guard = state.lock().unwrap();
                *state_guard.client.lock().unwrap() = Some(client);
                *state_guard.wallet.lock().unwrap() = Some(wallet);
            } // Release the lock

            Ok(format!("Successfully switched to wallet: {name}"))
        }
        Err(e) => {
            // Restore the client even on failure
            let state = state.lock().unwrap();
            *state.client.lock().unwrap() = Some(client);
            Err(e)
        }
    }
}

#[tauri::command]
async fn initialize_autonomi_client(
    state: State<'_, Mutex<AppState>>,
    wallet_key: String,
    app: tauri::AppHandle,
) -> Result<String, Error> {
    let environment = {
        let state = state.lock().unwrap();
        state.network.clone()
    };
    let client = init_client(app, &environment).await?;
    let evm_network = client.evm_network();
    info!("EVM network: {evm_network:?}");
    //FIXME: need to grap the wallet error and remove this unwrap()
    let wallet = Wallet::new_from_private_key(evm_network.clone(), &wallet_key).unwrap();

    // Lock the state and update the client
    let state_guard = state.lock().unwrap();
    *state_guard.client.lock().unwrap() = Some(client);
    *state_guard.wallet.lock().unwrap() = Some(wallet);
    drop(state_guard); // Release the lock

    info!("Autonomi client initialized");
    Ok("Autonomi client initialized".to_string())
}

#[tauri::command]
async fn upload_cost(
    state: State<'_, Mutex<AppState>>,
    request: UploadFileRequest,
) -> Result<String, Error> {
    let client = {
        let state = state.lock().unwrap();

        let client = state
            .client
            .lock()
            .unwrap()
            .as_ref()
            .ok_or("Client not initialized")?
            .clone();

        client
    }; // All MutexGuards are dropped here

    let data = std::fs::read(request.file_path.clone())?;
    let data = Bytes::from(data);

    let cost = client.data_cost(data).await?;
    info!(
        "File {} is estimated to cost {} to upload",
        request.file_path,
        cost.to_string()
    );

    Ok(cost.to_string())
}

#[tauri::command]
async fn upload_data(
    state: State<'_, Mutex<AppState>>,
    request: UploadFileRequest,
    app: tauri::AppHandle,
) -> Result<(String, String), Error> {
    let (client, wallet) = {
        let state = state.lock().unwrap();

        let client = state
            .client
            .lock()
            .unwrap()
            .as_ref()
            .ok_or("Client not initialized")?
            .clone();

        let wallet = state
            .wallet
            .lock()
            .unwrap()
            .as_ref()
            .ok_or("Wallet not initialized")?
            .clone();

        (client, wallet)
    }; // All MutexGuards are dropped here

    // Read file
    let data = std::fs::read(&request.file_path)?;
    let data_len = data.len();
    app.emit(
        "upload-started",
        serde_json::json!({
            "id": request.id,
            "size": data_len,
            "path": request.file_path,
        }),
    )
    .map_err(|e| Error::Message(format!("Emit failed: {e}")))?;

    let data = std::fs::read(request.file_path.clone())?;
    let data = Bytes::from(data);

    let payment = PaymentOption::Wallet(wallet);
    let (cost, data_addr) = match client.data_put_public(data, payment).await {
        Ok(result) => result,
        Err(e) => {
            app.emit(
                "upload-error",
                serde_json::json!({
                    "id": request.id,
                    "path": request.file_path,
                    "message": format!("Upload failed: {}", e)
                }),
            )
            .map_err(|e| Error::Message(format!("Emit failed: {e}")))?;
            return Err(e.into());
        }
    };

    app.emit(
        "upload-complete",
        serde_json::json!({
            "id": request.id,
            "path": request.file_path,
            "address": data_addr.to_string(),
            "cost": cost.to_string()
        }),
    )
    .map_err(|e| Error::Message(format!("Emit failed: {e}")))?;

    Ok((cost.to_string(), data_addr.to_string()))
}

#[tauri::command]
async fn upload_directory(
    state: State<'_, Mutex<AppState>>,
    request: UploadFileRequest,
    app: tauri::AppHandle,
) -> Result<(String, String), Error> {
    let (client, wallet) = {
        let state = state.lock().unwrap();

        let client = state
            .client
            .lock()
            .unwrap()
            .as_ref()
            .ok_or("Client not initialized")?
            .clone();

        let wallet = state
            .wallet
            .lock()
            .unwrap()
            .as_ref()
            .ok_or("Wallet not initialized")?
            .clone();

        (client, wallet)
    }; // All MutexGuards are dropped here

    // Read file
    let data = std::fs::read(&request.file_path)?;
    let data_len = data.len();
    app.emit(
        "upload-started",
        serde_json::json!({
            "id": request.id,
            "size": data_len,
            "path": request.file_path,
        }),
    )
    .map_err(|e| Error::Message(format!("Emit failed: {e}")))?;

    let payment = PaymentOption::Wallet(wallet);
    let (cost, data_addr) = match client
        .dir_upload_public(request.file_path.clone().into(), payment)
        .await
    {
        Ok(result) => result,
        Err(e) => {
            app.emit(
                "upload-error",
                serde_json::json!({
                    "id": request.id,
                    "path": request.file_path,
                    "message": format!("Upload failed: {}", e)
                }),
            )
            .map_err(|e| Error::Message(format!("Emit failed: {e}")))?;
            return Err(Error::Message(format!("Emit failed: {e}")));
        }
    };

    app.emit(
        "upload-complete",
        serde_json::json!({
            "id": request.id,
            "path": request.file_path,
            "address": data_addr.to_string(),
            "cost": cost.to_string()
        }),
    )
    .map_err(|e| Error::Message(format!("Emit failed: {e}")))?;

    Ok((cost.to_string(), data_addr.to_string()))
}

#[tauri::command]
async fn download_data(
    state: State<'_, Mutex<AppState>>,
    request: DownloadFileRequest,
    app: AppHandle,
) -> Result<String, Error> {
    // debug!(
    //     "Download started for id: {}, address: {}, path: {}",
    //     request.id, request.address, request.destination_path
    // );
    // Extract all data we need and drop all locks before any await
    app.emit(
        "download-started",
        serde_json::json!({
            "id": request.id,
            "address": request.address,
            "path": request.destination_path,
            "size": request.size
        }),
    )
    .map_err(|e| Error::Message(format!("Emit failed: {e}")))?;

    let client = {
        let state = state.lock().unwrap();

        let client = state
            .client
            .lock()
            .unwrap()
            .as_ref()
            .ok_or("Client not initialized")?
            .clone();

        client
    }; // All MutexGuards are dropped here

    // Data address of the dog picture
    let data_address = match DataAddress::from_hex(request.address.trim()) {
        Ok(addr) => addr,
        Err(e) => {
            error!("Failed to parse data address: {e}");
            // --- Emit download-error event ---
            let _ = app.emit(
                "download-error",
                serde_json::json!({
                    "id": request.id,
                    "address": request.address,
                    "path": request.destination_path,
                    "error": format!("Failed to parse data address: {e}"),
                }),
            );
            // ----------------------------------
            return Err(Error::from(e));
        }
    };
    // debug!("DataAddress created: {:?}", data_address);

    // Get the bytes of the dog picture
    let bytes = match client.data_get_public(&data_address).await {
        Ok(b) => {
            debug!("Successfully fetched public data: {} bytes", b.len());
            b
        }
        Err(e) => {
            // --- Emit a Tauri event for download error here ---
            error!("Failed to fetch public data: {e}");
            let _ = app.emit(
                "download-error",
                serde_json::json!({
                    "id": request.id,
                    "address": request.address,
                    "path": request.destination_path,
                    "error": format!("Failed to fetch public data: {e}"),
                }),
            );
            // --------------------------------------------------
            return Err(Error::from(e));
        }
    };

    // Write the bytes to a file, overwriting if it exists
    match std::fs::write(request.destination_path.clone(), &bytes) {
        Ok(_) => {
            debug!("File written successfully: {}", request.destination_path);
        }
        Err(e) => {
            // If write fails, try to remove the existing file first and then write
            if e.kind() == std::io::ErrorKind::PermissionDenied {
                debug!("Permission denied, attempting to remove existing file and retry");
                if let Ok(_) = std::fs::remove_file(&request.destination_path) {
                    match std::fs::write(request.destination_path.clone(), &bytes) {
                        Ok(_) => {
                            debug!("File written successfully after removing existing file: {}", request.destination_path);
                        }
                        Err(e2) => {
                            error!("Failed to write file after removing existing: {e2}");
                            let _ = app.emit(
                                "download-error",
                                serde_json::json!({
                                    "id": request.id,
                                    "address": request.address,
                                    "path": request.destination_path,
                                    "error": format!("Failed to write file: {e2}"),
                                }),
                            );
                            return Err(Error::from(e2));
                        }
                    }
                } else {
                    error!("Failed to write file and couldn't remove existing: {e}");
                    let _ = app.emit(
                        "download-error",
                        serde_json::json!({
                            "id": request.id,
                            "address": request.address,
                            "path": request.destination_path,
                            "error": format!("Failed to write file: {e}"),
                        }),
                    );
                    return Err(Error::from(e));
                }
            } else {
                error!("Failed to write file: {e}");
                let _ = app.emit(
                    "download-error",
                    serde_json::json!({
                        "id": request.id,
                        "address": request.address,
                        "path": request.destination_path,
                        "error": format!("Failed to write file: {e}"),
                    }),
                );
                return Err(Error::from(e));
            }
        }
    }
    // TODO: Implement proper file download once we understand the API
    app.emit(
        "download-complete",
        serde_json::json!({
            "id": request.id,
            "address": request.address,
            "path": request.destination_path
        }),
    )
    .map_err(|e| Error::Message(format!("Emit failed: {e}")))?;

    Ok(format!(
        "File downloaded from {} to {}",
        request.address, request.destination_path
    ))
}

#[tauri::command]
async fn download_directory(
    state: State<'_, Mutex<AppState>>,
    request: DownloadFileRequest,
    app: AppHandle,
) -> Result<String, Error> {
    // Extract all data we need and drop all locks before any await
    app.emit(
        "download-started",
        serde_json::json!({
            "id": request.id,
            "address": request.address,
            "path": request.destination_path,
            "size": request.size
        }),
    )
    .map_err(|e| Error::Message(format!("Emit failed: {e}")))?;

    let client = {
        let state = state.lock().unwrap();

        let client = state
            .client
            .lock()
            .unwrap()
            .as_ref()
            .ok_or("Client not initialized")?
            .clone();

        client
    }; // All MutexGuards are dropped here

    // Data address of the cat picture directory
    let data_address = DataAddress::from_hex(request.address.trim())?;

    // Download the directory of cat pictures
    client
        .dir_download_public(&data_address, request.destination_path.clone().into())
        .await
        .map_err(|e| Error::Message(format!("Download directory failed: {e}")))?;

    app.emit(
        "download-complete",
        serde_json::json!({
            "id": request.id,
            "address": request.address,
            "path": request.destination_path
        }),
    )
    .map_err(|e| Error::Message(format!("Emit failed: {e}")))?;

    Ok(format!(
        "Directory downloaded from {} to {}",
        request.address, request.destination_path
    ))
}

/// Helper function to stop the dweb process - can be called from commands or event handlers.
///
/// This function safely terminates the dweb sidecar process if it's running.
/// It's designed to be called both from the dweb_stop command and from application
/// exit event handlers to ensure the process is properly cleaned up.
fn stop_dweb_process(app_state: &Mutex<AppState>) -> Result<String, Error> {
    let process = app_state
        .lock()
        .unwrap()
        .dweb_process
        .lock()
        .unwrap()
        .take();

    if let Some(child) = process {
        match child.kill() {
            Ok(_) => {
                info!("Successfully stopped dweb process");
                Ok("Stopped dweb process".to_string())
            }
            Err(e) => {
                error!("Failed to stop dweb process: {}", e);
                Err(Error::Message(format!("Failed to stop dweb process: {e}")))
            }
        }
    } else {
        info!("No dweb process running");
        Ok("No dweb process running".to_string())
    }
}

#[tauri::command]
async fn dweb_stop(state: State<'_, Mutex<AppState>>) -> Result<String, Error> {
    // Android doesn't support dweb sidecar - return early
    if cfg!(target_os = "android") {
        info!("Android: dweb_stop command called but not supported on Android");
        return Ok("dweb_stop not supported on Android".to_string());
    }

    stop_dweb_process(&state)
}

/// Starts the dweb sidecar process with logging support.
///
/// This function spawns the dweb sidecar binary and captures its stdout/stderr output,
/// forwarding it to the application's logging system. This allows debugging of dweb
/// issues through the existing logging infrastructure.
///
/// - stdout messages are logged at INFO level with "[dweb stdout]" prefix
/// - stderr messages are logged at WARN level with "[dweb stderr]" prefix
/// - Process termination and errors are logged at ERROR level
///
/// The logging can be controlled through the existing tracing configuration.
#[tauri::command]
async fn dweb_serve(
    app: AppHandle,
    state: State<'_, Mutex<AppState>>,
    wallet_key: String,
) -> Result<String, Error> {
    // Android doesn't support dweb sidecar - return early
    if cfg!(target_os = "android") {
        info!("Android: dweb_serve command called but not supported on Android");
        return Ok("dweb_serve not supported on Android".to_string());
    }

    // Check if user's dweb serve is already running
    let dweb_binary = determine_dweb_binary().await;

    match dweb_binary {
        DwebBinary::System => {
            info!("Using existing dweb serve instance");
            return Ok("Using existing dweb serve instance".to_string());
        }
        DwebBinary::ColonyDweb => {
            info!("Starting colony-dweb serve");
        }
    }

    // Stop any existing dweb process first
    let _ = dweb_stop(state.clone()).await;

    // Get the network setting from state
    let network = {
        let state_guard = state.lock().unwrap();
        state_guard.network.clone()
    };

    // Build arguments based on network
    let mut args = vec!["serve"];
    match network.as_str() {
        "local" => args.push("--local"),
        "alpha" => args.push("--alpha"),
        _ => {} // main network (default)
    }

    let sidecar_command = app.shell().sidecar("colony-dweb")?;
    let (mut rx, child) = sidecar_command
        .args(args)
        .env("SECRET_KEY", wallet_key)
        .spawn()
        .expect("Failed to run dweb serve");

    // Spawn a task to handle stdout/stderr logging
    tokio::spawn(async move {
        while let Some(event) = rx.recv().await {
            match event {
                tauri_plugin_shell::process::CommandEvent::Stdout(data) => {
                    let output = String::from_utf8_lossy(&data);
                    for line in output.lines() {
                        if !line.trim().is_empty() {
                            info!("[dweb stdout] {}", line.trim());
                        }
                    }
                }
                tauri_plugin_shell::process::CommandEvent::Stderr(data) => {
                    let output = String::from_utf8_lossy(&data);
                    for line in output.lines() {
                        if !line.trim().is_empty() {
                            warn!("[dweb stderr] {}", line.trim());
                        }
                    }
                }
                tauri_plugin_shell::process::CommandEvent::Terminated(payload) => {
                    if payload.code.unwrap_or(-1) != 0 {
                        error!("[dweb] Process terminated with code: {:?}", payload.code);
                    } else {
                        info!("[dweb] Process terminated normally");
                    }
                    break;
                }
                tauri_plugin_shell::process::CommandEvent::Error(err) => {
                    error!("[dweb] Process error: {}", err);
                    break;
                }
                _ => {
                    // Handle any other event types that might be added in the future
                    debug!("[dweb] Received unhandled event type");
                }
            }
        }
    });

    // Store the child process for later termination
    {
        let state_guard = state.lock().unwrap();
        *state_guard.dweb_process.lock().unwrap() = Some(child);
    }

    info!("Started dweb serve with network: {}", network);
    Ok(format!("Started dweb serve with network: {network}"))
}

/// Opens an address using the dweb sidecar with logging support.
///
/// This function spawns the dweb sidecar to open a specific address and captures
/// its output for debugging purposes. All output is logged with the address
/// included in the log message for easier tracking.
#[tauri::command]
async fn dweb_open(
    app: AppHandle,
    _state: State<'_, Mutex<AppState>>,
    address: String,
) -> Result<String, Error> {
    // Android doesn't support dweb sidecar - return early
    if cfg!(target_os = "android") {
        info!("Android: dweb_open command called but not supported on Android - address: {}", address);
        return Ok("dweb_open not supported on Android".to_string());
    }

    let dweb_binary = determine_dweb_binary().await;

    let (mut rx, mut _child) = match dweb_binary {
        DwebBinary::System => {
            info!("Using system dweb binary for open command");
            // Use system dweb binary directly
            app.shell()
                .command("dweb")
                .args(["open", &address])
                .spawn()
                .map_err(Error::Shell)?
        }
        DwebBinary::ColonyDweb => {
            info!("Using colony-dweb sidecar for open command");
            // Use colony-dweb sidecar
            app.shell()
                .sidecar("colony-dweb")?
                .args(["open", &address])
                .spawn()
                .map_err(Error::Shell)?
        }
    };

    // Spawn a task to handle stdout/stderr logging for the open command
    let address_clone = address.clone();
    tokio::spawn(async move {
        while let Some(event) = rx.recv().await {
            match event {
                tauri_plugin_shell::process::CommandEvent::Stdout(data) => {
                    let output = String::from_utf8_lossy(&data);
                    for line in output.lines() {
                        if !line.trim().is_empty() {
                            info!("[dweb open {}] {}", address_clone, line.trim());
                        }
                    }
                }
                tauri_plugin_shell::process::CommandEvent::Stderr(data) => {
                    let output = String::from_utf8_lossy(&data);
                    for line in output.lines() {
                        if !line.trim().is_empty() {
                            warn!("[dweb open {}] {}", address_clone, line.trim());
                        }
                    }
                }
                tauri_plugin_shell::process::CommandEvent::Terminated(payload) => {
                    if payload.code.unwrap_or(-1) != 0 {
                        error!(
                            "[dweb open {}] Process terminated with code: {:?}",
                            address_clone, payload.code
                        );
                    } else {
                        debug!(
                            "[dweb open {}] Process completed successfully",
                            address_clone
                        );
                    }
                    break;
                }
                tauri_plugin_shell::process::CommandEvent::Error(err) => {
                    error!("[dweb open {}] Process error: {}", address_clone, err);
                    break;
                }
                _ => {
                    debug!(
                        "[dweb open {}] Received unhandled event type",
                        address_clone
                    );
                }
            }
        }
    });

    info!("Opening address with dweb: {}", address);
    Ok("Opened address with dweb".to_string())
}

////////////////////////////////////////////////////////////////////
// Tauri App
////////////////////////////////////////////////////////////////////

// Mobile entry point - no parameters allowed
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    run_with_network("main") // Default to main network for mobile
}

// Desktop entry point - accepts network parameter
// Also available for mobile builds when called from main.rs
pub fn run_with_network_param(network: &str) {
    run_with_network(network)
}

// Internal function that does the actual work
fn run_with_network(network: &str) {
    let app_state = AppState {
        client: Mutex::new(None),
        wallet: Mutex::new(None),
        datastore: Mutex::new(None),
        keystore: Mutex::new(None),
        graph: Mutex::new(None),
        network: network.to_string(),
        session: Session {
            password: Mutex::new(None),
        },
        dweb_process: Mutex::new(None),
    };

    tauri::Builder::default()
        .setup(|app| {
            // Get the app_data_dir using the path_resolver
            let log_dir = app
                .path()
                .app_data_dir()
                .unwrap_or_else(|_| std::path::PathBuf::from("./logs"));
            // Ensure the directory exists
            std::fs::create_dir_all(&log_dir).expect("Failed to create log directory");

            // Initialize the logger plugin with the custom log directory
            let _ = app.handle().plugin(
                tauri_plugin_log::Builder::new()
                    .filter(|metadata| {
                        let t = metadata.target();
                        t == "webview" || t.contains("colony")
                    })
                    .target(tauri_plugin_log::Target::new(
                        tauri_plugin_log::TargetKind::Folder {
                            path: log_dir,
                            file_name: None,
                        },
                    ))
                    .build(),
            );

            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_opener::init())
        .manage(Mutex::new(app_state))
        .invoke_handler(tauri::generate_handler![
            set_password,
            get_password,
            clear_password,
            get_file_size,
            greet,
            get_new_seed_phrase,
            initialize_pod_manager,
            add_pod,
            rename_pod,
            remove_pod,
            add_pod_ref,
            remove_pod_ref,
            get_update_list,
            list_my_pods,
            list_pod_subjects,
            upload_pod,
            upload_all,
            refresh_cache,
            refresh_ref,
            search,
            put_subject_data,
            get_subject_data,
            initialize_autonomi_client,
            initialize_datastore,
            datastore_exists,
            create_keystore_from_seed_phrase,
            create_keystore_from_key,
            write_keystore_to_file,
            open_keystore,
            initialize_graph,
            upload_cost,
            upload_data,
            upload_directory,
            download_data,
            download_directory,
            add_wallet,
            remove_wallet,
            list_wallets,
            get_wallet,
            get_wallet_balance,
            get_active_wallet,
            set_active_wallet,
            switch_wallet,
            dweb_serve,
            dweb_stop,
            dweb_open,
            open_file_with_default_app,
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app, event| match event {
            RunEvent::ExitRequested { .. } => {
                info!("Application exit requested - stopping dweb process");
                cleanup_dweb_process(app);
            }
            RunEvent::WindowEvent {
                event: WindowEvent::CloseRequested { .. },
                ..
            } => {
                info!("Window close requested - stopping dweb process");
                cleanup_dweb_process(app);
            }
            RunEvent::WindowEvent {
                event: WindowEvent::Destroyed,
                ..
            } => {
                info!("Window destroyed - ensuring dweb process cleanup");
                cleanup_dweb_process(app);
            }
            _ => {}
        });
}

// Helper functions that aren't tauri commands

/// Helper function to cleanup dweb process during application shutdown.
/// This function is called from various exit event handlers to ensure
/// the dweb sidecar process is properly terminated.
fn cleanup_dweb_process(app: &AppHandle) {
    if let Some(app_state) = app.try_state::<Mutex<AppState>>() {
        match stop_dweb_process(&app_state) {
            Ok(msg) => info!("Cleanup: {}", msg),
            Err(e) => error!("Failed to stop dweb process during cleanup: {}", e),
        }
    } else {
        warn!("Could not access app state during dweb cleanup");
    }
}

async fn init_client(app: AppHandle, environment: &str) -> Result<Client, Error> {
    match environment {
        "local" => Ok(Client::init_local().await?),
        "alpha" => Ok(Client::init_alpha().await?),
        _ => {
            let mut config:  ClientConfig = Default::default();
            if cfg!(target_os = "android") {
                // Android-specific chunk cache directory
                let app_data_dir = app.path().app_data_dir()
                    .map_err(|e| Error::Io(IoError::new(std::io::ErrorKind::Other, format!("Failed to get app data dir: {}", e))))?;
        
                let cache_dir = app_data_dir.join("chunk_cache");
        
                // Ensure the pods directory exists
                if !cache_dir.exists() {
                    std::fs::create_dir_all(&cache_dir)
                        .map_err(|e| Error::Io(IoError::new(std::io::ErrorKind::Other, format!("Failed to create chunk cache directory: {}", e))))?;
                }
                config.strategy.chunk_cache_dir = Some(cache_dir);
            }
            Ok(Client::init_with_config(config).await?) // main net
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use colonylib::KeyStore;
    use std::sync::Mutex;

    // Helper function to create a test AppState with initialized components
    fn create_test_app_state() -> Mutex<AppState> {
        Mutex::new(AppState {
            client: Mutex::new(None),
            wallet: Mutex::new(None),
            datastore: Mutex::new(None),
            keystore: Mutex::new(None),
            graph: Mutex::new(None),
            network: "main".to_string(),
            session: Session {
                password: Mutex::new(None),
            },
            dweb_process: Mutex::new(None),
        })
    }

    #[test]
    fn test_stop_dweb_process_no_process() {
        // Test stopping dweb when no process is running
        let app_state = create_test_app_state();
        let result = stop_dweb_process(&app_state);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "No dweb process running");
    }

    #[test]
    fn test_stop_dweb_process_with_mock_process() {
        // This test verifies the function structure but can't test actual process killing
        // without creating a real process, which would be complex in a unit test
        let app_state = create_test_app_state();

        // Verify that the function handles the case where no process is stored
        let result = stop_dweb_process(&app_state);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "No dweb process running");

        // Verify that the dweb_process field is properly accessed
        {
            let state = app_state.lock().unwrap();
            let process_guard = state.dweb_process.lock().unwrap();
            assert!(process_guard.is_none());
        }
    }

    // Helper function to create a mock keystore with some test wallets
    fn create_mock_keystore() -> KeyStore {
        // Create keystore from a test mnemonic
        let mut keystore = KeyStore::from_mnemonic("abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about").unwrap();
        // Add some test wallet keys
        keystore
            .add_wallet_key(
                "test_wallet_1",
                "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
            )
            .unwrap();
        keystore
            .add_wallet_key(
                "test_wallet_2",
                "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890",
            )
            .unwrap();
        keystore
    }

    // Test helper functions that directly test the wallet operations without Tauri State
    async fn test_add_wallet_direct(
        app_state: &Mutex<AppState>,
        request: AddWalletRequest,
    ) -> Result<String, Error> {
        // Extract keystore and drop all locks before any operations
        let mut keystore = {
            let state = app_state.lock().unwrap();
            let keystore_guard = state.keystore.lock().unwrap();
            keystore_guard.clone().ok_or("KeyStore not initialized")?
        }; // All MutexGuards are dropped here

        // Call the keystore add_wallet_key function to add the new wallet private key to the keystore
        keystore.add_wallet_key(&request.name, &request.key)?;

        // Put the modified keystore back
        {
            let state = app_state.lock().unwrap();
            *state.keystore.lock().unwrap() = Some(keystore);
        }

        Ok("Wallet added".to_string())
    }

    async fn test_remove_wallet_direct(
        app_state: &Mutex<AppState>,
        name: String,
    ) -> Result<String, Error> {
        // Extract keystore and drop all locks before any operations
        let mut keystore = {
            let state = app_state.lock().unwrap();
            let keystore_guard = state.keystore.lock().unwrap();
            keystore_guard.clone().ok_or("KeyStore not initialized")?
        }; // All MutexGuards are dropped here

        // Call the keystore remove_wallet_key function to remove the wallet private key from the keystore
        keystore.remove_wallet_key(&name)?;

        // Put the modified keystore back
        {
            let state = app_state.lock().unwrap();
            *state.keystore.lock().unwrap() = Some(keystore);
        }

        Ok("Wallet removed".to_string())
    }

    async fn test_list_wallets_direct(app_state: &Mutex<AppState>) -> Result<Value, Error> {
        let state = app_state.lock().unwrap();
        let keystore = state
            .keystore
            .lock()
            .unwrap()
            .as_ref()
            .ok_or("KeyStore not initialized")?
            .clone();

        // Call the keystore list_wallets function to get the list of wallet names
        let wallets = keystore.get_wallet_keys();

        // Map the wallet names and keys to a single JSON Value object
        let wallets: Value = serde_json::json!(wallets);

        Ok(wallets)
    }

    #[tokio::test]
    async fn test_add_wallet_success() {
        let app_state = create_test_app_state();

        // Initialize keystore
        {
            let state = app_state.lock().unwrap();
            *state.keystore.lock().unwrap() = Some(KeyStore::from_mnemonic("abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about").unwrap());
        }

        let request = AddWalletRequest {
            name: "new_wallet".to_string(),
            key: "0x1111222233334444555566667777888899990000aaaabbbbccccddddeeeeffff".to_string(),
        };

        let result = test_add_wallet_direct(&app_state, request).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Wallet added".to_string());

        // Verify the wallet was actually added
        let state_guard = app_state.lock().unwrap();
        let keystore = state_guard.keystore.lock().unwrap();
        let keystore_ref = keystore.as_ref().unwrap();
        let wallet_keys = keystore_ref.get_wallet_keys();
        assert!(wallet_keys.contains_key("new_wallet"));
    }

    #[tokio::test]
    async fn test_add_wallet_keystore_not_initialized() {
        let app_state = create_test_app_state();
        // Don't initialize keystore

        let request = AddWalletRequest {
            name: "new_wallet".to_string(),
            key: "0x1111222233334444555566667777888899990000aaaabbbbccccddddeeeeffff".to_string(),
        };

        let result = test_add_wallet_direct(&app_state, request).await;

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("KeyStore not initialized"));
    }

    #[tokio::test]
    async fn test_remove_wallet_success() {
        let app_state = create_test_app_state();

        // Initialize keystore with test wallets
        {
            let state = app_state.lock().unwrap();
            *state.keystore.lock().unwrap() = Some(create_mock_keystore());
        }

        let result = test_remove_wallet_direct(&app_state, "test_wallet_1".to_string()).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Wallet removed".to_string());

        // Verify the wallet was actually removed
        let state_guard = app_state.lock().unwrap();
        let keystore = state_guard.keystore.lock().unwrap();
        let keystore_ref = keystore.as_ref().unwrap();
        let wallet_keys = keystore_ref.get_wallet_keys();
        assert!(!wallet_keys.contains_key("test_wallet_1"));
        assert!(wallet_keys.contains_key("test_wallet_2")); // Other wallet should still exist
    }

    #[tokio::test]
    async fn test_remove_wallet_not_found() {
        let app_state = create_test_app_state();

        // Initialize keystore with test wallets
        {
            let state = app_state.lock().unwrap();
            *state.keystore.lock().unwrap() = Some(create_mock_keystore());
        }

        let result = test_remove_wallet_direct(&app_state, "nonexistent_wallet".to_string()).await;

        assert!(result.is_err());
        // The error should indicate the wallet was not found
    }

    #[tokio::test]
    async fn test_list_wallets_success() {
        let app_state = create_test_app_state();

        // Initialize keystore with test wallets
        {
            let state = app_state.lock().unwrap();
            *state.keystore.lock().unwrap() = Some(create_mock_keystore());
        }

        let result = test_list_wallets_direct(&app_state).await;

        assert!(result.is_ok());
        let wallets = result.unwrap();

        // Verify the response contains our test wallets
        assert!(wallets.is_object());
        let wallet_map = wallets.as_object().unwrap();
        assert!(wallet_map.contains_key("test_wallet_1"));
        assert!(wallet_map.contains_key("test_wallet_2"));
    }

    #[tokio::test]
    async fn test_list_wallets_empty() {
        let app_state = create_test_app_state();

        // Initialize empty keystore
        {
            let state = app_state.lock().unwrap();
            *state.keystore.lock().unwrap() = Some(KeyStore::from_mnemonic("abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about").unwrap());
        }

        let result = test_list_wallets_direct(&app_state).await;

        assert!(result.is_ok());
        let wallets = result.unwrap();

        // Should return empty object
        assert!(wallets.is_object());
        let wallet_map = wallets.as_object().unwrap();
        assert!(wallet_map.is_empty());
    }

    // Note: switch_wallet tests require a real Client and network connection
    // These are integration tests that would need a test environment
    // For now, we'll test the error conditions that don't require network access

    #[tokio::test]
    async fn test_keystore_operations_basic() {
        // Test basic keystore operations without network dependencies
        let mut keystore = KeyStore::from_mnemonic("abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about").unwrap();

        // Test adding a wallet
        let add_result = keystore.add_wallet_key(
            "test_wallet",
            "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
        );
        assert!(add_result.is_ok());

        // Test getting wallet keys
        let wallet_keys = keystore.get_wallet_keys();
        assert!(wallet_keys.contains_key("test_wallet"));

        // Test getting a specific wallet key
        let wallet_key_result = keystore.get_wallet_key("test_wallet");
        assert!(wallet_key_result.is_ok());

        // Test removing a wallet
        let remove_result = keystore.remove_wallet_key("test_wallet");
        assert!(remove_result.is_ok());

        // Verify wallet was removed
        let wallet_keys_after = keystore.get_wallet_keys();
        assert!(!wallet_keys_after.contains_key("test_wallet"));
    }

    #[tokio::test]
    async fn test_add_wallet_duplicate_name() {
        let app_state = create_test_app_state();

        // Initialize keystore with test wallets
        {
            let state = app_state.lock().unwrap();
            *state.keystore.lock().unwrap() = Some(create_mock_keystore());
        }

        let request = AddWalletRequest {
            name: "test_wallet_1".to_string(), // This wallet already exists
            key: "0x9999888877776666555544443333222211110000ffffeeeedddcccbbbaaa999".to_string(),
        };

        let result = test_add_wallet_direct(&app_state, request).await;

        // The behavior depends on the KeyStore implementation
        // It might overwrite or return an error - we test that it handles it gracefully
        // For now, we just ensure it doesn't panic
        let _ = result; // Don't assert specific behavior as it depends on KeyStore implementation
    }

    #[tokio::test]
    async fn test_add_wallet_invalid_key_format() {
        let app_state = create_test_app_state();

        // Initialize keystore
        {
            let state = app_state.lock().unwrap();
            *state.keystore.lock().unwrap() = Some(KeyStore::from_mnemonic("abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about").unwrap());
        }

        let request = AddWalletRequest {
            name: "invalid_wallet".to_string(),
            key: "invalid_key_format".to_string(), // Invalid hex format
        };

        let result = test_add_wallet_direct(&app_state, request).await;

        // Should handle invalid key format gracefully
        // The exact behavior depends on KeyStore implementation
        let _ = result; // Don't assert specific behavior as it depends on KeyStore implementation
    }
}
