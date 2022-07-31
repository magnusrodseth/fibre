# `fibre` - A multi-threaded web server

## Description

Fibre is a **very simple** web server, written from scratch in Rust.

It uses a thread pool of worker threads to handle incoming requests, 
serving sample HTML content found in the [`static`](./static) directory.

## Interacting with the web server

```sc
# Navigate to the project directory
cd fibre

# Start the application
cargo run
```

In another terminal tab, use `curl` to query the web server:

```sh
# Send parallel requests. The `-P5` flags determines the amount of parallel requests.
seq 1 200 | xargs -n1 -P5  curl "http://localhost:7878"
```

Alternatively, use your web browser of choice (e.g. Google Chrome), and visit the URL `localhost:7878`.
