# tutorial-dockerfile-app

A small Rust (axum) word counter with its own Dockerfile, used in the Light Cloud tutorial
[Bring your own Dockerfile: deploy anything that runs in a container](https://blog.light-cloud.com/tutorials/deploy-any-app-with-a-dockerfile).

```mermaid
flowchart LR
  push[git push] --> build[Light Cloud builds your Dockerfile]
  build --> s1[Stage 1: rust image compiles the binary]
  s1 --> s2[Stage 2: debian-slim with the binary only]
  s2 -->|EXPOSE 8080 / PORT| app[Running container]
```

## Run it

```sh
cargo run
curl -X POST localhost:8080/count -H "content-type: application/json" -d '{"text":"hello world"}'
```
