# Configure only this repository, stage the complete project and verify real LFS pointers.
# Does not commit or push.
$ErrorActionPreference='Stop'
$projectRoot=Split-Path -Parent $PSScriptRoot
& git -C $projectRoot lfs install --local
if ($LASTEXITCODE -ne 0) { throw 'Git LFS local setup failed. Run this script from your normal terminal if a sandbox protects .git.' }
& git -C $projectRoot add -- .gitattributes
if ($LASTEXITCODE -ne 0) { throw 'Could not stage .gitattributes.' }
& git -C $projectRoot add --all
if ($LASTEXITCODE -ne 0) { throw 'Could not stage the new repository.' }
& (Join-Path $PSScriptRoot 'verify-lfs.ps1') -Staged
Write-Host 'Ready for your review and commit. No commit or push was made.'
