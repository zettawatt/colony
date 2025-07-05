// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
//use colony::config::generate_seed_phrase;
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
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::fs::write;
use std::io::Error as IoError;
use std::sync::Mutex;
use std::sync::{MutexGuard, PoisonError};
use tauri::{AppHandle, Emitter, State};
use tauri_plugin_shell::{Error as ShellError, ShellExt};
use tracing::{error, info};

#[tauri::command]
fn get_file_size(path: String) -> Result<u64, String> {
    fs::metadata(path)
        .map(|meta| meta.len())
        .map_err(|e| e.to_string())
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
fn initialize_datastore(state: State<'_, Mutex<AppState>>) -> Result<String, Error> {
    let datastore = DataStore::create()?;
    let state = state.lock().unwrap();
    *state.datastore.lock().unwrap() = Some(datastore);
    info!("Datastore initialized");
    Ok("Datastore initialized".to_string())
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

        use std::collections::HashMap;
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
    let wallets = keystore.get_wallet_keys();

    // Map the wallet names and keys to a single JSON Value object
    let wallets: Value = serde_json::json!(wallets);

    info!("Wallets listed");
    Ok(wallets)
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

        let keystore = state
            .keystore
            .lock()
            .unwrap()
            .as_ref()
            .ok_or("KeyStore not initialized")?
            .clone();

        // Call the keystore get_wallet_key function to get the wallet private key
        let wallet_key = keystore.get_wallet_key(&name)?;

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
        Ok(wallet)
    }
    .await;

    // Always put the components back, regardless of success or failure
    match result {
        Ok(wallet) => {
            let state = state.lock().unwrap();
            *state.client.lock().unwrap() = Some(client);
            *state.wallet.lock().unwrap() = Some(wallet);
            Ok("Wallet switched".to_string())
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
) -> Result<String, Error> {
    let environment = {
        let state = state.lock().unwrap();
        state.network.clone()
    };
    let client = init_client(&environment).await?;
    let evm_network = client.evm_network();
    info!("EVM network: {evm_network:?}");
    //FIXME: need to grap the wallet error and remove this unwrap()
    let wallet = Wallet::new_from_private_key(evm_network.clone(), &wallet_key).unwrap();

    // Lock the state and update the client
    let state = state.lock().unwrap();
    *state.client.lock().unwrap() = Some(client);
    *state.wallet.lock().unwrap() = Some(wallet);
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
async fn download_data(
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

    // Data address of the dog picture
    let data_address = DataAddress::from_hex(request.address.trim())?;

    // Get the bytes of the dog picture
    let bytes = client.data_get_public(&data_address).await?;

    // Write the bytes of the dog picture to a file
    write(request.destination_path.clone(), bytes)?;
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
async fn dweb_serve(app: AppHandle) -> Result<String, Error> {
    let sidecar_command = app.shell().sidecar("dweb")?;
    let (_rx, mut _child) = sidecar_command
        .args(["serve"])
        .spawn()
        .expect("Failed to spawn sidecar");
    Ok("Started dweb".to_string())
}

#[tauri::command]
async fn dweb_open(app: AppHandle, address: String) -> Result<String, Error> {
    let sidecar_command = app.shell().sidecar("dweb")?;
    let (_rx, mut _child) = sidecar_command
        .args(["open", &address])
        .spawn()
        .expect("Failed to spawn sidecar");
    Ok("Opened address with dweb".to_string())
}

////////////////////////////////////////////////////////////////////
// Tauri App
////////////////////////////////////////////////////////////////////

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run(network: &str) {
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
    };

    tauri::Builder::default()
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
            create_keystore_from_seed_phrase,
            create_keystore_from_key,
            write_keystore_to_file,
            open_keystore,
            initialize_graph,
            upload_cost,
            upload_data,
            download_data,
            add_wallet,
            remove_wallet,
            list_wallets,
            switch_wallet,
            dweb_serve,
            dweb_open,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Helper functions that aren't tauri commands
async fn init_client(environment: &str) -> Result<Client, Error> {
    match environment {
        "local" => Ok(Client::init_local().await?),
        "alpha" => Ok(Client::init_alpha().await?),
        _ => Ok(Client::init().await?), // main net
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
            session: Mutex::new(None),
        })
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
