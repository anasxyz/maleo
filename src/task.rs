use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

// a task is a description of background work that produces an action when done
pub enum Task<A: Send + 'static> {
    // run an async future
    Future(Pin<Box<dyn Future<Output = A> + Send + 'static>>),
    // run a blocking closure on the thread pool
    Background(Box<dyn FnOnce() -> A + Send + 'static>),
    // fire an action once after a delay
    Delay(Duration, A),
    // fire an action repeatedly on an interval
    Every(Duration, A),
}

impl<A: Send + Clone + 'static> Task<A> {
    pub fn run(future: impl Future<Output = A> + Send + 'static) -> Self {
        Task::Future(Box::pin(future))
    }

    pub fn background(f: impl FnOnce() -> A + Send + 'static) -> Self {
        Task::Background(Box::new(f))
    }

    pub fn delay(duration: Duration, action: A) -> Self {
        Task::Delay(duration, action)
    }

    pub fn every(duration: Duration, action: A) -> Self {
        Task::Every(duration, action)
    }

    // spawn the task
    // send is a closure that delivers the action and wakes winit
    pub(crate) fn spawn(self, send: impl Fn(A) + Send + 'static) {
        match self {
            Task::Future(fut) => {
                tokio::spawn(async move {
                    let action = fut.await;
                    send(action);
                });
            }
            Task::Background(f) => {
                tokio::task::spawn_blocking(move || {
                    let action = f();
                    send(action);
                });
            }
            Task::Delay(duration, action) => {
                tokio::spawn(async move {
                    tokio::time::sleep(duration).await;
                    send(action);
                });
            }
            Task::Every(duration, action) => {
                tokio::spawn(async move {
                    let mut interval = tokio::time::interval(duration);
                     // skip first tick
                    interval.tick().await;
                    loop {
                        interval.tick().await;
                        send(action.clone());
                    }
                });
            }
        }
    }
}
