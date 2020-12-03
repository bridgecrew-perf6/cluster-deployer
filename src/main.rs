mod namespace;
mod ssh;
mod errors;
mod host;

use kube::Client;

use errors::Error;
use namespace::ensure_namespace;
use ssh::ensure_ssh_key;

const NAMESPACE: &str = "cluster-manager";

async fn init(client: Client) -> Result<(), Error> {
    ensure_namespace(client.clone(), NAMESPACE).await?;
    ensure_ssh_key(client.clone(), NAMESPACE).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::try_default().await?;

    init(client.clone()).await?;
    host::Preparation::run(client.clone(), NAMESPACE.into()).await?;

    Ok(())
}
