use std::collections::BTreeMap;
use k8s_openapi::api::core::v1::{
    Container,
    HostPathVolumeSource,
    Pod,
    PodSpec,
    SecretVolumeSource,
    Volume,
    VolumeMount,
};

pub fn make_pod(node_name: &String) -> Pod {
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

    pod
}