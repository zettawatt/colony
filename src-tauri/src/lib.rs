// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
//use colony::config::generate_seed_phrase;
use anttp::config::anttp_config::AntTpConfig;
use autonomi::client::config::ClientConfig;
use autonomi::client::payment::PaymentOption;
use autonomi::client::quote::CostError;
use autonomi::client::ConnectError;
use autonomi::client::{GetError, PutError};
use autonomi::data::DataAddress;
use autonomi::{AddressParseError, Bytes, Client, Wallet};
use colonylib::data::Error as DataStoreError;
use colonylib::graph::Error as GraphError;
use colonylib::key::Error as KeyStoreError;
use colonylib::pod::Error as PodError;
use colonylib::{DataStore, Graph, KeyStore, PodManager};
use dweb::client::DwebClientConfig;
use dweb_server::DwebService;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::io::Error as IoError;
use std::sync::Mutex;
use std::sync::{MutexGuard, PoisonError};
use tauri::Manager;
use tauri::{AppHandle, Emitter, RunEvent, State, WindowEvent};
use tokio::task::JoinHandle;
use tracing::{debug, error, info, warn};

/// Check if anttp server is already running by querying the REST API
async fn is_anttp_running() -> bool {
    let client = reqwest::Client::new();

    // Try the default anttp port (18888)
    match client
        .get("http://127.0.0.1:18888/")
        .timeout(std::time::Duration::from_secs(2))
        .send()
        .await
    {
        Ok(response) => {
            // Any HTTP response (including 404) means the server is running
            // We just care that we can connect to the port
            info!(
                "Detected running anttp server on port 18888 (status: {})",
                response.status()
            );
            true
        }
        Err(e) => {
            debug!("No anttp server detected on port 18888: {}", e);
            false
        }
    }
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

#[tauri::command]
fn get_file_size(path: String) -> Result<u64, String> {
    fs::metadata(path)
        .map(|meta| meta.len())
        .map_err(|e| e.to_string())
}

// File opener command for Android
#[tauri::command]
async fn open_file_with_default_app(
    file_path: String,
    app: tauri::AppHandle,
) -> Result<String, String> {
    if cfg!(target_os = "android") {
        open_file_via_socket_communication(&file_path, &app).await
    } else {
        Err("This command is only available on Android".to_string())
    }
}

#[cfg(target_os = "android")]
async fn open_file_via_socket_communication(
    file_path: &str,
    _app: &tauri::AppHandle,
) -> Result<String, String> {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpStream;
    use tokio::time::{timeout, Duration};

    let address = "127.0.0.1:8765";

    let mut stream = timeout(Duration::from_secs(5), TcpStream::connect(address))
        .await
        .map_err(|_| "Timeout connecting to socket".to_string())?
        .map_err(|e| format!("Failed to connect to socket: {}", e))?;

    stream
        .write_all(file_path.as_bytes())
        .await
        .map_err(|e| format!("Failed to write to socket: {}", e))?;

    stream
        .shutdown()
        .await
        .map_err(|e| format!("Failed to shutdown write: {}", e))?;

    let mut response = String::new();
    timeout(Duration::from_secs(5), stream.read_to_string(&mut response))
        .await
        .map_err(|_| "Timeout reading response".to_string())?
        .map_err(|e| format!("Failed to read response: {}", e))?;

    Ok(response.trim().to_string())
}

#[cfg(not(target_os = "android"))]
async fn open_file_via_socket_communication(
    _file_path: &str,
    _app: &tauri::AppHandle,
) -> Result<String, String> {
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
    pub dweb_service: Mutex<Option<DwebService>>,
    pub anttp_handle: Mutex<Option<JoinHandle<std::io::Result<()>>>>,
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
    pub id: String,
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
fn initialize_datastore(
    app: AppHandle,
    state: State<'_, Mutex<AppState>>,
) -> Result<String, Error> {
    let datastore = if cfg!(target_os = "android") {
        // Android-specific initialization using from_paths
        let app_data_dir = app
            .path()
            .app_data_dir()
            .map_err(|e| Error::Io(IoError::other(format!("Failed to get app data dir: {e}"))))?;

        let data_dir = app_data_dir.clone();
        let pods_dir = app_data_dir.join("pods");

        // Ensure the pods directory exists
        if !pods_dir.exists() {
            std::fs::create_dir_all(&pods_dir).map_err(|e| {
                Error::Io(IoError::other(format!(
                    "Failed to create pods directory: {e}"
                )))
            })?;
        }

        info!(
            "Android: Using data_dir: {:?}, pods_dir: {:?}",
            data_dir, pods_dir
        );
        // Use standard Android Downloads directory
        let downloads_dir = std::path::PathBuf::from("/storage/emulated/0/Download");
        if !downloads_dir.exists() {
            std::fs::create_dir_all(&downloads_dir).map_err(|e| {
                Error::Io(IoError::other(format!(
                    "Failed to create downloads directory: {e}"
                )))
            })?;
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
    // First, set the active wallet
    let wallet_result: Result<(String, String), Error> = {
        let state_guard = state.lock().unwrap();

        let mut keystore = state_guard
            .keystore
            .lock()
            .unwrap()
            .as_ref()
            .ok_or("KeyStore not initialized")?
            .clone();

        let datastore = state_guard
            .datastore
            .lock()
            .unwrap()
            .as_ref()
            .ok_or("DataStore not initialized")?
            .clone();

        // Set the active wallet
        let (name, address) = keystore.set_active_wallet(&name)?;
        datastore.set_active_wallet(&name, &address)?;

        Ok((name, address))
    }; // Release all locks

    // Handle the result
    let (wallet_name, wallet_address) = wallet_result?;
    info!("Active wallet set to: {wallet_name}");

    // Check if DwebService needs to be restarted (no locks held)
    let is_running = is_dweb_serve_running().await;
    if is_running {
        info!("Restarting DwebService with new active wallet");
        match initialize_dweb_service_with_wallet(&state) {
            Ok(_) => {
                // Restart the service on port 5537
                let state_guard = state.lock().unwrap();
                let mut dweb_service_guard = state_guard.dweb_service.lock().unwrap();
                if let Some(dweb_service) = dweb_service_guard.as_mut() {
                    dweb_service.start(5537, None);
                    info!("DwebService restarted with new active wallet");
                }
            }
            Err(e) => {
                warn!("Failed to restart DwebService with new wallet: {}", e);
            }
        }
    }

    Ok((wallet_name, wallet_address))
}

#[tauri::command]
async fn switch_wallet(
    _app: AppHandle,
    state: State<'_, Mutex<AppState>>,
    name: String,
) -> Result<String, Error> {
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
    info!("EVM network: {evm_network:?}");

    // Create new wallet with the specified key
    let wallet = match Wallet::new_from_private_key(evm_network.clone(), &wallet_key) {
        Ok(wallet) => wallet,
        Err(e) => {
            // Restore the client on failure
            let state_guard = state.lock().unwrap();
            *state_guard.client.lock().unwrap() = Some(client);
            return Err(Error::Message(format!("Failed to create wallet: {e}")));
        }
    };

    // Store the wallet and client back in state
    {
        let state_guard = state.lock().unwrap();
        *state_guard.client.lock().unwrap() = Some(client);
        *state_guard.wallet.lock().unwrap() = Some(wallet);
    } // Release the lock

    info!("Wallet switched to: {}", name);

    // Check if DwebService needs to be restarted (no locks held)
    let is_running = is_dweb_serve_running().await;
    if is_running {
        info!("Restarting DwebService with switched wallet");
        match initialize_dweb_service_with_wallet(&state) {
            Ok(_) => {
                // Restart the service on port 5537
                let state_guard = state.lock().unwrap();
                let mut dweb_service_guard = state_guard.dweb_service.lock().unwrap();
                if let Some(dweb_service) = dweb_service_guard.as_mut() {
                    dweb_service.start(5537, None);
                    info!("DwebService restarted with switched wallet");
                }
            }
            Err(e) => {
                warn!("Failed to restart DwebService with switched wallet: {}", e);
            }
        }
    }

    Ok(format!("Successfully switched to wallet: {name}"))
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
                if std::fs::remove_file(&request.destination_path).is_ok() {
                    match std::fs::write(request.destination_path.clone(), &bytes) {
                        Ok(_) => {
                            debug!(
                                "File written successfully after removing existing file: {}",
                                request.destination_path
                            );
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

/// Helper function to stop the dweb service - can be called from commands or event handlers.
///
/// This function safely stops the DwebService if it's running.
/// It's designed to be called both from the dweb_stop command and from application
/// exit event handlers to ensure the service is properly cleaned up.
fn stop_dweb_service(app_state: &Mutex<AppState>) -> Result<String, Error> {
    let state_guard = app_state.lock().unwrap();
    let mut dweb_service_guard = state_guard.dweb_service.lock().unwrap();
    if let Some(dweb_service) = dweb_service_guard.as_mut() {
        // Stop the service by starting it on port 0 (which disables it)
        dweb_service.start(0, None);
        info!("Successfully stopped dweb service");
        Ok("Stopped dweb service".to_string())
    } else {
        info!("No dweb service running");
        Ok("No dweb service running".to_string())
    }
}

/// Helper function to initialize or restart the DwebService with Colony's active wallet.
///
/// This function creates a new DwebService configured with the active wallet from Colony's
/// keystore and datastore. It should be called when:
/// - Colony initializes and connects to the Autonomi network
/// - The active wallet changes
/// - The dweb service needs to be restarted with updated wallet configuration
fn initialize_dweb_service_with_wallet(app_state: &Mutex<AppState>) -> Result<String, Error> {
    let (wallet_key, network) = {
        let state_guard = app_state.lock().unwrap();

        // Get the keystore and datastore
        let keystore = state_guard
            .keystore
            .lock()
            .unwrap()
            .as_ref()
            .ok_or("KeyStore not initialized")?
            .clone();

        let datastore = state_guard
            .datastore
            .lock()
            .unwrap()
            .as_ref()
            .ok_or("DataStore not initialized")?
            .clone();

        let network = state_guard.network.clone();

        // Get the active wallet
        let (wallet_name, _wallet_address) = datastore
            .get_active_wallet()
            .map_err(|e| Error::Message(format!("Failed to get active wallet: {e}")))?;

        // Get the wallet private key
        let wallet_key = keystore
            .get_wallet_key(&wallet_name)
            .map_err(|e| Error::Message(format!("Failed to get wallet key: {e}")))?;

        (wallet_key, network)
    }; // Release all locks

    // Create the wallet for the DwebClientConfig
    // For local networks, we'll use ArbitrumSepoliaTest as the EVM network since
    // local networks typically use test configurations
    let evm_network = match network.as_str() {
        "local" => autonomi::Network::ArbitrumSepoliaTest, // Local uses test network
        "alpha" => autonomi::Network::ArbitrumSepoliaTest,
        _ => autonomi::Network::ArbitrumOne, // main network
    };

    let wallet = Wallet::new_from_private_key(evm_network.clone(), &wallet_key)
        .map_err(|e| Error::Message(format!("Failed to create wallet: {e}")))?;

    // Create DwebClientConfig with the wallet
    let dweb_config = DwebClientConfig {
        wallet: Some(wallet),
        local_network: network == "local",
        alpha_network: network == "alpha",
        ..DwebClientConfig::default()
    };

    // Stop any existing service and create a new one with the wallet
    {
        let state_guard = app_state.lock().unwrap();
        let mut dweb_service_guard = state_guard.dweb_service.lock().unwrap();

        // Stop existing service if running
        if let Some(existing_service) = dweb_service_guard.as_mut() {
            existing_service.start(0, None); // Stop the service
        }

        // Create new service with wallet configuration
        *dweb_service_guard = Some(DwebService::new(dweb_config));
    }

    info!("DwebService initialized with active wallet");
    Ok("DwebService initialized with active wallet".to_string())
}

#[tauri::command]
async fn dweb_stop(state: State<'_, Mutex<AppState>>) -> Result<String, Error> {
    stop_dweb_service(&state)
}

/// Helper function to stop the anttp server - can be called from commands or event handlers.
///
/// This function safely terminates the anttp server if it's running.
/// It's designed to be called both from the anttp_stop command and from application
/// exit event handlers to ensure the server is properly cleaned up.
fn stop_anttp_server(app_state: &Mutex<AppState>) -> Result<String, Error> {
    let handle = app_state
        .lock()
        .unwrap()
        .anttp_handle
        .lock()
        .unwrap()
        .take();

    if let Some(join_handle) = handle {
        // Abort the tokio task
        join_handle.abort();

        // Call anttp's stop_server function to gracefully shutdown
        tokio::spawn(async {
            if let Err(e) = anttp::stop_server().await {
                error!("Failed to gracefully stop anttp server: {}", e);
            }
        });

        info!("Successfully stopped anttp server");
        Ok("Stopped anttp server".to_string())
    } else {
        info!("No anttp server running");
        Ok("No anttp server running".to_string())
    }
}

#[tauri::command]
async fn anttp_stop(state: State<'_, Mutex<AppState>>) -> Result<String, Error> {
    stop_anttp_server(&state)
}

#[tauri::command]
async fn anttp_start(
    app: AppHandle,
    state: State<'_, Mutex<AppState>>,
    wallet_key: String,
) -> Result<String, Error> {
    // Check if anttp is already running on the system
    if is_anttp_running().await {
        info!("anttp server already running on port 18888");
        return Ok("anttp server already running on port 18888".to_string());
    }

    // Stop any existing anttp server first
    let _ = anttp_stop(state.clone()).await;

    // Get the network setting from state and extract keystore and datastore
    let (network, keystore, datastore) = {
        let state_guard = state.lock().unwrap();
        let network = state_guard.network.clone();
        let keystore = state_guard
            .keystore
            .lock()
            .unwrap()
            .as_ref()
            .ok_or("KeyStore not initialized")?
            .clone();
        let datastore = state_guard
            .datastore
            .lock()
            .unwrap()
            .as_ref()
            .ok_or("DataStore not initialized")?
            .clone();
        (network, keystore, datastore)
    };

    // FIXME: using the first wallet key as the app_private_key. Unsure if anttp needs a specific key
    let app_private_key = {
        let wallet_keys = keystore.get_wallet_keys();
        if let Some((_, key)) = wallet_keys.iter().next() {
            key.clone()
        } else {
            // If no wallet keys exist, use empty string (will need to be set later)
            String::new()
        }
    };

    // Get data directory and create anttp directories
    let data_dir = if cfg!(target_os = "android") {
        app.path()
            .app_data_dir()
            .map_err(|e| Error::Io(IoError::other(format!("Failed to get app data dir: {e}"))))?
    } else {
        // For desktop, we need to get the data directory from DataStore
        // Since we don't have a direct method, we'll derive it from keystore path
        let keystore_path = datastore.get_keystore_path();
        keystore_path
            .parent()
            .ok_or("Failed to get parent directory of keystore")?
            .to_path_buf()
    };

    // Create anttp directory structure
    let anttp_dir = data_dir.join("anttp");
    let static_files_dir = anttp_dir.join("static_files");
    let map_cache_dir = anttp_dir.join("map_cache");

    // Ensure directories exist
    std::fs::create_dir_all(&static_files_dir).map_err(|e| {
        Error::Io(IoError::other(format!(
            "Failed to create static files directory: {e}"
        )))
    })?;
    std::fs::create_dir_all(&map_cache_dir).map_err(|e| {
        Error::Io(IoError::other(format!(
            "Failed to create map cache directory: {e}"
        )))
    })?;

    // Create anttp configuration
    let anttp_config = AntTpConfig {
        listen_address: "127.0.0.1:18888".parse().unwrap(),
        static_file_directory: static_files_dir.to_string_lossy().to_string(),
        wallet_private_key: wallet_key,
        download_threads: 8,
        app_private_key,
        bookmarks: vec![
            "traktion-blog=8e16406561d0c460f3dbe37fef129582d6410ec7cb9d5aebdf9cbb051676624c543a315f7e857103cd71088a927c9085".to_string(),
            "imim=959c2ba5b84e1a68fedc14caaae96e97cfff19ff381127844586b2e0cdd2afdfb1687086a5668bced9f3dc35c03c9bd7".to_string(),
            "gimim=82fb48d691a65e771e2279ff56d8c5f7bc007fa386c9de95d64be52e081f01b1fdfb248095238b93db820836cc88c67a".to_string(),
            "index=b970cf40a1ba880ecc27d5495f543af387fcb014863d0286dd2b1518920df38ac311d854013de5d50b9b04b84a6da021".to_string(),
            "gindex=879d061580e6200a3f1dbfc5c87c13544fcd391dfec772033f1138a9469df35c98429ecd3acb4a9ab631ea7d5f6fae0f".to_string(),
            "cinema=953ff297c689723a59e20d6f80b67233b0c0fe17ff4cb37a2c8cfb46e276ce0e45d59c17e006e4990deaa634141e4c77".to_string(),
        ],
        uploads_disabled: false,
        cached_mutable_ttl: 5,
        peers: vec![],
        map_cache_directory: map_cache_dir.to_string_lossy().to_string(),
        evm_network: "main".to_string(),
        immutable_disk_cache_size: 1024,
        immutable_memory_cache_size: 32,
        idle_disconnect: 30,
    };

    // Clone network for logging after the spawn
    let network_clone = network.clone();

    // Spawn anttp server in a tokio task
    let handle = tokio::spawn(async move {
        info!(
            "Starting anttp server on 127.0.0.1:18888 with network: {}",
            network
        );
        anttp::run_server(anttp_config).await
    });

    // Store the handle for later termination
    {
        let state_guard = state.lock().unwrap();
        *state_guard.anttp_handle.lock().unwrap() = Some(handle);
    }

    info!("Started anttp server with network: {}", network_clone);
    Ok(format!(
        "Started anttp server with network: {network_clone}"
    ))
}

/// Starts the dweb serve using the DwebService library.
///
/// This function manages the lifecycle of the dweb serve, including:
/// - Checking if dweb serve is already running
/// - Initializing DwebService with Colony's active wallet
/// - Starting the DwebService with appropriate port (5537)
///
/// # Arguments
/// * `app` - The Tauri application handle (unused but kept for compatibility)
/// * `state` - The application state containing the dweb service
/// * `wallet_key` - The wallet secret key (unused - we get it from Colony's active wallet)
///
/// # Returns
/// * `Ok(String)` - Success message indicating the serve was started or already running
/// * `Err(Error)` - If the service failed to start or other errors occurred
///
/// # Behavior
/// - If dweb serve is already running, it returns early without starting another instance
/// - Initializes DwebService with Colony's active wallet configuration
/// - Uses DwebService.start() to start the service on port 5537
/// - The service runs in the background and handles requests
#[tauri::command]
async fn dweb_serve(
    _app: AppHandle,
    state: State<'_, Mutex<AppState>>,
    _wallet_key: String,
) -> Result<String, Error> {
    // Check if dweb serve is already running
    if is_dweb_serve_running().await {
        info!("dweb serve is already running, not starting another instance");
        return Ok("dweb serve is already running".to_string());
    }

    // Initialize DwebService with Colony's active wallet
    initialize_dweb_service_with_wallet(&state)?;

    // Start the DwebService
    {
        let state_guard = state.lock().unwrap();
        let mut dweb_service_guard = state_guard.dweb_service.lock().unwrap();
        if let Some(dweb_service) = dweb_service_guard.as_mut() {
            dweb_service.start(5537, None);
            info!("Started dweb serve on port 5537 with active wallet");
            Ok("Started dweb serve on port 5537 with active wallet".to_string())
        } else {
            Err(Error::Message("DwebService not initialized".to_string()))
        }
    }
}

/// Opens an address using the dweb REST API.
///
/// This function makes a REST API call to the dweb server to open a specific address,
/// similar to the example.rs implementation. It constructs the URL and opens it in
/// the default browser using tauri_plugin_opener for cross-platform compatibility.
///
/// # Arguments
/// * `_app` - The Tauri application handle (unused but kept for compatibility)
/// * `_state` - The application state (unused but kept for compatibility)
/// * `address` - The address/name/link to open
///
/// # Returns
/// * `Ok(String)` - Success message indicating the address was opened
/// * `Err(Error)` - If the operation failed
#[tauri::command]
async fn dweb_open(
    _app: AppHandle,
    _state: State<'_, Mutex<AppState>>,
    address: String,
) -> Result<String, Error> {
    info!("Opening address with dweb: {}", address);

    let main_server = "http://127.0.0.1:5537";
    let url = format!("{main_server}/dweb-open/{address}");

    info!("dweb_open() opening {}", url);

    // Use tauri_plugin_opener to open the URL in the default browser
    match tauri_plugin_opener::open_url(&url, None::<String>) {
        Ok(_) => {
            info!("Successfully opened dweb URL: {}", url);
            Ok(format!("Opened address with dweb: {}", address))
        }
        Err(e) => {
            error!("Failed to open dweb URL {}: {}", url, e);
            Err(Error::Message(format!("Failed to open dweb URL: {}", e)))
        }
    }
}

/// Opens an anttp website by making a REST API call to the anttp server
/// and then opening the resulting URL in the default web browser.
///
/// This function is similar to dweb_open but for anttp websites. Instead of
/// directly opening a browser, it calls the anttp REST API to get the proper
/// URL and then opens that in the default browser.
#[tauri::command]
async fn anttp_open(address: String) -> Result<String, Error> {
    info!("Opening anttp address: {}", address);

    // Check if anttp server is running
    if !is_anttp_running().await {
        return Err(Error::Message(
            "anttp server is not running. Please start the anttp server first.".to_string(),
        ));
    }

    // Make REST API call to anttp server to get the proper URL
    let client = reqwest::Client::new();
    let anttp_url = format!("http://127.0.0.1:18888/{}", address);

    match client
        .get(&anttp_url)
        .timeout(std::time::Duration::from_secs(60))
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                // Open the URL in the default browser
                match tauri_plugin_opener::open_url(&anttp_url, None::<String>) {
                    Ok(_) => {
                        info!(
                            "Successfully opened anttp address in browser: {}",
                            anttp_url
                        );
                        Ok(format!("Opened anttp address in browser: {}", address))
                    }
                    Err(e) => {
                        error!("Failed to open anttp URL in browser: {}", e);
                        Err(Error::Message(format!(
                            "Failed to open anttp URL in browser: {}",
                            e
                        )))
                    }
                }
            } else {
                let error_msg =
                    format!("anttp server returned error status: {}", response.status());
                error!("{}", error_msg);
                Err(Error::Message(error_msg))
            }
        }
        Err(e) => {
            let error_msg = format!("Failed to connect to anttp server: {}", e);
            error!("{}", error_msg);
            Err(Error::Message(error_msg))
        }
    }
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
        dweb_service: Mutex::new(None),
        anttp_handle: Mutex::new(None),
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

            // Make builtin names such as 'awesome' available (in addition to opening xor addresses)
            dweb::web::name::register_builtin_names(false);

            // Initialize DwebService in the app state with default config
            // The wallet will be set later when Colony initializes
            if let Some(app_state) = app.try_state::<Mutex<AppState>>() {
                let state_guard = app_state.lock().unwrap();
                *state_guard.dweb_service.lock().unwrap() =
                    Some(DwebService::new(DwebClientConfig::default()));
            }

            Ok(())
        })
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
            anttp_start,
            anttp_stop,
            anttp_open,
            open_file_with_default_app,
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app, event| match event {
            RunEvent::ExitRequested { .. } => {
                info!("Application exit requested - stopping servers");
                cleanup_anttp_server(app);
                cleanup_dweb_process(app);
            }
            RunEvent::WindowEvent {
                event: WindowEvent::CloseRequested { .. },
                ..
            } => {
                info!("Window close requested - stopping servers");
                cleanup_anttp_server(app);
                cleanup_dweb_process(app);
            }
            RunEvent::WindowEvent {
                event: WindowEvent::Destroyed,
                ..
            } => {
                info!("Window destroyed - ensuring server cleanup");
                cleanup_anttp_server(app);
                cleanup_dweb_process(app);
            }
            _ => {}
        });
}

// Helper functions that aren't tauri commands

/// Helper function to cleanup anttp server during application shutdown.
/// This function is called from various exit event handlers to ensure
/// the anttp server is properly terminated.
fn cleanup_anttp_server(app: &AppHandle) {
    if let Some(app_state) = app.try_state::<Mutex<AppState>>() {
        match stop_anttp_server(&app_state) {
            Ok(msg) => info!("Cleanup: {}", msg),
            Err(e) => error!("Failed to stop anttp server during cleanup: {}", e),
        }
    } else {
        warn!("Could not access app state during anttp cleanup");
    }
}

/// Helper function to cleanup dweb service during application shutdown.
/// This function is called from various exit event handlers to ensure
/// the dweb service is properly stopped.
fn cleanup_dweb_process(app: &AppHandle) {
    if let Some(app_state) = app.try_state::<Mutex<AppState>>() {
        match stop_dweb_service(&app_state) {
            Ok(msg) => info!("Cleanup: {}", msg),
            Err(e) => error!("Failed to stop dweb service during cleanup: {}", e),
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
            let mut config: ClientConfig = Default::default();
            if cfg!(target_os = "android") {
                // Android-specific chunk cache directory
                let app_data_dir = app.path().app_data_dir().map_err(|e| {
                    Error::Io(IoError::other(format!("Failed to get app data dir: {e}")))
                })?;

                let cache_dir = app_data_dir.join("chunk_cache");

                // Ensure the pods directory exists
                if !cache_dir.exists() {
                    std::fs::create_dir_all(&cache_dir).map_err(|e| {
                        Error::Io(IoError::other(format!(
                            "Failed to create chunk cache directory: {e}"
                        )))
                    })?;
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
            dweb_service: Mutex::new(None),
            anttp_handle: Mutex::new(None),
        })
    }

    #[test]
    fn test_stop_dweb_service_no_service() {
        // Test stopping dweb when no service is running
        let app_state = create_test_app_state();
        let result = stop_dweb_service(&app_state);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "No dweb service running");
    }

    #[test]
    fn test_stop_dweb_service_with_mock_service() {
        // This test verifies the function structure but can't test actual service stopping
        // without creating a real service, which would be complex in a unit test
        let app_state = create_test_app_state();

        // Verify that the function handles the case where no service is stored
        let result = stop_dweb_service(&app_state);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "No dweb service running");

        // Verify that the dweb_service field is properly accessed
        {
            let state = app_state.lock().unwrap();
            let service_guard = state.dweb_service.lock().unwrap();
            assert!(service_guard.is_none());
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
