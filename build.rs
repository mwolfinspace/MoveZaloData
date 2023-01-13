extern crate winres;

fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("1.ico"); // Replace this with the filename of your .ico file.
        res.compile().unwrap();
        use std::io::Write;
        // only build the resource for release builds
        // as calling rc.exe might be slow

        let mut res = winres::WindowsResource::new();
        res.set_icon("1.ico").set_manifest(
            r#"
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
<trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
    <security>
        <requestedPrivileges>
            <requestedExecutionLevel level="requireAdministrator" uiAccess="false" />
        </requestedPrivileges>
    </security>
</trustInfo>
</assembly>
"#,
        );
        match res.compile() {
            Err(error) => {
                write!(std::io::stderr(), "{}", error).unwrap();
                std::process::exit(1);
            }
            Ok(_) => {}
        }
    }
}
