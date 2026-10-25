param([switch]$Native)
$ErrorActionPreference = 'Stop'
. (Join-Path $PSScriptRoot 'environment.ps1')
& (Join-Path $PSScriptRoot 'verify-lfs.ps1')
Push-Location $projectRoot
try {
    & cargo fmt --all -- --check
    if ($LASTEXITCODE -ne 0) { throw 'Formatting check failed.' }
    & cargo test --locked
    if ($LASTEXITCODE -ne 0) { throw 'Tests failed.' }
    & cargo clippy --locked --all-targets -- -D warnings
    if ($LASTEXITCODE -ne 0) { throw 'Clippy check failed.' }
    & (Join-Path $PSScriptRoot 'test-packaging.ps1')
    if ($Native) {
        $buildRoot = if ($env:CARGO_TARGET_DIR) { $env:CARGO_TARGET_DIR } else { Join-Path $projectRoot 'target' }
        if (-not [IO.Path]::IsPathRooted($buildRoot)) { $buildRoot = Join-Path $projectRoot $buildRoot }
        & cargo run --locked -- --campaign-test --screenshots (Join-Path $projectRoot 'target\smoke-screenshots')
        if ($LASTEXITCODE -ne 0) { throw 'Native smoke test failed.' }
        & (Join-Path $PSScriptRoot 'native-regressions.ps1') -Executable (Join-Path $buildRoot 'debug\rook-rainbow-quest.exe') -OutputDirectory (Join-Path $projectRoot 'target\settings-regressions')
    }
} finally { Pop-Location }

