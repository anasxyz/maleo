use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

pub struct Task<A: Send + Clone + 'static> {
    pub(crate) kind: TaskKind<A>,
    pub(crate) exclusive_key: Option<u64>,
    pub(crate) timeout: Option<(Duration, A)>,
}

pub(crate) enum TaskKind<A: Send + 'static> {
    Future(Pin<Box<dyn Future<Output = A> + Send + 'static>>),
    Background(Box<dyn FnOnce() -> A + Send + 'static>),
    Delay(Duration, A),
    Every(Duration, A),
}

impl<A: Send + Clone + 'static> Task<A> {
    pub fn run(future: impl Future<Output = A> + Send + 'static) -> Self {
        Task {
            kind: TaskKind::Future(Box::pin(future)),
            exclusive_key: None,
            timeout: None,
        }
    }

    pub fn background(f: impl FnOnce() -> A + Send + 'static) -> Self {
        Task {
            kind: TaskKind::Background(Box::new(f)),
            exclusive_key: None,
            timeout: None,
        }
    }

    pub fn delay(duration: Duration, action: A) -> Self {
        Task {
            kind: TaskKind::Delay(duration, action),
            exclusive_key: None,
            timeout: None,
        }
    }

    pub fn every(duration: Duration, action: A) -> Self {
        Task {
            kind: TaskKind::Every(duration, action),
            exclusive_key: None,
            timeout: None,
        }
    }

    // only one task from this call site can run at a time
    // if one is already running it gets cancelled before this one starts
    #[track_caller]
    pub fn exclusive(mut self) -> Self {
        let location = std::panic::Location::caller();
        // hash file + line + column into a u64 key
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        location.file().hash(&mut hasher);
        location.line().hash(&mut hasher);
        location.column().hash(&mut hasher);
        self.exclusive_key = Some(hasher.finish());
        self
    }

    // if the task takes longer than duration, fire the fallback action instead
    pub fn timeout(mut self, duration: Duration, action: A) -> Self {
        self.timeout = Some((duration, action));
        self
    }

    pub(crate) fn spawn(
        self,
        send: Arc<dyn Fn(A) + Send + Sync + 'static>,
        on_abort_handle: impl FnOnce(tokio::task::AbortHandle),
    ) {
        match self.kind {
            TaskKind::Future(fut) => {
                let timeout = self.timeout;
                let handle = tokio::spawn(async move {
                    if let Some((duration, fallback)) = timeout {
                        match tokio::time::timeout(duration, fut).await {
                            Ok(action) => send(action),
                            Err(_) => send(fallback),
                        }
                    } else {
                        send(fut.await);
                    }
                })
                .abort_handle();
                on_abort_handle(handle);
            }
            TaskKind::Background(f) => {
                let send = send.clone();
                let handle = tokio::spawn(async move {
                    let action = tokio::task::spawn_blocking(f).await.unwrap();
                    send(action);
                })
                .abort_handle();
                on_abort_handle(handle);
            }
            TaskKind::Delay(duration, action) => {
                let handle = tokio::spawn(async move {
                    tokio::time::sleep(duration).await;
                    send(action);
                })
                .abort_handle();
                on_abort_handle(handle);
            }
            TaskKind::Every(duration, action) => {
                let handle = tokio::spawn(async move {
                    let mut interval = tokio::time::interval(duration);
                    interval.tick().await;
                    loop {
                        interval.tick().await;
                        send(action.clone());
                    }
                })
                .abort_handle();
                on_abort_handle(handle);
            }
        }
    }
}
