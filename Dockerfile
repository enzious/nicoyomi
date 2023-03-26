# BUILDER
FROM debian:unstable-slim AS builder

WORKDIR /opt

RUN apt update

RUN apt install -y curl jq 

RUN mkdir -p /opt/calibre && \
  if [ -z ${CALIBRE_RELEASE+x} ]; then \
    CALIBRE_RELEASE=$(curl -sX GET "https://api.github.com/repos/kovidgoyal/calibre/releases/latest" \
    | jq -r .tag_name); \
  fi && \
  CALIBRE_VERSION="$(echo ${CALIBRE_RELEASE} | cut -c2-)" && \
  CALIBRE_URL="https://download.calibre-ebook.com/${CALIBRE_VERSION}/calibre-${CALIBRE_VERSION}-x86_64.txz" && \
  curl -o \
    /opt/calibre-tarball.txz -L \
    "$CALIBRE_URL";

RUN apt install -y xz-utils

RUN tar xvf /opt/calibre-tarball.txz -C /opt/calibre

RUN /opt/calibre/calibre_postinstall

# BASE
FROM debian:unstable-slim AS base

WORKDIR /var/lib/nicoyomi

RUN apt update

COPY --from=builder /opt/calibre/ /opt/calibre
RUN ln -s /opt/calibre/ebook-convert /usr/bin/ebook-convert

RUN apt install -y pip
RUN apt install -y python3-pyqt6
RUN apt install -y python3-bs4
RUN apt install -y python3-pillow

ENV PATH="$PATH:/root/.local/bin"

RUN apt install -y git
RUN git clone -b feat/exclude-folders https://github.com/enzious/mangadex-downloader.git
RUN apt purge -y git

RUN cd mangadex-downloader && pip install . --break-system-packages
RUN rm -rf mangadex-downloader

RUN rm -rf /opt/calibre/libQt6Web*
RUN rm -rf /opt/calibre/resources/qtweb*
RUN rm -rf /usr/lib/x86_64/perl
RUN rm -rf /usr/lib/x86_64/libx265*
RUN rm -rf /usr/lib/x86_64/gcc*
RUN rm -rf /usr/lib/x86_64/git-core*

# RUST
FROM clux/muslrust:1.68.0-nightly-2022-12-31 AS rust

WORKDIR /usr/src/nicoyomi

RUN cargo init

COPY Cargo.lock .
COPY Cargo.toml .

RUN cargo build --release

COPY rustfmt.toml .
COPY src src
COPY templates templates

RUN rm -rf ./target/x86_64-unknown-linux-musl/release/.fingerprint/nicoyomi*

RUN cargo install --path .

# FINAL
FROM base

WORKDIR /var/lib/nicoyomi

COPY --from=rust /root/.cargo/bin/nicoyomi /usr/local/bin

CMD ["nicoyomi"]
