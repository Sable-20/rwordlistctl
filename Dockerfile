FROM rust:latest as builder
WORKDIR /usr/src/rwordlistctl
COPY . .
RUN install -Dv /dev/null /usr/share/wordlistctl/.config/config.toml
#RUN mkdir -p /usr/share/wordlistctl/.config
#RUN touch /usr/share/wordlistctl/.config/config.toml
RUN cp config/config.toml /usr/share/wordlistctl/.config/config.toml
# reqwest = { version = "0.12.2", features = ["gzip", "deflate", "stream", "blocking", "brotli"] }
RUN sed -i 's/^reqwest = { version = "\(\d*\.*\d*\.*\d*\)", features = \[\("gzip", "deflate", "stream", "blocking", "brotli"\)\] }/reqwest = { version = "\1", default-features = false, features = \["http2", "rustls-tls", \2\] }' Cargo.toml
RUN cargo install --path .

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/rwordlistctl /usr/local/bin/rwordlistctl
ENTRYPOINT ["rwordlistctl"]

# FROM python:alpine

# LABEL version="0.8.8" \
#     author="Author BlackArch (https://github.com/BlackArch)" \
#     docker_build="docker build -t blackarch/wordlistctl:0.8.8 ." \
#     docker_run_basic="docker run --rm blackarch/wordlistctl:0.8.8 -h"

# COPY [".", "/wordlistctl"]

# ENV PATH=${PATH}:/wordlistctl

# RUN pip install -r /wordlistctl/requirements.txt && \
#     mkdir -p /usr/share/wordlists/{usernames,passwords,discovery,fuzzing,misc} && \
#     addgroup wordlistctl && \
#     adduser -G wordlistctl -g "Wordlistctl user" -s /bin/sh -D wordlistctl && \
#     chown -R wordlistctl.wordlistctl /wordlistctl /usr/share/wordlists && \
#     export RANDOM_PASSWORD=$(tr -dc A-Za-z0-9 </dev/urandom | head -c44) && \
#     echo "root:$RANDOM_PASSWORD" | chpasswd && \
#     unset RANDOM_PASSWORD && \
#     passwd -l root

# USER wordlistctl

# ENTRYPOINT ["wordlistctl.py"]

# CMD ["-h"]
