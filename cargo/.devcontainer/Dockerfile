FROM ghcr.io/rust-lang/rust:nightly-bookworm
ENV DEBIAN_FRONTEND=noninteractive
ENV LC_ALL=C.UTF-8
ENV LANG=C.UTF-8

# Arguments
ARG CONTAINER_USER=esp
ARG CONTAINER_GROUP=esp
ARG ESP_BOARD=all
ARG GITHUB_TOKEN

ARG RUST_INSTALL_DIR=/usr/local/cargo

# Install dependencies needed for esp-idf + clang for riscv rust
RUN apt-get update \
    && apt-get install -y flex bison gperf python3-pip python3-venv cmake ninja-build ccache dfu-util clang \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/* /tmp/library-scripts

# Set users
RUN adduser --disabled-password --gecos "" ${CONTAINER_USER}
USER ${CONTAINER_USER}
WORKDIR /home/${CONTAINER_USER}

# Install extra crates
RUN ARCH=$(${RUST_INSTALL_DIR}/bin/rustup show | grep "Default host" | sed -e 's/.* //') && \
    {% if arch != "riscv" -%}
    curl -L "https://github.com/esp-rs/espup/releases/latest/download/espup-${ARCH}" -o "${RUST_INSTALL_DIR}/bin/espup" && \
    chmod u+x "${RUST_INSTALL_DIR}/bin/espup" && \
    {% endif -%}
    curl -L "https://github.com/esp-rs/embuild/releases/latest/download/ldproxy-${ARCH}.zip" -o "${RUST_INSTALL_DIR}/bin/ldproxy.zip" && \
    unzip "${RUST_INSTALL_DIR}/bin/ldproxy.zip" -d "${RUST_INSTALL_DIR}/bin/" && \
    rm "${RUST_INSTALL_DIR}/bin/ldproxy.zip" && \
    chmod u+x "${RUST_INSTALL_DIR}/bin/ldproxy" && \
    curl -L "https://github.com/bjoernQ/esp-web-flash-server/releases/latest/download/web-flash-${ARCH}.zip" -o "${RUST_INSTALL_DIR}/bin/web-flash.zip" && \
    unzip "${RUST_INSTALL_DIR}/bin/web-flash.zip" -d "${RUST_INSTALL_DIR}/bin/" && \
    rm "${RUST_INSTALL_DIR}/bin/web-flash.zip" && \
    chmod u+x "${RUST_INSTALL_DIR}/bin/web-flash"

# Install Xtensa Rust
{% if arch != "riscv" -%}
RUN if [ -n "${GITHUB_TOKEN}" ]; then export GITHUB_TOKEN=${GITHUB_TOKEN}; fi  \
    && ${RUST_INSTALL_DIR}/bin/espup install\
    --targets "${ESP_BOARD}" \
    --log-level debug \
    --export-file /home/${CONTAINER_USER}/export-esp.sh
{% endif %}

# Set default toolchain
{% if arch == "riscv" -%}
RUN rustup default nightly
{% else -%}
RUN rustup default esp
{% endif %}

{% if arch != "riscv" -%}
# Activate ESP environment
RUN echo "source /home/${CONTAINER_USER}/export-esp.sh" >> ~/.bashrc
{% endif %}

CMD [ "/bin/bash" ]
