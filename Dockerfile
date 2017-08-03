FROM ubuntu:16.04

COPY src/ /opt/spark_submit_helper/src/
COPY Cargo.toml /opt/spark_submit_helper/

ENV DEBIAN_FRONTEND noninteractive
ENV PATH $PATH:/root/.cargo/bin
WORKDIR /opt/spark_submit_helper

RUN \
  apt-get update && apt-get install -y \
    gcc \
    curl && \
  curl https://sh.rustup.rs -sSf | bash -s -- -y && \
  cargo build && \
  apt purge -y curl && \
  apt-get autoremove -y && \
  apt-get clean && rm -rf /var/lib/apt/lists/*

VOLUME ["/var/log/spark_submit_helper", "/test"]

ENTRYPOINT ["cargo", "run", "|", "tee", "/var/log/spark_submit_helper/error.log"]
