# Isolated script regressions: no real executable is launched and Defender is never bypassed.
$ErrorActionPreference = 'Stop'
$fixture = Join-Path (Split-Path -Parent $PSScriptRoot) ('target\package-script-tests\' + [guid]::NewGuid().ToString('N'))
New-Item -ItemType Directory -Path (Join-Path $fixture 'scripts'),(Join-Path $fixture 'target\release'),(Join-Path $fixture 'dist') -Force | Out-Null
Copy-Item -LiteralPath (Join-Path $PSScriptRoot 'package.ps1'),(Join-Path $PSScriptRoot 'verify-release.ps1') -Destination (Join-Path $fixture 'scripts')
'$projectRoot = Split-Path -Parent $PSScriptRoot' | Set-Content (Join-Path $fixture 'scripts\environment.ps1')
'param($Destination)' | Set-Content (Join-Path $fixture 'scripts\licenses.ps1')
'# LFS audit is covered separately; these fixtures are plain text, never executed.' | Set-Content (Join-Path $fixture 'scripts\verify-lfs.ps1')
'version = "9.9.9"' | Set-Content (Join-Path $fixture 'Cargo.toml')
'test fixture, not a program' | Set-Content (Join-Path $fixture 'target\release\rook-rainbow-quest.exe')
'launcher fixture, not a program' | Set-Content (Join-Path $fixture 'target\release\rook-rainbow-quest-launcher.exe')
'param($Launcher,$Version,$OutputDirectory)' | Set-Content (Join-Path $fixture 'scripts\verify-launcher.ps1')
'fixture' | Set-Content (Join-Path $fixture 'PLAY.txt'),(Join-Path $fixture 'THIRD-PARTY-NOTICES.md')
'previous launcher' | Set-Content (Join-Path $fixture 'dist\Play-RookRainbowQuest.cmd')
'previous native launcher' | Set-Content (Join-Path $fixture 'dist\Play Rook Rainbow Quest.exe')
$hash = (Get-FileHash (Join-Path $fixture 'target\release\rook-rainbow-quest.exe')).Hash
$hash | Set-Content (Join-Path $fixture 'scripts\blocked-release-sha256.txt')
$rejected = $false
try {
    & (Join-Path $fixture 'scripts\verify-release.ps1') -Executable (Join-Path $fixture 'target\release\rook-rainbow-quest.exe') -OutputDirectory (Join-Path $fixture 'validation')
} catch {
    if ($_.Exception.Message -notlike 'Release blocked:*') { throw }
    $rejected = $true
}
if (-not $rejected) { throw 'A known quarantined hash was not rejected.' }
Write-Host 'Passed: known quarantined executable is rejected before scanning or execution.'
$oldTarget = $env:CARGO_TARGET_DIR
$env:CARGO_TARGET_DIR = $null
function cargo { $global:LASTEXITCODE = 0 }
try {
    'param($Executable,$OutputDirectory); throw "fixture validation failed"' | Set-Content (Join-Path $fixture 'scripts\verify-release.ps1')
    $rejected = $false
    try { & (Join-Path $fixture 'scripts\package.ps1') } catch {
        if ($_.Exception.Message -ne 'fixture validation failed') { throw }
        $rejected = $true
    }
    if (-not $rejected -or (Test-Path (Join-Path $fixture 'dist\RookRainbowQuest-9.9.9-Windows-x64.zip')) -or (Get-Content -Raw (Join-Path $fixture 'dist\Play-RookRainbowQuest.cmd')).Trim() -ne 'previous launcher' -or (Get-Content -Raw (Join-Path $fixture 'dist\Play Rook Rainbow Quest.exe')).Trim() -ne 'previous native launcher') { throw 'Failed validation changed the published release.' }
    Write-Host 'Passed: failed validation preserves the launcher and publishes no ZIP.'
    'param($Executable,$OutputDirectory)' | Set-Content (Join-Path $fixture 'scripts\verify-release.ps1')
    'param($Launcher,$Version,$OutputDirectory); throw "fixture launcher validation failed"' | Set-Content (Join-Path $fixture 'scripts\verify-launcher.ps1')
    $rejected=$false
    try { & (Join-Path $fixture 'scripts\package.ps1') } catch {
        if ($_.Exception.Message -ne 'fixture launcher validation failed') { throw }
        $rejected=$true
    }
    if (-not $rejected -or (Test-Path (Join-Path $fixture 'dist\RookRainbowQuest-9.9.9-Windows-x64.zip')) -or (Get-Content -Raw (Join-Path $fixture 'dist\Play Rook Rainbow Quest.exe')).Trim() -ne 'previous native launcher') { throw 'Failed launcher validation changed the published release.' }
    Write-Host 'Passed: failed native launcher validation preserves the previous release.'
    'param($Launcher,$Version,$OutputDirectory)' | Set-Content (Join-Path $fixture 'scripts\verify-launcher.ps1')
    & (Join-Path $fixture 'scripts\package.ps1')
    $archive = Join-Path $fixture 'dist\RookRainbowQuest-9.9.9-Windows-x64.zip'
    $checksum = Get-Content -Raw (Join-Path $fixture 'dist\RookRainbowQuest-9.9.9.sha256')
    if (-not (Test-Path $archive) -or $checksum -notmatch $hash -or $checksum -notmatch (Get-FileHash $archive).Hash -or (Get-Content -Raw (Join-Path $fixture 'dist\Play-RookRainbowQuest.cmd')) -notmatch 'RookRainbowQuest-9.9.9') { throw 'Successful packaging did not publish the expected files and hashes.' }
    $launcherHash=(Get-FileHash (Join-Path $fixture 'target\release\rook-rainbow-quest-launcher.exe')).Hash
    if ($checksum -notmatch $launcherHash -or (Get-FileHash (Join-Path $fixture 'dist\Play Rook Rainbow Quest.exe')).Hash -ne $launcherHash) { throw 'Published native launcher or checksum is incorrect.' }
    Write-Host 'Passed: validated packaging publishes the executable, ZIP, checksums and launcher.'
} finally {
    Remove-Item Function:\cargo
    $env:CARGO_TARGET_DIR = $oldTarget
}
