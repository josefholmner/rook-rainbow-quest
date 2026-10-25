$ErrorActionPreference = 'Stop'
. (Join-Path $PSScriptRoot 'environment.ps1')
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) { & (Join-Path $PSScriptRoot 'setup.ps1') }
Push-Location $projectRoot
try {
    & cargo run --locked
    if ($LASTEXITCODE -ne 0) { throw 'Game did not exit successfully.' }
} finally { Pop-Location }

