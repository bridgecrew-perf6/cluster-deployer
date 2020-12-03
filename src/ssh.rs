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

/// Store given keypair in a secret
async fn create_secret(api: &Api<Secret>, name: &str, keypair: KeyPair) -> Result<Secret, Error> {
    let mut secret = Secret::default();
    let mut data = std::collections::BTreeMap::new();
    data.insert(String::from("private_key"), keypair.private_key);
    data.insert(String::from("public_key"), keypair.public_key);
    secret.string_data = Some(data);
    secret.metadata.name = Some(name.into());

    return api.create(&PostParams::default(), &secret).await
        .map_err(|err| Error::KubeError(err));
}

/// Generate and store SSH key in a Secret if it doesn't exit
pub async fn ensure_ssh_key(namespace: &str) -> Result<Secret, Error> {
    let client = Client::try_default().await?;
    let secret_api: Api<Secret> = Api::namespaced(client, namespace);

    match secret_api.get(KEY_NAME).await {
        Ok(key) => {
            println!("Key {} exists", KEY_NAME);
            return Ok(key);
        },
        Err(some_error) => {
            if let kube::Error::Api(kube::error::ErrorResponse {reason, ..}) = &some_error {
                if reason == &String::from("NotFound") {
                    println!("Key {} doesn't exists, creating", KEY_NAME);
                    let keypair = generate_ssh_keypair()?;
                    let _secret = create_secret(&secret_api, KEY_NAME, keypair).await?;
                }
            }
            return Err(some_error.into());
        }
    };
}