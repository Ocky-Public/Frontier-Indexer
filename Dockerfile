FROM rust:1.90.0 AS builder

ARG PROFILE=release
ARG GIT_REVISION
ENV GIT_REVISION=$GIT_REVISION

WORKDIR work

COPY Cargo.lock Cargo.toml ./
COPY diesel.toml ./
COPY src/ ./src/
COPY migrations/ ./migrations/

RUN apt-get update
RUN apt-get -y --no-install-recommends install \
    build-essential \
    libssl-dev \
    pkg-config \
    curl \
    cmake \
    clang \
    ca-certificates
    
ENV PATH="/root/.cargo/bin:${PATH}"

RUN cargo build --profile $PROFILE --config net.git-fetch-with-cli=true

FROM docker.io/debian:trixie-slim AS runtime

RUN apt-get update
RUN apt-get -y --no-install-recommends install \
    wget \
    iputils-ping \
    procps \
    bind9-host \
    bind9-dnsutils \
    curl \
    iproute2 \
    git \
    ca-certificates \
    libpq-dev \
    postgresql

RUN rm -rf /var/lib/apt/lists/*

RUN mkdir -p /opt/indexer/bin

COPY --from=builder /work/target/release/indexer /opt/indexer/bin/indexer

RUN ["chmod", "+x", "/opt/indexer/bin/indexer"]

ARG BUILD_DATE
LABEL build-date=$BUILD_DATE

ARG GIT_REVISION
LABEL git-revision=$GIT_REVISION

ENTRYPOINT ["/opt/indexer/bin/indexer"]