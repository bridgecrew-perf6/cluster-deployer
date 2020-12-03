//use futures::{StreamExt, TryStreamExt};
use k8s_openapi::api::core::v1::Node;
use kube::{
    api::{Api, ListParams},
    Client
};
//use kube_runtime::{utils::try_flatten_applied, watcher};
use crate::errors::Error;
use std::thread;

pub struct Preparation {}

const ANNOTATION_SSH: &str = "cluster-manager/ssh";

async fn prepare_host(node: Node) {
    let name = node.metadata.name.unwrap();
    println!("Working on {}", name);

    if !node.metadata.annotations.unwrap().contains_key(ANNOTATION_SSH) {
        println!("{}: No SSH access prepared", name);
    }

}

impl Preparation {
    pub async fn run(client: Client) -> Result<(), Error> {
        let nodes: Api<Node> = Api::all(client.clone());

        for node in nodes.list(&ListParams::default()).await? {
            tokio::spawn(async {
                prepare_host(node).await;
            });
        }

        Ok(())
    }
}