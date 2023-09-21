use bollard::container::{
    Config, CreateContainerOptions, LogsOptions, RemoveContainerOptions, StartContainerOptions,
};
use bollard::models::HostConfig;
use bollard::{Docker, API_DEFAULT_VERSION};
use futures::StreamExt;
use std::default::Default;

pub async fn launch(unix_socket: Option<&str>) -> Result<Docker, bollard::errors::Error> {
    let docker = match unix_socket {
        Some(socket) => Docker::connect_with_unix(socket, 600, API_DEFAULT_VERSION)?,
        None => Docker::connect_with_local_defaults()?,
    };

    let container_options = Some(CreateContainerOptions {
        name: "regchest",
        ..Default::default()
    });
    // TODO: Upgrade to published ports. Using host network mode has security concerns due to allowing the container unnecessary privileges. Should currently only be used in cases where the host is not put at risk such as VMs i.e. Github ephemeral runners.
    let host_config = HostConfig {
        network_mode: Some(String::from("host")),
        ..Default::default()
    };
    let container_config = Config {
        image: Some("zingodevops/regchest:002"),
        host_config: Some(host_config),
        ..Default::default()
    };
    let logs_options = Some(LogsOptions::<String> {
        stdout: true,
        follow: true,
        ..Default::default()
    });

    docker
        .create_container(container_options, container_config)
        .await?;
    docker
        .start_container("regchest", None::<StartContainerOptions<String>>)
        .await?;

    let mut stream = docker.logs("regchest", logs_options);
    while let Some(result) = stream.next().await {
        match result {
            Ok(message) => {
                let m = &message.into_bytes();
                let s = std::str::from_utf8(m).unwrap();
                if s.contains("Successfully launched") {
                    break;
                }
            }
            Err(e) => return Err(e),
        }
    }

    Ok(docker)
}

pub async fn close(docker: Docker) -> Result<(), bollard::errors::Error> {
    let remove_options = Some(RemoveContainerOptions {
        force: true,
        ..Default::default()
    });

    docker.remove_container("regchest", remove_options).await?;

    Ok(())
}
