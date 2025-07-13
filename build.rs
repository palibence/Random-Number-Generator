use winresource::WindowsResource;

fn main() {
    slint_build::compile("ui/app-window.slint").expect("Slint build failed");
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let mut res = winresource::WindowsResource::new();
        res.set("FileDescription", "Random Number Generator");
        res.set("ProductName", "Random Number Generator");
        res.set("LegalCopyright", "© 2025 PBBNC");
        res.set("FileVersion", "1.0.0");
        res.set("ProductVersion", "1.0.0");
        res.set_icon("./favicon.ico");
        res.compile().unwrap();
    }

}
