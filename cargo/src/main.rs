{% unless std -%}
#![no_std]
#![no_main]
{% endunless -%}
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
{% if std == false and hal != "No" %}
use log::*;
{% endif %}

{% unless std -%}
#[no_mangle]
{%- endunless %}
fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
{%- if std %}
    println!("Hello, world!");
{% elsif hal == "Yes (default features)" or hal == "Yes (all features)" %}
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("Hello, world!");
{%- endif %}
}
