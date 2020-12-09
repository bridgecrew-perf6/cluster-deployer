mod namespace;
mod errors;
mod ssh_keygen;
mod ssh_copy_id;
mod ovs;
mod ansible;
mod utils;

use kube::Client;

use errors::Error;
use namespace::ensure_namespace;
use ssh_keygen::ensure_ssh_key;

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
    ssh_copy_id::run(client.clone(), NAMESPACE.into()).await?;
    ovs::run(client.clone(), NAMESPACE.into()).await?;

    Ok(())
}
