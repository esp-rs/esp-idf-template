---
name: Bug report
about: Create a report to help us improve
title: ''
labels: ["bug", "status:needs-attention"]
assignees: ''

---

<!-- If you have issues building the template project, check if CI is having the
same issue at https://github.com/esp-rs/esp-idf-template/actions. If CI is green,
there might be an issue with your environment, please check that the prerequistes
(https://github.com/esp-rs/esp-idf-template#prerequisites) are properly installed
and that environment variables (LIBCLANG_PATH, PATH...) are properly updated.

Have a look at our std troubleshooting section
(https://esp-rs.github.io/book/troubleshooting/std.html)

If you are still encountering the issue, feel free to ask for help in our
matrix channel (https://matrix.to/#/#esp-rs:matrix.org) or open the issue-->

## Bug description

<!-- A clear and concise description of what the bug is. -->

- Would you like to work on a fix? [y/n]

## To Reproduce

<!-- Steps to reproduce the behavior. -->
1. ...
2. ...

<!-- Please share the minimal repro of the issue where the bug can be reproduced. -->

## Expected behavior

<!-- A clear and concise description of what you expected to happen. Attach screenshots if needed. -->

## Environment

- ESP-IDF branch or tag: [e.g. release/v5.2, v5.2.1, etc.] <!-- if you use the cargo-based build, you have the ESP-IDF version inside the `.cargo/config.toml` file of your binary crate, variable `ESP_IDF_VERSION` in section [env] . -->
- Target device (MCU): [e.g. esp32s3] <!-- if you use the cargo-based build, you have the MCU inside the `.cargo/config.toml` file of your binary crate, variable `MCU` in section [env] . -->
- OS: [e.g. Ubuntu 20.04]
- How did you install the environment: [e.g. espup 0.3.2]
