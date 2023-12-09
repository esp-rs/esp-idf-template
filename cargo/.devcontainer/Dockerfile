ARG VARIANT=bullseye
FROM debian:${VARIANT}
ENV DEBIAN_FRONTEND=noninteractive
ENV LC_ALL=C.UTF-8
ENV LANG=C.UTF-8

# Arguments
ARG CONTAINER_USER=esp
ARG CONTAINER_GROUP=esp
ARG ESP_BOARD=all
ARG GITHUB_TOKEN

# Install dependencies
RUN apt-get update \
    && apt-get install -y git curl gcc clang ninja-build libudev-dev unzip xz-utils\
    python3 python3-pip python3-venv libusb-1.0-0 libssl-dev pkg-config libpython2.7 \
    && apt-get clean -y && rm -rf /var/lib/apt/lists/* /tmp/library-scripts

# Set users
RUN adduser --disabled-password --gecos "" ${CONTAINER_USER}
USER ${CONTAINER_USER}
WORKDIR /home/${CONTAINER_USER}

# Install rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- \
    --default-toolchain none -y --profile minimal

# Update envs
ENV PATH=${PATH}:/home/${CONTAINER_USER}/.cargo/bin

# Compile binaries to avoid GLIBC missmatch error
RUN rustup default stable \
    && cargo install espup --version 0.10.0 \
    && cargo install ldproxy --version 0.3.3 \
    && cargo install espflash --version 2.1.0 \
    && cargo install --git https://github.com/esp-rs/esp-web-flash-server --tag v0.2.1

# Install Xtensa Rust
RUN if [ -n "${GITHUB_TOKEN}" ]; then export GITHUB_TOKEN=${GITHUB_TOKEN}; fi  \
    && ${HOME}/.cargo/bin/espup install\
    --targets "${ESP_BOARD}" \
    --log-level debug \
    --export-file /home/${CONTAINER_USER}/export-esp.sh

# Set default toolchain
{% if arch == "riscv" -%}
RUN rustup default nightly
{% else -%}
RUN rustup default esp
{% endif %}

# Activate ESP environment
RUN echo "source /home/${CONTAINER_USER}/export-esp.sh" >> ~/.bashrc

CMD [ "/bin/bash" ]
