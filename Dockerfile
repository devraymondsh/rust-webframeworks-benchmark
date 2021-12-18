FROM rust:1.57-slim

RUN apt-get update && apt-get -y upgrade && apt-get install -y openssl wrk build-essential manpages-dev curl jq

WORKDIR rust_web_frameworks_benchmark/

RUN mkdir scripts

ENTRYPOINT [ "/bin/bash", "-c", "scripts/benchmark.sh" ]