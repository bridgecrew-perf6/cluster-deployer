use kube::{
    api::{
        Api,
        PostParams,
    },
    Client,
};

use k8s_openapi::api::{
    core::v1::{
        Namespace,
    },
};

pub async fn ensure_namespace(name: &str) -> Result<(), kube::Error> {
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