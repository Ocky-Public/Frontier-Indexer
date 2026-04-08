FROM debian:trixie-slim AS runtime

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

COPY ./target/release/indexer /opt/indexer/bin/indexer

RUN ["chmod", "+x", "/opt/indexer/bin/indexer"]

ENTRYPOINT ["/opt/indexer/bin/indexer"]