use kube;
use std::convert::From;

impl From<std::io::Error> for KeyGenerationError {
    fn from(err: std::io::Error) -> Self {
        KeyGenerationError::IoError(err)
    }
}

impl From<kube::Error> for KeyGenerationError {
    fn from(err: kube::Error) -> Self {
        KeyGenerationError::KubeError(err)
    }
}

#[derive(Debug)]
pub enum KeyGenerationError {
    IoError(std::io::Error),
    KubeError(kube::Error),
    CommandError(std::process::Output),
}

impl From<KeyGenerationError> for Error {
    fn from(err: KeyGenerationError) -> Self {
        Error::KeyGenerationError(err)
    }
}

impl From<kube::Error> for Error {
    fn from(err: kube::Error) -> Self {
        Error::KubeError(err)
    }
}

impl From<kube_runtime::watcher::Error> for Error {
    fn from(err: kube_runtime::watcher::Error) -> Self {
        Error::WatcherError(err)
    }
}

impl From<tokio::task::JoinError> for Error {
    fn from(err: tokio::task::JoinError) -> Self {
        Error::JoinError(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::SerdeJsonError(err)
    }
}

#[derive(Debug)]
pub enum Error {
    KeyGenerationError(KeyGenerationError),
    KubeError(kube::Error),
    WatcherError(kube_runtime::watcher::Error),
    JoinError(tokio::task::JoinError),
    MultipleErrors(Vec<Error>),
    SerdeJsonError(serde_json::Error),
}

