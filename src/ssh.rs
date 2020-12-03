use std::process::Command;
use tempfile::tempdir;

use k8s_openapi::api::core::v1::Secret;
use kube::{
    api::{Api, PostParams},
    Client,
};

use crate::errors::{Error, KeyGenerationError};

const KEY_NAME: &str = "admin-key";

#[derive(Debug)]
struct KeyPair {
    private_key: String,
    public_key: String,
}

/// Generate a new SSH private/public keypair in a temporary location
fn generate_ssh_keypair() -> Result<KeyPair, KeyGenerationError> {
    let dir = tempdir()?;
    println!("Generating key in {}", dir.path().display());
    let result = Command::new("ssh-keygen")
        .current_dir(&dir)
        .arg("-t").arg("ed25519")
        .arg("-N").arg("")
        .arg("-C").arg("cluster-admin")
        .arg("-f").arg(&dir.path().join("key"))
        .output()?;

    if !result.status.success() {
        return Err(KeyGenerationError::CommandError(result));
    }

    let private_key = std::fs::read_to_string(&dir.path().join("key"))?;
    let public_key = std::fs::read_to_string(&dir.path().join("key.pub"))?;

    Ok(KeyPair {
        private_key,
        public_key,
    })
}

/// Create SSH keypair if it doesn't exist
/// Finished keypair will be stored in a k8s Secret.
pub async fn ensure_ssh_key(namespace: &str) -> Result<(), Error> {
    let client = Client::try_default().await?;
    let secret_api: Api<Secret> = Api::namespaced(client, namespace);

    match secret_api.get(KEY_NAME).await {
        Ok(_key) => {
            println!("Key {} exists", KEY_NAME);
        },
        Err(kube::Error::Api(err)) => {
          if err.reason == String::from("NotFound") {
              println!("Key {} doesn't exists, creating", KEY_NAME);
              let keypair = generate_ssh_keypair()?;

              let mut secret = Secret::default();
              let mut data = std::collections::BTreeMap::new();
              data.insert(String::from("private_key"), keypair.private_key);
              data.insert(String::from("public_key"), keypair.public_key);
              secret.string_data = Some(data);
              secret.metadata.name = Some(String::from(KEY_NAME));

              secret_api.create(&PostParams::default(), &secret).await?;
          } else {
              panic!(err);
          }
        },
        err => panic!(err),
    };
    Ok(())
}