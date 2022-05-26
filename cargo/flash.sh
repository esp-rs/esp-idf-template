#!/usr/bin/env bash

set -e

BUILD_MODE=""
case "$1" in
    ""|"release")
        bash build.sh
        BUILD_MODE="release"
        ;;
    "debug")
        bash build.sh debug
        BUILD_MODE="debug"
        ;;
    *)
        echo "Wrong argument. Only \"debug\"/\"release\" arguments are supported"
        exit 1;;
esac

export ESP_ARCH=
{%- if mcu == "esp32c3" -%}
riscv32imac-unknown-none-elf
{%- else -%}
xtensa-{{ mcu }}-none-elf
{%- endif %}

web-flash --chip {{ mcu }} target/${ESP_ARCH}/${BUILD_MODE}/{{ crate_name }}
