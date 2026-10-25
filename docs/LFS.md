# Binary assets and Git LFS

`.gitattributes` tracks images (including the icon and concept art), raw RGBA textures, fonts, media, executables, symbol/object/library binaries, packaged archives and binary document formats in Git LFS. Rust, JSON, Markdown, scripts and license texts remain ordinary Git files. The current Windows release and both launcher copies are included in this policy.

## Prepare your check-in

Stage, commit and push with your usual Git client. `.gitattributes` supplies the binary-file rules. The initial staged project passed verification for all 17 binary files, and its LFS pre-push hook is present. The helper below is optional if you prefer automated local setup/staging:

```powershell
.\scripts\prepare-check-in.ps1
git lfs ls-files
git diff --cached --stat
```

The optional script runs repository-local `git lfs install`, installs the pre-push hook, stages `.gitattributes` and then all intended project/release files, and checks every staged binary is an LFS pointer with SHA-256/size matching its actual contents. It does not commit, push, change global Git settings or touch the old repository. Push normally using your chosen client; the LFS pre-push hook uploads binary objects to this repository's LFS storage.

To audit independently:

```powershell
.\scripts\verify-lfs.ps1
.\scripts\verify-lfs.ps1 -Staged
```

The first command checks every eligible tracked/unignored binary has the LFS filter and real local content. It also detects unknown binary types using NUL bytes/invalid UTF-8, so a missing attribute pattern fails explicitly. The second additionally checks the index contains actual pointers rather than raw binaries. A fresh clone needs `git lfs pull` if automatic checkout did not download the objects; setup.ps1 does this automatically. CI checks out LFS content and enforces the pointer audit.

Only the latest versioned release is eligible under `.gitignore`. The unversioned convenience ZIP and validation/build folders are ignored. If a new binary format is added, extend `.gitattributes` before staging. A file is not stored through LFS merely because it is large; attributes and pointer verification are required.
