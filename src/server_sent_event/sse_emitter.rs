use std::{sync::Arc, time::Duration};
use std::collections::HashMap;

use actix_web_lab::{
    sse::{self, Sse},
    util::InfallibleStream,
};
use parking_lot::Mutex;
use tokio::sync::mpsc;
use tokio::time::interval;
use tokio_stream::wrappers::ReceiverStream;

use crate::common::response::BaseResponse;

#[derive(Debug)]
pub struct SseBroadcaster {
    inner: Mutex<SseBroadcasterInner>,
}

#[derive(Debug, Clone, Default)]
pub struct SseBroadcasterInner {
    clients: HashMap<String, mpsc::Sender<sse::Event>>,
}

impl SseBroadcaster {
    pub fn create() -> Arc<Self> {
        let this = Arc::new(SseBroadcaster {
            inner: Mutex::new(SseBroadcasterInner::default())
        });
        SseBroadcaster::spawn_ping(Arc::clone(&this));
        this
    }

    /// Pings clients every 10 seconds to see if they are alive and remove them from the broadcast
    /// list if not.
    fn spawn_ping(this: Arc<Self>) {
        actix_web::rt::spawn(async move {
            let mut interval = interval(Duration::from_secs(30));

            loop {
                interval.tick().await;
                this.remove_stale_clients().await;
            }
        });
    }

    /// Removes all non-responsive clients from broadcast list.
    async fn remove_stale_clients(&self) {
        let clients = self
            .inner.lock().clients.clone();

        let mut ok_clients = HashMap::new();

        for (key, client) in clients {
            if client
                .send(sse::Event::Comment("ping".into()))
                .await
                .is_ok()
            {
                ok_clients.insert(key, client.clone());
            }
        }

        self.inner.lock().clients = ok_clients;
    }
    /// Registers client with broadcaster, returning an SSE response body.
    pub async fn new_client(&self, key: &str) -> Sse<InfallibleStream<ReceiverStream<sse::Event>>> {
        let (tx, rx) = mpsc::channel(10);

        tx.send(sse::Data::new("connected").into()).await.unwrap();

        self.inner.lock().clients.insert(key.to_string(), tx);

        Sse::from_infallible_receiver(rx)
    }
    /// Broadcasts `msg` to all clients.
    pub async fn broadcast(&self, topic: &str, msg: &str) {
        let clients = self.inner.lock().clients.clone();
        for (_, value) in clients {
            let _ = value
                .send(sse::Event::Data(
                    sse::Data::new_json(
                        BaseResponse::success(200, Some(msg), "ini message".to_string())
                    ).unwrap().event(topic))
                ).await;
        }
    }

    pub async fn send_to(&self,topic:&str, to: String, msg: &str) {
        let clients = self.inner.lock().clients.clone();
        let _ = clients.get(to.as_str())
            .unwrap()
            .send(
                sse::Event::Data(
                    sse::Data::new_json(
                        BaseResponse::success(200, Some(msg), "ini message".to_string())
                    ).unwrap().event(topic)
                )
            )
            .await;
    }
}