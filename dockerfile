FROM i386/ubuntu

# Update default packages
RUN apt-get update

# Get Ubuntu packages
RUN apt-get install -y \
    build-essential \
    curl \
    pkg-config \
    libssl-dev

# Update new packages
RUN apt-get update

# Get Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

RUN rustup toolchain install stable-i686-unknown-linux-gnu
RUN rustup default stable-i686-unknown-linux-gnu

# Copy app
WORKDIR /app
COPY . .

# build app
RUN cargo build --release

# Copy lib into root
RUN cp target/release/libiorp_core.so iorp_core.so