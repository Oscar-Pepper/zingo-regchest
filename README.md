# Regchest
Rust-driven zcash/lightwalletd regtest server in a docker container. Developed to reduce variation between systems and to allow hosts that are unable to run lightwalletd/zcashd binaries (i.e. non-linux systems) to have access to local regtest test environments.

## Regchest utils
This repo also includes a seperate rust library which provides functions which allow the docker container to be easily managed with cargo test runners.

## Upload new version to Docker Hub
From the root directory:
 - run 'docker build -t zingodevops/regchest:{new image version number} .' to build the image locally 
 - run 'docker login' and fill in the credentials for DockerHub
 - run 'docker push zingodevops/regchest:{new image version number}' to push to DockerHub

