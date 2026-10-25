$ErrorActionPreference = 'Stop'
$projectRoot = Split-Path -Parent $PSScriptRoot
$localTools = if ($env:ROOK_RAINBOW_QUEST_TOOLCHAIN) { $env:ROOK_RAINBOW_QUEST_TOOLCHAIN } else { Join-Path $projectRoot '.tools' }
if (Test-Path -LiteralPath (Join-Path $localTools 'cargo\bin\cargo.exe')) {
    $env:CARGO_HOME = Join-Path $localTools 'cargo'
    $env:RUSTUP_HOME = Join-Path $localTools 'rustup'
    $env:PATH = "$env:CARGO_HOME\bin;$env:PATH"
}

