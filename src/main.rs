//#[macro_use]
//extern crate serde_derive;

mod namespace;

use namespace::ensure_namespace;

/*use kube::{
    api::{
        Api,
        ListParams,
        PostParams,
    },
    Client,
    error::ErrorResponse,
};*/

/*use k8s_openapi::api::{
    core::v1::{
        Namespace,
        Node,
        Secret,
    },
};*/

const NAMESPACE: &str = "cluster-manager";



async fn init() -> Result<(), kube::Error> {
    ensure_namespace(NAMESPACE).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), kube::Error> {
    init().await?;
    //let kubeconfig = config::load_kube_config().expect("kubeconfig failed to load");
    //let client = Client::try_default().await?;


    /*let api_nodes: Api<Node> = Api::all(client);
    let nodes = api_nodes.list(&ListParams::default()).await?;

    for node in nodes {
        dbg!(node);
    }*/

    //let namespace = "default";
    Ok(())
}
