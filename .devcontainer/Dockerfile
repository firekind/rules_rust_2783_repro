FROM ubuntu:20.04

ARG USERNAME=developer
ARG USER_UID=1000
ARG USER_GID=$USER_UID
ARG BAZELISK_VERSION=1.20.0

RUN apt update && apt install -y wget curl sudo git

RUN wget https://github.com/bazelbuild/bazelisk/releases/download/v${BAZELISK_VERSION}/bazelisk-linux-amd64 -O /usr/bin/bazel && \
    chmod +x /usr/bin/bazel

RUN groupadd --gid $USER_GID $USERNAME \
    && useradd --uid $USER_UID --gid $USER_GID -ms /bin/bash $USERNAME \
    && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME \
    && chmod 0440 /etc/sudoers.d/$USERNAME

USER developer
