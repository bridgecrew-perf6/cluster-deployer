mod namespace;
mod ssh;
mod errors;

use errors::Error;
use namespace::ensure_namespace;
use ssh::ensure_ssh_key;

const NAMESPACE: &str = "cluster-manager";

async fn init() -> Result<(), Error> {
    ensure_namespace(NAMESPACE).await?;
    ensure_ssh_key(NAMESPACE).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    init().await?;
    Ok(())
}
