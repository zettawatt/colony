use autonomi::Client;
use super::ColonyUI;
use slint::ComponentHandle;
use futures::future::FutureExt;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use crate::WalletData;

//pub const ENVIRONMENTS: [&str; 3] = ["local", "autonomi", "alpha"];
//pub const DEFAULT_ENVIRONMENT: &str = "alpha";

pub enum NetworkMessage {
    ClientInit,
    GetBalance,
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

    let get_balance_future = get_balance(handle.clone()).fuse();
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
            NetworkMessage::GetBalance => get_balance_future
            .set(get_balance(handle.clone()).fuse()),
            NetworkMessage::ClientInit => client_init_future.set(client_init(handle.clone()).fuse()),
        }
    }
}

async fn get_balance(handle: slint::Weak<ColonyUI>) -> tokio::io::Result<()> {
    let balance: String = "1000".to_string(); // Placeholder for actual balance fetching logic
    let _ = handle
        .clone()
        .upgrade_in_event_loop(|h| {
            h.global::<WalletData>().set_eth_balance(balance.clone().into());
            h.global::<WalletData>().set_ant_balance(balance.into());
        }).unwrap();
    Ok(())
}

async fn client_init(handle: slint::Weak<ColonyUI>) -> tokio::io::Result<Client> {
    let client = Client::init().await.unwrap();
    println!("Client initialized");
    Ok(client)
}
