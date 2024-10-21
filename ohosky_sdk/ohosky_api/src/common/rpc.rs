use super::share::ISharableValue;

pub trait ISharedRpcService<T: ISharableValue, R: ISharableValue> {
    fn register(name: &str) -> Self;
    fn recv(&self) -> impl std::future::Future<Output = anyhow::Result<T>>;
    fn recv_blocking(&self) -> anyhow::Result<T>;
    fn try_recv(&self) -> anyhow::Result<Option<T>>;
    fn reply(&self, ret: R) -> anyhow::Result<()>;
}

pub trait ISharedRpcClient<T: ISharableValue, R: ISharableValue> {
    fn new(name: &str) -> Self;
    fn is_online(&self) -> anyhow::Result<bool>;
    fn wait_online(&self) -> impl std::future::Future<Output = anyhow::Result<()>>;
    fn call(&self, arg: T) -> impl std::future::Future<Output = anyhow::Result<R>>;
    fn call_blocking(&self, arg: T) -> anyhow::Result<R>;
}
