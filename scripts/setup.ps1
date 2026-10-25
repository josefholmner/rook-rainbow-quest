# Install isolated project tooling; players only need the packaged executable.
$ErrorActionPreference = 'Stop'
. (Join-Path $PSScriptRoot 'environment.ps1')
& git -C $projectRoot lfs install --local
if ($LASTEXITCODE -ne 0) { throw 'Install Git for Windows with Git LFS, then retry setup.' }
& git -C $projectRoot lfs pull
if ($LASTEXITCODE -ne 0) { throw 'Git LFS asset download failed.' }
& (Join-Path $PSScriptRoot 'verify-lfs.ps1')
if (-not [Environment]::Is64BitOperatingSystem) { throw 'This build targets 64-bit Windows.' }
$vswhere = Join-Path ${env:ProgramFiles(x86)} 'Microsoft Visual Studio\Installer\vswhere.exe'
$hasCpp = $false
if (Test-Path -LiteralPath $vswhere) {
    $cppInstall = & $vswhere -latest -products '*' -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath
    $hasCpp = [bool]$cppInstall
}
if (-not $hasCpp) {
    Write-Host 'Installing Windows C++ build tools. Windows may request administrator consent.'
    New-Item -ItemType Directory -Path $localTools -Force | Out-Null
    $vsInstaller = Join-Path $localTools 'vs-buildtools.exe'
    Invoke-WebRequest 'https://aka.ms/vs/17/release/vs_BuildTools.exe' -OutFile $vsInstaller
    if ((Get-AuthenticodeSignature -LiteralPath $vsInstaller).Status -ne 'Valid') { throw 'Build tools installer signature is not valid.' }
    $installProcess = Start-Process -FilePath $vsInstaller -ArgumentList '--quiet','--wait','--norestart','--nocache','--add','Microsoft.VisualStudio.Workload.VCTools','--includeRecommended' -Verb RunAs -WindowStyle Hidden -Wait -PassThru
    if ($installProcess.ExitCode -notin @(0,3010)) { throw "Build tools installation failed: $($installProcess.ExitCode)" }
    if ($installProcess.ExitCode -eq 3010) { throw 'Windows needs a restart to finish installing the build tools. Run setup again afterward.' }
}
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    New-Item -ItemType Directory -Path $localTools -Force | Out-Null
    $rustInstaller = Join-Path $localTools 'rustup-init.exe'
    Invoke-WebRequest 'https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe' -OutFile $rustInstaller
    # Verify against the publisher's HTTPS checksum before executing.
    $checksum = (Invoke-WebRequest 'https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe.sha256').Content
    if ((Get-FileHash -LiteralPath $rustInstaller -Algorithm SHA256).Hash.ToLowerInvariant() -ne (($checksum -split '\s+')[0]).ToLowerInvariant()) { throw 'Rust installer checksum mismatch.' }
    $env:CARGO_HOME = Join-Path $localTools 'cargo'
    $env:RUSTUP_HOME = Join-Path $localTools 'rustup'
    & $rustInstaller -y --no-modify-path --profile minimal --default-toolchain stable --default-host x86_64-pc-windows-msvc
    if ($LASTEXITCODE -ne 0) { throw 'Rust toolchain installation failed.' }
    . (Join-Path $PSScriptRoot 'environment.ps1')
}
& rustup component add rustfmt clippy
if ($LASTEXITCODE -ne 0) { throw 'Developer tools installation failed.' }
& cargo fetch --locked --target x86_64-pc-windows-msvc --manifest-path (Join-Path $projectRoot 'Cargo.toml')
if ($LASTEXITCODE -ne 0) { throw 'Dependency download failed.' }
Write-Host 'Ready. Run scripts\test.ps1 or scripts\run.ps1.'

