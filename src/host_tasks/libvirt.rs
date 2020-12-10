use serde_json::json;
use k8s_openapi::api::core::v1::{Node, Pod};
use kube::{
    api::{Api, ListParams, PatchParams},
    Client
};

use crate::errors::Error;
use crate::ansible::apply_playbook;

const RUNNER_IMAGE: &str = "registry.acl.fi/public/libvirt-install";
const RUNNER_TAG: &str = "sha256:3018d6b716e843b9599aad1f446642252a9b7a2cbe717da91b54045205cd9d8f";
const OVS_ANNOTATION: &str = "cluster-manager/libvirt";


pub async fn run(client: Client, namespace: String) -> Result<(), Error> {
    let nodes: Api<Node> = Api::all(client.clone());

    for node in nodes.list(&ListParams::default()).await? {
        let annotations = node.metadata.annotations.as_ref().unwrap();
        if let Some(value) = annotations.get(OVS_ANNOTATION) {
            if value == &String::from(RUNNER_TAG) {
                println!("{}: Skipping libvirt-install due to tag match", node.metadata.name.as_ref().unwrap());
                continue;
            }
        }

        println!("{}: Starting libvirt-install playbook", node.metadata.name.as_ref().unwrap());
        let pods: Api<Pod> = Api::namespaced(client.clone(),namespace.as_str());
        apply_playbook(pods, &node,
                       &format!("{}@{}", &RUNNER_IMAGE, &RUNNER_TAG),
                       &"libvirt-install".into()
        ).await?;

        println!("{}: libvirt-install playbook success, setting tag", node.metadata.name.as_ref().unwrap());

        nodes.patch(
            &node.metadata.name.as_ref().unwrap().clone(),
            &PatchParams::default(),
            serde_json::to_vec(
                &json!({ "metadata": { "annotations": { OVS_ANNOTATION: RUNNER_TAG } } })
            )?
        ).await?;
    }

    Ok(())
}