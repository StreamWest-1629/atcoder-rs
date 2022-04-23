FROM rust:bullseye

WORKDIR /tmp
RUN apt-get update && \
    apt-get install -y build-essential git clang cmake libstdc++-10-dev \
        libssl-dev libxxhash-dev zlib1g-dev pkg-config && \
    git clone https://github.com/rui314/mold.git && \
    cd mold && \
    git checkout v1.2.0 && \
    make -j$(nproc) CXX=clang++ && \
    make install && \
    mold -run cargo install cargo-generate && \
    mold -run rustup component add rustfmt clippy rls
