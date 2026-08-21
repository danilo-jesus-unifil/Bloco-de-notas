use std::{env, path::Path};

fn main() {
    println!("cargo:rerun-if-changed=assets/app.manifest");
    println!("cargo:rerun-if-changed=assets/app.ico");

    let target = env::var("TARGET").unwrap_or_default();
    if target.contains("-windows-") {
        let output_dir = env::var("OUT_DIR").expect("OUT_DIR is required for Windows resources");
        let mut resource = winres::WindowsResource::new();
        resource.set_manifest_file("assets/app.manifest");
        resource.set_icon("assets/app.ico");

        if target == "x86_64-pc-windows-gnu" {
            resource.set_windres_path("x86_64-w64-mingw32-windres");
            resource.set_ar_path("x86_64-w64-mingw32-ar");
        }

        resource
            .compile()
            .expect("failed to compile Windows resources");

        if target.ends_with("-gnu") {
            let resource_object = Path::new(&output_dir).join("resource.o");
            println!(
                "cargo:rustc-link-arg-bin=bloco-de-notas={}",
                resource_object.display()
            );
        }
    }
}
