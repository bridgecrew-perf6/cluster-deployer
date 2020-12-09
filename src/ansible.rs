mod podspec;

use k8s_openapi::api::core::v1::{Node, Pod};
use kube::{
    api::{Api, PostParams},
};

use crate::errors::Error;
use crate::ansible::podspec::make_pod;
use crate::utils::wait_for_pod;

pub async fn apply_playbook(pods: Api<Pod>, node: &Node, image: &String, job: &String) -> Result<(), Error> {
    let node_name = node.metadata.name.as_ref().unwrap().clone();
    let pod = make_pod(&node_name, &image, &job);
    let pod_name = pod.metadata.name.as_ref().unwrap();

    pods.create(&PostParams::default(), &pod).await?;
    wait_for_pod(&pods, &pod_name).await?;

    Ok(())
}