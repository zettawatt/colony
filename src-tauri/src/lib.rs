// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
//use colony::config::generate_seed_phrase;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Mutex;
use tauri::State;
use autonomi::{Client, Wallet, AddressParseError, Bytes};
use autonomi::client::{GetError, PutError};
use autonomi::client::quote::CostError;
use autonomi::client::payment::PaymentOption;
use autonomi::data::DataAddress;
use colonylib::{DataStore, KeyStore, Graph, PodManager};
use colonylib::pod::Error as PodError;
use colonylib::key::Error as KeyStoreError;
use colonylib::data::Error as DataStoreError;
use colonylib::graph::Error as GraphError;
use autonomi::client::ConnectError;
use std::fs::write;
use std::io::Error as IoError;
use std::sync::{PoisonError, MutexGuard};
use thiserror;
use tracing::{error, info};

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
    Get(#[from] GetError),
    #[error(transparent)]
    Put(#[from] PutError),
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
}

// Data structures for Tauri commands
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PodInfo {
    pub address: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePodRequest {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePodRefRequest {
    pub pod_address: String,
    pub pod_ref_address: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadFileRequest {
    pub file_path: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RefreshRefRequest {
    pub depth: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DownloadFileRequest {
    pub address: String,
    pub destination_path: String,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct SubjectDataResult {
    pub data: String,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
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
    state: State<'_, Mutex<AppState>>,
) -> Result<String, Error> {
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
fn open_keystore(
    state: State<'_, Mutex<AppState>>,
    password: String,
) -> Result<String, Error> {
    let state = state.lock().unwrap();
    let keystore_path = match state.datastore.lock().unwrap().as_ref() {
        Some(datastore) => datastore.get_keystore_path(),
        None => return Err(Error::Message("Datastore not initialized".to_string())),
    };
    let mut file = std::fs::File::open(keystore_path.clone())?;
    let keystore = KeyStore::from_file(&mut file, &password)?;
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

        let datastore = state.datastore.lock().unwrap()
            .take()
            .ok_or("DataStore not initialized")?;

        let keystore = state.keystore.lock().unwrap()
            .take()
            .ok_or("KeyStore not initialized")?;

        (datastore, keystore)
    }; // All MutexGuards are dropped here

    let key_store_file = datastore.get_keystore_path();
    let mut file = std::fs::File::create(key_store_file)?;
    let _ = KeyStore::to_file(&keystore, &mut file, &password)?;

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
fn initialize_graph(
    state: State<'_, Mutex<AppState>>,
) -> Result<String, Error> {
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
async fn initialize_pod_manager(
    state: State<'_, Mutex<AppState>>,
) -> Result<String, Error> {
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
    request: CreatePodRequest
) -> Result<PodInfo, Error> {
    // Extract all data we need and drop all locks before any await
    let (client, wallet, mut datastore, mut keystore, mut graph) = {
        let state = state.lock().unwrap();

        let client = state.client.lock().unwrap().as_ref()
            .ok_or("Client not initialized")?
            .clone();

        let wallet = state.wallet.lock().unwrap().as_ref()
            .ok_or("Wallet not initialized")?
            .clone();

        let datastore = state.datastore.lock().unwrap()
            .take()
            .ok_or("DataStore not initialized")?;

        let keystore = state.keystore.lock().unwrap()
            .take()
            .ok_or("KeyStore not initialized")?;

        let graph = state.graph.lock().unwrap()
            .take()
            .ok_or("Graph not initialized")?;

        (client, wallet, datastore, keystore, graph)
    }; // All MutexGuards are dropped here

    // Now we can safely use async operations
    let mut podman = PodManager::new(
        client,
        &wallet,
        &mut datastore,
        &mut keystore,
        &mut graph
    ).await?;

    // Use the PodManager
    let (pod_address, _) = podman.add_pod(&request.name).await?;

    // Put the components back
    {
        let state = state.lock().unwrap();
        *state.datastore.lock().unwrap() = Some(datastore);
        *state.keystore.lock().unwrap() = Some(keystore);
        *state.graph.lock().unwrap() = Some(graph);
    }

    info!("Added pod {} with address {}", &request.name, &pod_address);
    Ok(PodInfo {
        address: pod_address,
    })
}

#[tauri::command]
async fn add_pod_ref(
    state: State<'_, Mutex<AppState>>,
    request: CreatePodRefRequest
) -> Result<PodInfo, Error> {
    // Extract all data we need and drop all locks before any await
    let (client, wallet, mut datastore, mut keystore, mut graph) = {
        let state = state.lock().unwrap();

        let client = state.client.lock().unwrap().as_ref()
            .ok_or("Client not initialized")?
            .clone();

        let wallet = state.wallet.lock().unwrap().as_ref()
            .ok_or("Wallet not initialized")?
            .clone();

        let datastore = state.datastore.lock().unwrap()
            .take()
            .ok_or("DataStore not initialized")?;

        let keystore = state.keystore.lock().unwrap()
            .take()
            .ok_or("KeyStore not initialized")?;

        let graph = state.graph.lock().unwrap()
            .take()
            .ok_or("Graph not initialized")?;

        (client, wallet, datastore, keystore, graph)
    }; // All MutexGuards are dropped here

    // Now we can safely use async operations
    let mut podman = PodManager::new(
        client,
        &wallet,
        &mut datastore,
        &mut keystore,
        &mut graph
    ).await?;

    // Use the PodManager
    podman.add_pod_ref(&request.pod_address, &request.pod_ref_address).await?;

    // Put the components back
    {
        let state = state.lock().unwrap();
        *state.datastore.lock().unwrap() = Some(datastore);
        *state.keystore.lock().unwrap() = Some(keystore);
        *state.graph.lock().unwrap() = Some(graph);
    }

    info!("Added pod reference {} to pod {}", &request.pod_ref_address, &request.pod_address);
    Ok(PodInfo {
        address: request.pod_address,
    })
}

#[tauri::command]
async fn remove_pod_ref(
    state: State<'_, Mutex<AppState>>,
    request: CreatePodRefRequest
) -> Result<PodInfo, Error> {
    // Extract all data we need and drop all locks before any await
    let (client, wallet, mut datastore, mut keystore, mut graph) = {
        let state = state.lock().unwrap();

        let client = state.client.lock().unwrap().as_ref()
            .ok_or("Client not initialized")?
            .clone();

        let wallet = state.wallet.lock().unwrap().as_ref()
            .ok_or("Wallet not initialized")?
            .clone();

        let datastore = state.datastore.lock().unwrap()
            .take()
            .ok_or("DataStore not initialized")?;

        let keystore = state.keystore.lock().unwrap()
            .take()
            .ok_or("KeyStore not initialized")?;

        let graph = state.graph.lock().unwrap()
            .take()
            .ok_or("Graph not initialized")?;

        (client, wallet, datastore, keystore, graph)
    }; // All MutexGuards are dropped here

    // Now we can safely use async operations
    let mut podman = PodManager::new(
        client,
        &wallet,
        &mut datastore,
        &mut keystore,
        &mut graph
    ).await?;

    // Use the PodManager
    podman.remove_pod_ref(&request.pod_address, &request.pod_ref_address).await?;

    // Put the components back
    {
        let state = state.lock().unwrap();
        *state.datastore.lock().unwrap() = Some(datastore);
        *state.keystore.lock().unwrap() = Some(keystore);
        *state.graph.lock().unwrap() = Some(graph);
    }

    info!("Added pod reference {} to pod {}", &request.pod_ref_address, &request.pod_address);
    Ok(PodInfo {
        address: request.pod_address,
    })
}

#[tauri::command]
async fn upload_all(
    state: State<'_, Mutex<AppState>>,
) -> Result<String, Error> {
    // Extract all data we need and drop all locks before any await
    let (client, wallet, mut datastore, mut keystore, mut graph) = {
        let state = state.lock().unwrap();

        let client = state.client.lock().unwrap().as_ref()
            .ok_or("Client not initialized")?
            .clone();

        let wallet = state.wallet.lock().unwrap().as_ref()
            .ok_or("Wallet not initialized")?
            .clone();

        let datastore = state.datastore.lock().unwrap()
            .take()
            .ok_or("DataStore not initialized")?;

        let keystore = state.keystore.lock().unwrap()
            .take()
            .ok_or("KeyStore not initialized")?;

        let graph = state.graph.lock().unwrap()
            .take()
            .ok_or("Graph not initialized")?;

        (client, wallet, datastore, keystore, graph)
    }; // All MutexGuards are dropped here

    // Now we can safely use async operations
    let mut podman = PodManager::new(
        client,
        &wallet,
        &mut datastore,
        &mut keystore,
        &mut graph
    ).await?;

    // Use the PodManager
    podman.upload_all().await?;

    // Put the components back
    {
        let state = state.lock().unwrap();
        *state.datastore.lock().unwrap() = Some(datastore);
        *state.keystore.lock().unwrap() = Some(keystore);
        *state.graph.lock().unwrap() = Some(graph);
    }

    info!("Uploaded all updated pods to Autonomi");
    Ok(format!("Successfully uploaded all updated pods to Autonomi"))
}

#[tauri::command]
async fn refresh_cache(
    state: State<'_, Mutex<AppState>>,
) -> Result<String, Error> {
    // Extract all data we need and drop all locks before any await
    let (client, wallet, mut datastore, mut keystore, mut graph) = {
        let state = state.lock().unwrap();

        let client = state.client.lock().unwrap().as_ref()
            .ok_or("Client not initialized")?
            .clone();

        let wallet = state.wallet.lock().unwrap().as_ref()
            .ok_or("Wallet not initialized")?
            .clone();

        let datastore = state.datastore.lock().unwrap()
            .take()
            .ok_or("DataStore not initialized")?;

        let keystore = state.keystore.lock().unwrap()
            .take()
            .ok_or("KeyStore not initialized")?;

        let graph = state.graph.lock().unwrap()
            .take()
            .ok_or("Graph not initialized")?;

        (client, wallet, datastore, keystore, graph)
    }; // All MutexGuards are dropped here

    // Now we can safely use async operations
    let mut podman = PodManager::new(
        client,
        &wallet,
        &mut datastore,
        &mut keystore,
        &mut graph
    ).await?;

    // Use the PodManager
    podman.refresh_cache().await?;

    // Put the components back
    {
        let state = state.lock().unwrap();
        *state.datastore.lock().unwrap() = Some(datastore);
        *state.keystore.lock().unwrap() = Some(keystore);
        *state.graph.lock().unwrap() = Some(graph);
    }

    info!("Refreshed local pod cache");
    Ok(format!("Successfully efreshed local pod cache"))
}

#[tauri::command]
async fn refresh_ref(
    state: State<'_, Mutex<AppState>>,
    request: RefreshRefRequest
) -> Result<String, Error> {
    // Extract all data we need and drop all locks before any await
    let (client, wallet, mut datastore, mut keystore, mut graph) = {
        let state = state.lock().unwrap();

        let client = state.client.lock().unwrap().as_ref()
            .ok_or("Client not initialized")?
            .clone();

        let wallet = state.wallet.lock().unwrap().as_ref()
            .ok_or("Wallet not initialized")?
            .clone();

        let datastore = state.datastore.lock().unwrap()
            .take()
            .ok_or("DataStore not initialized")?;

        let keystore = state.keystore.lock().unwrap()
            .take()
            .ok_or("KeyStore not initialized")?;

        let graph = state.graph.lock().unwrap()
            .take()
            .ok_or("Graph not initialized")?;

        (client, wallet, datastore, keystore, graph)
    }; // All MutexGuards are dropped here

    // Now we can safely use async operations
    let mut podman = PodManager::new(
        client,
        &wallet,
        &mut datastore,
        &mut keystore,
        &mut graph
    ).await?;

    // Use the PodManager
    let depth: u64 = request.depth.clone().parse().map_err(|_| Error::from("Invalid depth"))?;
    podman.refresh_ref(depth).await?;

    // Put the components back
    {
        let state = state.lock().unwrap();
        *state.datastore.lock().unwrap() = Some(datastore);
        *state.keystore.lock().unwrap() = Some(keystore);
        *state.graph.lock().unwrap() = Some(graph);
    }

    info!("Refreshed all local pods and pod reference to cache to depth {}", &request.depth);
    Ok(format!("Successfully refreshed all local pods and pod reference to cache to depth {}", &request.depth))
}

#[tauri::command]
async fn search(
    state: State<'_, Mutex<AppState>>,
    request: SearchRequest
) -> Result<SearchResult, Error> {
    // Extract all data we need and drop all locks before any await
    let (client, wallet, mut datastore, mut keystore, mut graph) = {
        let state = state.lock().unwrap();

        let client = state.client.lock().unwrap().as_ref()
            .ok_or("Client not initialized")?
            .clone();

        let wallet = state.wallet.lock().unwrap().as_ref()
            .ok_or("Wallet not initialized")?
            .clone();

        let datastore = state.datastore.lock().unwrap()
            .take()
            .ok_or("DataStore not initialized")?;

        let keystore = state.keystore.lock().unwrap()
            .take()
            .ok_or("KeyStore not initialized")?;

        let graph = state.graph.lock().unwrap()
            .take()
            .ok_or("Graph not initialized")?;

        (client, wallet, datastore, keystore, graph)
    }; // All MutexGuards are dropped here

    // Now we can safely use async operations
    let mut podman = PodManager::new(
        client,
        &wallet,
        &mut datastore,
        &mut keystore,
        &mut graph
    ).await?;

    // Use the PodManager
    let search_results = podman.search(request.query.clone()).await?;

    // Put the components back
    {
        let state = state.lock().unwrap();
        *state.datastore.lock().unwrap() = Some(datastore);
        *state.keystore.lock().unwrap() = Some(keystore);
        *state.graph.lock().unwrap() = Some(graph);
    }

    info!("Search completed");
    Ok(SearchResult {
        results: search_results,
    })
}

#[tauri::command]
async fn put_subject_data(
    state: State<'_, Mutex<AppState>>,
    request: PutSubjectDataRequest
) -> Result<String, Error> {
    // Extract all data we need and drop all locks before any await
    let (client, wallet, mut datastore, mut keystore, mut graph) = {
        let state = state.lock().unwrap();

        let client = state.client.lock().unwrap().as_ref()
            .ok_or("Client not initialized")?
            .clone();

        let wallet = state.wallet.lock().unwrap().as_ref()
            .ok_or("Wallet not initialized")?
            .clone();

        let datastore = state.datastore.lock().unwrap()
            .take()
            .ok_or("DataStore not initialized")?;

        let keystore = state.keystore.lock().unwrap()
            .take()
            .ok_or("KeyStore not initialized")?;

        let graph = state.graph.lock().unwrap()
            .take()
            .ok_or("Graph not initialized")?;

        (client, wallet, datastore, keystore, graph)
    }; // All MutexGuards are dropped here

    // Now we can safely use async operations
    let mut podman = PodManager::new(
        client,
        &wallet,
        &mut datastore,
        &mut keystore,
        &mut graph
    ).await?;

    // Use the PodManager
    podman.put_subject_data(&request.pod_address, &request.subject_address, &request.data).await?;

    // Put the components back
    {
        let state = state.lock().unwrap();
        *state.datastore.lock().unwrap() = Some(datastore);
        *state.keystore.lock().unwrap() = Some(keystore);
        *state.graph.lock().unwrap() = Some(graph);
    }

    info!("Put data for subject {} in pod {}", &request.subject_address, &request.pod_address);
    Ok(format!("Successfully put data for subject {} in pod {}", &request.subject_address, &request.pod_address))
}

#[tauri::command]
async fn get_subject_data(
    state: State<'_, Mutex<AppState>>,
    request: GetSubjectDataRequest
) -> Result<SubjectDataResult, Error> {
    // Extract all data we need and drop all locks before any await
    let (client, wallet, mut datastore, mut keystore, mut graph) = {
        let state = state.lock().unwrap();

        let client = state.client.lock().unwrap().as_ref()
            .ok_or("Client not initialized")?
            .clone();

        let wallet = state.wallet.lock().unwrap().as_ref()
            .ok_or("Wallet not initialized")?
            .clone();

        let datastore = state.datastore.lock().unwrap()
            .take()
            .ok_or("DataStore not initialized")?;

        let keystore = state.keystore.lock().unwrap()
            .take()
            .ok_or("KeyStore not initialized")?;

        let graph = state.graph.lock().unwrap()
            .take()
            .ok_or("Graph not initialized")?;

        (client, wallet, datastore, keystore, graph)
    }; // All MutexGuards are dropped here

    // Now we can safely use async operations
    let mut podman = PodManager::new(
        client,
        &wallet,
        &mut datastore,
        &mut keystore,
        &mut graph
    ).await?;

    // Use the PodManager
    let subject_data = podman.get_subject_data(&request.subject_address).await?;

    // Put the components back
    {
        let state = state.lock().unwrap();
        *state.datastore.lock().unwrap() = Some(datastore);
        *state.keystore.lock().unwrap() = Some(keystore);
        *state.graph.lock().unwrap() = Some(graph);
    }

    info!("Retrieved data for subject {}", &request.subject_address);
    Ok(SubjectDataResult {
        data: subject_data,
    })
}

////////////////////////////////////////////////////////////////////
// Autonomi commands
////////////////////////////////////////////////////////////////////

#[tauri::command]
async fn initialize_autonomi_client(
    state: State<'_, Mutex<AppState>>,
    wallet_key: String,
) -> Result<String, Error> {
    //FIXME: do we want to hard code this or have an argument to set this in the frontend?
    let environment = "local";
    let client = init_client(environment).await?;
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

        let client = state.client.lock().unwrap().as_ref()
            .ok_or("Client not initialized")?
            .clone();

        client
    }; // All MutexGuards are dropped here

    let data = std::fs::read(request.file_path.clone())?;
    let data = Bytes::from(data);

    let cost = client.data_cost(data).await?;
    info!("File {} is estimated to cost {} to upload", request.file_path, cost.to_string());    

    Ok(format!("File {} is estimated to cost {} to upload", request.file_path, cost.to_string()))
}

#[tauri::command]
async fn upload_data(
    state: State<'_, Mutex<AppState>>,
    request: UploadFileRequest,
) -> Result<String, Error> {
    let (client, wallet) = {
        let state = state.lock().unwrap();

        let client = state.client.lock().unwrap().as_ref()
            .ok_or("Client not initialized")?
            .clone();

        let wallet = state.wallet.lock().unwrap().as_ref()
            .ok_or("Wallet not initialized")?
            .clone();

        (client, wallet)
    }; // All MutexGuards are dropped here

    let data = std::fs::read(request.file_path.clone())?;
    let data = Bytes::from(data);

    let payment = PaymentOption::Wallet(wallet);
    let (cost, data_addr) = client.data_put_public(data, payment).await?;

    Ok(format!("File {} uploaded to address {} for {}", request.file_path, data_addr, cost.to_string()))
}

#[tauri::command]
async fn download_data(
    state: State<'_, Mutex<AppState>>,
    request: DownloadFileRequest,
) -> Result<String, Error> {
    // Extract all data we need and drop all locks before any await
    let client = {
        let state = state.lock().unwrap();

        let client = state.client.lock().unwrap().as_ref()
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
    Ok(format!("File downloaded from {} to {}",
              request.address, request.destination_path))
}

////////////////////////////////////////////////////////////////////
// Tauri App
////////////////////////////////////////////////////////////////////

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_state = AppState {
        client: Mutex::new(None),
        wallet: Mutex::new(None),
        datastore: Mutex::new(None),
        keystore: Mutex::new(None),
        graph: Mutex::new(None),
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_opener::init())
        .manage(Mutex::new(app_state))
        .invoke_handler(tauri::generate_handler![
            greet,
            get_new_seed_phrase,
            initialize_pod_manager,
            add_pod,
            add_pod_ref,
            remove_pod_ref,
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
            download_data
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
