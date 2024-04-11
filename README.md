# rust-http-server

A sample implementation of a bare bones http server in rust. Also includes functionality of serving files located in the public folder.

## Getting started

Here's how to run the project on your local machine for development and testing purposes. First Ensure you have rust tooling installed (https://rustup.rs/), then:

cd into project:

```shell
$ cd rust-http-server
```

create .env file:

```shell
$ echo API_URL=http://localhost:{PORT} > .env
```

run server:

```shell
$ cargo run -p server
```

## Endpoints

home

```shell
$ http://localhost:{PORT}/
```

health

```shell
$ http://localhost:{PORT}/health
```

orders

```shell
$ http://localhost:{PORT}/api/v1/orders
```
