use autonomi::Client;
use super::ColonyUI;
use slint::ComponentHandle;
use futures::future::FutureExt;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use crate::{WalletData, BAD_MNEMONIC};
use crate::data::SecretData;
use ruint::Uint;

pub enum NetworkMessage {
    ClientInit,
    GetBalance {
        secret_data: SecretData,
    },
    Quit,
}

pub struct NetworkWorker {
    pub channel: UnboundedSender<NetworkMessage>,
    worker_thread: std::thread::JoinHandle<()>,
}

impl NetworkWorker {
    pub fn new(ui: &ColonyUI) -> Self {
        let (channel, r) = tokio::sync::mpsc::unbounded_channel();
        let worker_thread = std::thread::spawn({
            let handle_weak = ui.as_weak();
            move || {
                tokio::runtime::Runtime::new()
                    .unwrap()
                    .block_on(network_worker_loop(r, handle_weak))
                    .unwrap()
            }
        });
        Self {
            channel,
            worker_thread,
        }
    }

    pub fn join(self) -> std::thread::Result<()> {
        let _ = self.channel.send(NetworkMessage::Quit);
        self.worker_thread.join()
    }
}

async fn network_worker_loop(
    mut r: UnboundedReceiver<NetworkMessage>,
    handle: slint::Weak<ColonyUI>,
) -> tokio::io::Result<()> {
    let mut secret_data = SecretData::from_mnemonic(BAD_MNEMONIC.to_string()).unwrap();

    let get_balance_future = get_balance(secret_data.clone(), handle.clone()).fuse();
    let client_init_future = client_init(handle.clone()).fuse();
    futures::pin_mut!(
        client_init_future,
        get_balance_future,
    );
    loop {
        let m = futures::select! {
            res = get_balance_future => {
                res?;
                continue;
            }
            res = client_init_future => {
                res?;
                continue;
            }
            m = r.recv().fuse() => {
                match m {
                    None => return Ok(()),
                    Some(m) => m,
                }
            }
        };

        match m {
            NetworkMessage::Quit => return Ok(()),
            NetworkMessage::GetBalance {secret_data}=> get_balance_future
            .set(get_balance(secret_data, handle.clone()).fuse()),
            NetworkMessage::ClientInit => client_init_future.set(client_init(handle.clone()).fuse()),
        }
    }
}

async fn get_balance(secret_data: SecretData, handle: slint::Weak<ColonyUI>) -> tokio::io::Result<()> {
    let _ = handle
        .clone()
        .upgrade_in_event_loop(|h| {
            h.global::<WalletData>().set_eth_balance("Loading...".into());
            h.global::<WalletData>().set_ant_balance("Loading...".into());
        }).unwrap();
    let wallet = secret_data.get_wallet();
    let eth_balance: Uint<256, 4> = wallet.clone().balance_of_gas_tokens().await.unwrap();
    let eth_balance: f64 = eth_balance.try_into().unwrap_or(0f64);
    let eth_balance: f64 = eth_balance / 1_000_000_000_000_000_000.0f64;
    let ant_balance: Uint<256, 4> = wallet.balance_of_tokens().await.unwrap();
    let ant_balance: f64 = ant_balance.try_into().unwrap_or(0f64);
    let ant_balance: f64 = ant_balance / 1_000_000_000_000_000_000.0f64;
    // convert f64 to String
    let eth_balance_str = eth_balance.to_string() + " ETH";
    let ant_balance_str = ant_balance.to_string() + " ANT";
    let _ = handle
        .clone()
        .upgrade_in_event_loop(|h| {
            h.global::<WalletData>().set_eth_balance(eth_balance_str.into());
            h.global::<WalletData>().set_ant_balance(ant_balance_str.into());
        }).unwrap();
    Ok(())
}

async fn client_init(handle: slint::Weak<ColonyUI>) -> tokio::io::Result<Client> {
    let client = Client::init().await.unwrap();
    println!("Client initialized");
    Ok(client)
}
