use k8s_openapi::api::core::v1::Namespace;
use kube::{
    api::{
        Api,
        PostParams,
    },
    Client,
};
use crate::errors::Error;

pub async fn ensure_namespace(name: &str) -> Result<(), Error> {
    let client = Client::try_default().await?;
    let namespace_api: Api<Namespace> = Api::all(client);

    match namespace_api.get(name).await {
        Ok(namespace) => {
            println!("Namespace {} already exists", &name);
            Ok(namespace)
        },
        Err(kube::Error::Api(err)) => {
            if err.reason == String::from("NoFound") {
                println!("Namespace {} doesn't exists, creating", &name);
                let mut namespace = Namespace::default();
                namespace.metadata.name = Some(name.into());
                namespace_api.create(&PostParams::default(), &namespace)
                    .await
                    .map_err(|err| Error::KubeError(err))
            } else {
                Err(Error::KubeError(kube::Error::Api(err)))
            }
        },
        Err(err) => Err(err.into()),
    }?;

    Ok(())
}