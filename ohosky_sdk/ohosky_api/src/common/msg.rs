use super::share::ISharableValue;

pub trait ISharedMessageChannel<T: ISharableValue> {
    fn new(name: &str) -> Self;
    fn send(&self, value: T) -> impl std::future::Future<Output = anyhow::Result<()>>;
    fn send_blocking(&self, value: T) -> anyhow::Result<()>;
    fn recv(&self) -> impl std::future::Future<Output = anyhow::Result<T>>;
    fn recv_blocking(&self) -> anyhow::Result<T>;
    fn try_recv(&self) -> anyhow::Result<Option<T>>;
}

pub trait ISharedWatchValue<T: ISharableValue>: Sized {
    fn new(name: &str, watcher_unique: &str) -> (Self, impl ISharedValueWatcher<T>);
    fn update(&self, value: T) -> anyhow::Result<()>;
}

pub trait ISharedValueWatcher<T: ISharableValue> {
    fn fetch(&self) -> T;
    fn next_value(&self) -> impl std::future::Future<Output = T>;
    fn try_next_value(&self) -> Option<T>;
}
