FROM ubuntu:16.04

RUN \
  cd /opt/spark_submit_helper && \
  cargo build && \
  
