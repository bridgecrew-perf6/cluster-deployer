//#[macro_use]
//extern crate serde_derive;

use kube::{
    api::{
        Api,
        ListParams,
        PostParams,
    },
    Client,
    error::ErrorResponse,
};

use k8s_openapi::api::{
    core::v1::{
        Namespace,
        Node,
        Secret,
    },
};

const NAMESPACE: &str = "cluster-manager";

async fn ensure_namespace(name: &str) -> Result<(), kube::Error> {
    let client = Client::try_default().await?;
    let namespace_api: Api<Namespace> = Api::all(client);

    match namespace_api.get(name).await {
        Ok(namespace) => {
            println!("Namespace {} already exists", &name);
            namespace
        },
        Err(kube::Error::Api(err)) => {
            if err.reason == String::from("NotFound") {
                println!("Namespace {} doesn't exists, creating", &name);
                let mut namespace = Namespace::default();
                namespace.metadata.name = Some(name.into());
                namespace_api.create(&PostParams::default(), &namespace).await?
            } else {
                panic!(err);
            }
        },
        err => {panic!(err)},
    };

    Ok(())
}

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
