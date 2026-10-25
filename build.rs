use std::{env, fs, path::PathBuf};
fn main() {
    println!("cargo:rerun-if-changed=levels");
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let mut files: Vec<_> = fs::read_dir(root.join("levels"))
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|e| e == "json"))
        .collect();
    files.sort();
    let mut generated = String::from("const EMBEDDED_LEVELS: &[&str] = &[\n");
    for file in files {
        assert!(
            file.with_extension("md").exists(),
            "Every level needs its own Markdown description: {}",
            file.display()
        );
        generated.push_str(&format!("include_str!({:?}),\n", file.to_str().unwrap()));
    }
    generated.push_str("];\n");
    fs::write(
        PathBuf::from(env::var("OUT_DIR").unwrap()).join("campaign.rs"),
        generated,
    )
    .unwrap();
    windows_resources(&root);
}

fn windows_resources(root: &std::path::Path) {
    if env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    println!("cargo:rerun-if-changed=assets/rook-rainbow-quest.ico");
    println!("cargo:rerun-if-env-changed=RC");
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let version = env::var("CARGO_PKG_VERSION").unwrap();
    let numbers = format!("{},0", version.replace('.', ","));
    let icon = root
        .join("assets/rook-rainbow-quest.ico")
        .display()
        .to_string()
        .replace('\\', "\\\\");
    let source = format!(
        r#"
1 ICON "{icon}"
1 VERSIONINFO
 FILEVERSION {numbers}
 PRODUCTVERSION {numbers}
 FILEOS 0x40004
 FILETYPE 0x1
BEGIN
 BLOCK "StringFileInfo"
 BEGIN
  BLOCK "040904B0"
  BEGIN
   VALUE "FileDescription", "Rook Rainbow Quest\0"
   VALUE "ProductName", "Rook Rainbow Quest\0"
   VALUE "FileVersion", "{version}\0"
   VALUE "ProductVersion", "{version}\0"
  END
 END
 BLOCK "VarFileInfo"
 BEGIN
  VALUE "Translation", 0x409, 1200
 END
END
"#
    );
    let rc_file = out.join("rook-rainbow-quest.rc");
    let resource = out.join("rook-rainbow-quest.res");
    fs::write(&rc_file, source).unwrap();
    let compiler = env::var_os("RC").map(PathBuf::from).unwrap_or_else(|| {
        let kits = PathBuf::from(
            env::var_os("ProgramFiles(x86)").unwrap_or_else(|| "C:/Program Files (x86)".into()),
        )
        .join("Windows Kits/10/bin");
        let mut versions: Vec<_> = fs::read_dir(kits)
            .into_iter()
            .flatten()
            .filter_map(Result::ok)
            .map(|entry| entry.path().join("x64/rc.exe"))
            .filter(|path| path.is_file())
            .collect();
        versions.sort();
        versions.pop().unwrap_or_else(|| "rc.exe".into())
    });
    let status = std::process::Command::new(compiler)
        .arg("/nologo")
        .arg("/fo")
        .arg(&resource)
        .arg(&rc_file)
        .status()
        .expect(
            "Windows SDK resource compiler is required; scripts/setup.ps1 installs the build tools",
        );
    assert!(
        status.success(),
        "Compiling the Windows icon/version resources failed"
    );
    for binary in ["rook-rainbow-quest", "rook-rainbow-quest-launcher"] {
        println!("cargo:rustc-link-arg-bin={binary}={}", resource.display());
    }
}
