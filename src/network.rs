use super::ColonyUI;
use slint::ComponentHandle;
use futures::future::{Fuse, FusedFuture, FutureExt};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

pub enum NetworkMessage {
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
    futures::pin_mut!(
        get_balance_future,
    );
    loop {
        let m = futures::select! {
            _ = get_balance_future => {
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
            NetworkMessage::GetBalance => return Ok(()),
        }
    }
}

async fn get_balance(handle: slint::Weak<ColonyUI>) {

}

