use std::collections::BTreeMap;
use k8s_openapi::api::core::v1::{
    Container,
    EnvVar,
    Pod,
    PodSpec,
    SecretVolumeSource,
    Volume,
    VolumeMount,
};

pub fn make_pod(node_name: &String, image: &String, job_name: &String) -> Pod {
    let mut pod = Pod {
        spec: Some(PodSpec {
            node_name: Some(node_name.clone()),
            containers: vec![
                Container {
                    image: Some(image.clone()),
                    name: String::from("ansible"),
                    env: Some(vec![
                        EnvVar {
                            name: "ANSIBLE_HOST_KEY_CHECKING".into(),
                            value: Some("False".into()),
                            value_from: None,
                        }
                    ]),
                    command: Some(vec!["/usr/bin/ansible-playbook".into()]),
                    args: Some(vec![
                        "-i".into(),
                        "inventory".into(),
                        //"--check".into(),
                        "playbook.yaml".into(),
                    ]),
                    working_dir: Some(String::from("/work")),
                    volume_mounts: Some(vec![
                        VolumeMount {
                            name: "ssh-key".into(),
                            read_only: Some(true),
                            mount_path: "/root/.ssh/id_rsa".into(),
                            sub_path: Some("private_key".into()),

                            .. VolumeMount::default()
                        }
                    ]),
                    .. Container::default()
                }
            ],
            host_network: Some(true),
            volumes: Some(vec![
                Volume {
                    name: "ssh-key".into(),
                    secret: Some(SecretVolumeSource {
                        secret_name: Some("admin-key".into()),
                        default_mode: Some(0o400),
                        .. SecretVolumeSource::default()
                    }),
                    .. Volume::default()
                }
            ]),
            restart_policy: Some("Never".into()),
            .. PodSpec::default()
        }),
        .. Pod::default()
    };
    let pod_name = format!("ansible-{}-{}", &node_name, &job_name);

    pod.metadata.name = Some(pod_name.clone());
    pod.metadata.labels = Some(BTreeMap::new());
    pod.metadata.labels.as_mut().unwrap().insert("app".into(), "cluster-manager".into());
    pod.metadata.labels.as_mut().unwrap().insert("task".into(), job_name.into());

    pod
}