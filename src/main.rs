// Necessary, so that the `app_main` symbol exported by the `binstart` feature of esp-if-sys is linked
use esp_idf_sys;

fn main() {
    // Temporary. Will disapper once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    println!("Hello, world!");
}
