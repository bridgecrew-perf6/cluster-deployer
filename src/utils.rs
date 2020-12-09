use futures::{StreamExt, TryStreamExt};
use k8s_openapi::api::core::v1::{Pod};
use kube::{
    api::{Api, ListParams, WatchEvent},
};

use crate::errors::{Error, PodFailure, Timeout};
pub async fn wait_for_pod(pods: &Api<Pod>, pod_name: &String) -> Result<(), Error> {
    let mut stream = pods.watch(
        &ListParams::default()
            .fields(&format!("metadata.name={}", pod_name))
            .timeout(120),
        "0"
    ).await?.boxed();

    let mut finished = false;

    while let Some(event) = stream.try_next().await? {
        match event {
            WatchEvent::Modified(pod) => {
                let status = pod.status.as_ref().unwrap().phase.as_ref().unwrap();
                println!("Pod {} changed status to {}", &pod_name, &status);
                match status.as_str() {
                    "Succeeded" => { finished = true; break },
                    "Failed" => return Err(Error::PodFailure(PodFailure { name: pod_name.clone() })),
                    _ => {},
                }
            },
            WatchEvent::Error(e) => { return Err(Error::KubeError(kube::Error::Api(e)))},
            _ => {}
        }
    }

    if !finished {
        return Err(Error::Timeout(Timeout { operation: format!("Waiting for pod {}", &pod_name)}))
    }

    Ok(())
}