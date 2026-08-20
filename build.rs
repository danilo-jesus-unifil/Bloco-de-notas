fn main() {
    #[cfg(windows)]
    {
        let mut resource = winres::WindowsResource::new();
        resource.set_manifest_file("assets/app.manifest");
        resource.set_icon("assets/app.ico");
        resource
            .compile()
            .expect("failed to compile Windows resources");
    }
}
