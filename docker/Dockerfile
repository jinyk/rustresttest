FROM ubuntu:trusty

MAINTAINER Jinyoung Kim <jinyoungkim@gmail.com>

RUN apt-get update -y && apt-get install -y \
  sudo \
  build-essential \
  software-properties-common \
  wget \
  curl \
  git \
  man \
  unzip \
  nano \
  zsh \
  libssl-dev \
  pkg-config

RUN useradd -m docker && echo "docker:docker" | chpasswd && adduser docker sudo

RUN curl -sSf https://static.rust-lang.org/rustup.sh | sh

RUN git clone git://github.com/robbyrussell/oh-my-zsh.git ~/.oh-my-zsh && \
  chsh -s /bin/zsh
COPY zshrc-template /root/.zshrc

VOLUME /code
WORKDIR /code
