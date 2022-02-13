# About

This is a simple **workshop** to work on :
- Actix.rs
- Docker

**No PRODUCTION purpose**

This is a web server with a single `POST` endpoint. Given an `input` of array (and
an optional `max` value), it returns all permutations of the given array.

WARNING, for an array of 10 elements, there is `3 628 800` uniq permutation possible. Think of building everything with the `--release` flag if you want to
play with lots of permutations.

# Docker

Docker knowledge is a requirement.

My requirement was to build the app **outside** of a container : into the host computer. That is the reason why I don't start FROM a `rust` container.

We run an Alpine Linux docker container. We should build the Rust program for
the target `x86_64-unknown-linux-musl`. The script `build_for_docker.sh` do
this job.

- Build an image `docker build -t permutation_web_server .`
- Run the image `docker run -dp 8080:8080 permutation_web_server`
- Test with the script `test.sh`
- Stop the docker container with `docker ps` to get the `container-id`  and `docker stop <container id>`

# Permutation project

See https://github.com/corentinway/permutation_way_rs
