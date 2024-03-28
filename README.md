# Zero Knowledge Authentication Proof (ZK-AUTH)

## Local Run

Install Rust and protobuf-compiler

```bash
sudo dnf install protobuf-compiler
```

## Docker

Run the program using Docker

Build the container

```
$ docker-compose build zk-auth-server
```

Run the container

```
$ docker-compose run --rm zk-auth-server
```

Run the Server

```
root@###:/zk-auth-server# cargo run --bin server --release
```

Open new terminal

Connect to the container and list out the running containers

```
$ docker container ls
CONTAINER ID   IMAGE                  COMMAND   CREATED          STATUS          PORTS     NAMES
12345   zk-auth-server   "bash"    20 minutes ago   Up 20 minutes             zk-auth-server_run_####

```

Connect to the running container

```
$ docker exec -it container_id /bin/bash
```

Run the client:

```
root@###:/zk-auth-server# cargo run --bin client --release
```


