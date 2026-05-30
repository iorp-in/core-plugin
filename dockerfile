FROM ubuntu

RUN dpkg --add-architecture i386 && \
    apt-get update && \
    apt-get install -y \
        gcc-multilib \
        g++-multilib \
        libc6-dev-i386 \
        curl \
        pkg-config \
        libssl-dev

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

RUN rustup target add i686-unknown-linux-gnu

WORKDIR /workspace