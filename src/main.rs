mod errors;
mod ansible;
mod utils;
mod host_tasks;
mod cluster_tasks;

use kube::Client;

use errors::Error;
use cluster_tasks::{namespace::ensure_namespace, ssh_keygen::ensure_ssh_key};
use host_tasks::{ssh_copy_id, ovs, libvirt};

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
    libvirt::run(client.clone(), NAMESPACE.into()).await?;

    Ok(())
}
