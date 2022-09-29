# Note: gitpod/workspace-base image references older version of CMake, it's necessary to install newer one
FROM  gitpod/workspace-base
ENV LC_ALL=C.UTF-8
ENV LANG=C.UTF-8

# ARGS
ARG CONTAINER_USER=gitpod
ARG CONTAINER_GROUP=gitpod
ARG TOOLCHAIN_VERSION=1.64.0.0
ARG ESP_IDF_VERSION=
{%- if espidfver == "mainline" -%}
"master"
{%- else -%}
"release/v4.4"
{%- endif %}
ARG ESP_BOARD={{ mcu }}
ARG INSTALL_RUST_TOOLCHAIN=install-rust-toolchain.sh

# Install dependencies
RUN sudo install-packages git curl gcc ninja-build libudev-dev libpython2.7 \
    python3 python3-pip python3-venv libusb-1.0-0 libssl-dev pkg-config libtinfo5 clang
# Set User
USER ${CONTAINER_USER}
WORKDIR /home/${CONTAINER_USER}

# Install Rust toolchain, extra crates and esp-idf
ENV PATH=${PATH}:/home/${CONTAINER_USER}/.cargo/bin:/home/${CONTAINER_USER}/opt/bin
ADD --chown=${CONTAINER_USER}:${CONTAINER_GROUP} \
    https://github.com/esp-rs/rust-build/releases/download/v${TOOLCHAIN_VERSION}/${INSTALL_RUST_TOOLCHAIN} \
    /home/${CONTAINER_USER}/${INSTALL_RUST_TOOLCHAIN}
RUN chmod a+x ${INSTALL_RUST_TOOLCHAIN} \
    && ./${INSTALL_RUST_TOOLCHAIN} \
    --extra-crates "ldproxy cargo-espflash wokwi-server web-flash" \
    --export-file /home/${CONTAINER_USER}/export-esp.sh \
    --esp-idf-version "${ESP_IDF_VERSION}" \
    --minified-esp-idf "YES" \
    --build-target "${ESP_BOARD}" \
    && rustup component add clippy rustfmt

