FROM rust:latest as builder
WORKDIR /usr/src/rwordlistctl
COPY . .
RUN install -Dv /dev/null /usr/share/wordlistctl/.config/config.toml
RUN install -Dv /dev/null ~/.config/rwordlistctl/config.toml

##### 
# remove config in /usr/share thats weird, repo can be kept though
####
RUN cp config/config.toml /usr/share/wordlistctl/.config/config.toml
RUN cp config/config.toml ~/.config/rwordlistctl/config.toml
# sed command
# -i to change it in the file
# s for substitution
# \(.*\) detects the version and saves it to capture group 1
# features = \[\(.*\)\] detects features and saves to capture group 2
# replacement just saves the version, disables default features, and adds in features http2, and rustls-tls so that this can be performed in docker
RUN sed -i 's/reqwest = { version = "\(.*\)", features = \[\(.*\)\] }/reqwest = { version = "\1", default-features = false, features = \["http2", "rustls-tls", \2\] }/' Cargo.toml
RUN cargo install --path .

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/rwordlistctl /usr/local/bin/rwordlistctl
ENTRYPOINT ["/bin/bash"]
# ENTRYPOINT ["rwordlistctl"]

#########
# will likely convert to docker-compose so that everything can be run in a terminal
# or maybe ill keep it this way and just get rid of `ENTRYPOINT` command
#########

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
