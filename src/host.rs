use futures::{StreamExt, TryStreamExt};
use k8s_openapi::api::core::v1::{Node, Pod, PodSpec, Container, VolumeMount, Volume, HostPathVolumeSource, SecretVolumeSource};
use kube::{
    api::{Api, ListParams, PostParams},
    Client
};
use crate::errors::Error;
use std::collections::BTreeMap;
use kube::api::WatchEvent;

pub struct Preparation {}

const ANNOTATION_SSH: &str = "cluster-manager/ssh";

async fn wait_for_pod(pods: &Api<Pod>, node_name: &String, pod_name: &String) -> Result<(), Error> {
    let mut stream = pods.watch(
        &ListParams::default()
            .fields(&format!("metadata.name={}", pod_name))
            .timeout(30),
        "0"
    ).await?.boxed();

    while let Some(event) = stream.try_next().await? {
        match event {
            WatchEvent::Modified(pod) => {
                let status = pod.status.as_ref().unwrap().phase.as_ref().unwrap();
                println!("{}: Pod {} changed status to {}", &node_name, &pod_name, &status);
                if status == "Succeeded" {
                    break;
                }
            },
            WatchEvent::Error(e) => { return Err(Error::KubeError(kube::Error::Api(e)))},
            _ => {}
        }
    }
    Ok(())
}

async fn add_ssh_key(client: Client, namespace: String, node: &Node) -> Result<(), Error> {
    let pods: Api<Pod> = Api::namespaced(client.clone(), namespace.as_ref());
    let node_name = node.metadata.name.as_ref().unwrap().clone();

    let mut pod = Pod {
        spec: Some(PodSpec {
            node_name: Some(node_name.clone()),
            containers: vec![
                Container {
                    image: Some(String::from("busybox")),
                    name: "copy".into(),
                    command: Some(vec!["/bin/sh".into()]),
                    args: Some(vec![
                        "-c".into(),
                        "cat /mnt/manager-public-key #>> /mnt/authorized_keys".into()
                    ]),
                    volume_mounts: Some(vec![
                        VolumeMount {
                            name: "public-key".into(),
                            read_only: Some(true),
                            mount_path: "/mnt/manager-public-key".into(),
                            sub_path: Some("public_key".into()),
                            .. VolumeMount::default()
                        },
                        VolumeMount {
                            name: "authorized-keys".into(),
                            read_only: Some(false),
                            mount_path: "/mnt/authorized_keys".into(),
                            .. VolumeMount::default()
                        }
                    ]),
                    .. Container::default()
                }
            ],
            volumes: Some(vec![
                Volume {
                    name: "public-key".into(),
                    secret: Some(SecretVolumeSource {
                        secret_name: Some("admin-key".into()),
                        .. SecretVolumeSource::default()
                    }),
                    .. Volume::default()
                },
                Volume {
                    name: "authorized-keys".into(),
                    host_path: Some(HostPathVolumeSource {
                        path: "/root/.ssh/authorized_keys".into(),
                        type_: Some("File".into())
                    }),
                    .. Volume::default()
                }
            ]),
            restart_policy: Some("OnFailure".into()),
            .. PodSpec::default()
        }),
        .. Pod::default()
    };
    let pod_name = format!("add-ssh-key-{}", &node_name);

    pod.metadata.name = Some(pod_name.clone());
    pod.metadata.labels = Some(BTreeMap::new());
    pod.metadata.labels.as_mut().unwrap().insert("app".into(), "cluster-manager".into());
    pod.metadata.labels.as_mut().unwrap().insert("task".into(), "copy-ssh-key".into());

    pods.create(&PostParams::default(), &pod).await?;
    wait_for_pod(&pods, &node_name, &pod_name).await?;
    Ok(())
}

async fn prepare_host(client: Client, namespace: String, node: Node) -> Result<(), Error>{
    let name = node.metadata.name.as_ref().unwrap();
    println!("Working on {}", name);

    let log = |msg: &str| {
        println!("{}: {}", name, msg);
    };

    if node.metadata.annotations.as_ref().unwrap().contains_key(ANNOTATION_SSH) {
        return Ok(());
    }

    log("No SSH access prepared (no annotation)");
    add_ssh_key(client.clone(), namespace, &node).await?;

    Ok(())
}

impl Preparation {
    pub async fn run(client: Client, namespace: String) -> Result<(), Error> {
        let nodes: Api<Node> = Api::all(client.clone());

        let mut tasks = Vec::new();
        for node in nodes.list(&ListParams::default()).await? {
            let c = client.clone();
            let n = namespace.clone();
            tasks.push(tokio::spawn(async move {
                prepare_host(c, n, node).await?;
                Ok::<(), Error>(())
            }));
        }

        let mut errors = Vec::new();
        for result in futures::future::join_all(tasks).await {
            match result {
                // Unhandled error
                Err(e) => panic!(e),
                // Task completed, with or without errors
                Ok(result) => match result {
                    Ok(_) => println!("Task completed succesfully"),
                    Err(e) => errors.push(e),
                }
            }
        }

        if errors.len() > 0 {
            Err(Error::MultipleErrors(errors))
        } else {
            Ok(())
        }
    }
}