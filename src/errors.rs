use kube;
use std::convert::From;

macro_rules! generate_error {
    { $name:ident: [$($type:ident: $content:ty,)+]} => {
        #[derive(Debug)]
        pub enum $name {
            $(
                $type($content),
            )+
        }

        $(
            impl From<$content> for $name {
                fn from(err: $content) -> Self {
                    $name::$type(err)
                }
            }
        )+
    }
}

generate_error! {
    KeyGenerationError: [
        IoError: std::io::Error,
        KubeError: kube::Error,
        CommandError: std::process::Output,
    ]
}

generate_error! {
    Error: [
        KeyGenerationError: KeyGenerationError,
        KubeError: kube::Error,
        WatcherError: kube_runtime::watcher::Error,
        JoinError: tokio::task::JoinError,
        MultipleErrors: Vec<Error>,
        SerdeJsonError: serde_json::Error,
        PodFailure: PodFailure,
        Timeout: Timeout,
    ]
}

#[derive(Debug)]
pub struct PodFailure {
    pub name: String,
}

#[derive(Debug)]
pub struct Timeout {
    pub operation: String,
}