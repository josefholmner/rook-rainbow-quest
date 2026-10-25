$ErrorActionPreference = 'Stop'
. (Join-Path $PSScriptRoot 'environment.ps1')
& (Join-Path $PSScriptRoot 'verify-lfs.ps1')
Push-Location $projectRoot
try {
    & cargo build --locked --release
    if ($LASTEXITCODE -ne 0) { throw 'Release build failed.' }
} finally { Pop-Location }
$manifest = Get-Content -Raw -LiteralPath (Join-Path $projectRoot 'Cargo.toml')
$gameVersion = [regex]::Match($manifest, '(?m)^version\s*=\s*"([^"]+)"').Groups[1].Value
if (-not $gameVersion) { throw 'Package version is missing.' }
$packageName = "RookRainbowQuest-$gameVersion"
$buildRoot = if ($env:CARGO_TARGET_DIR) { $env:CARGO_TARGET_DIR } else { Join-Path $projectRoot 'target' }
if (-not [IO.Path]::IsPathRooted($buildRoot)) { $buildRoot = Join-Path $projectRoot $buildRoot }
$stageRoot = Join-Path $buildRoot ("package-staging\" + [guid]::NewGuid().ToString('N'))
$packageDir = Join-Path $stageRoot $packageName
New-Item -ItemType Directory -Path $packageDir -Force | Out-Null
Copy-Item -LiteralPath (Join-Path $buildRoot 'release\rook-rainbow-quest.exe') -Destination (Join-Path $packageDir 'RookRainbowQuest.exe')
Copy-Item -LiteralPath (Join-Path $buildRoot 'release\rook-rainbow-quest-launcher.exe') -Destination (Join-Path $stageRoot 'Play Rook Rainbow Quest.exe')
Copy-Item -LiteralPath (Join-Path $stageRoot 'Play Rook Rainbow Quest.exe') -Destination (Join-Path $packageDir 'Play Rook Rainbow Quest.exe')
Copy-Item -LiteralPath (Join-Path $projectRoot 'PLAY.txt') -Destination $packageDir
Copy-Item -LiteralPath (Join-Path $projectRoot 'THIRD-PARTY-NOTICES.md') -Destination $packageDir
& (Join-Path $PSScriptRoot 'licenses.ps1') -Destination $packageDir
& (Join-Path $PSScriptRoot 'verify-release.ps1') -Executable (Join-Path $packageDir 'RookRainbowQuest.exe') -OutputDirectory (Join-Path $stageRoot 'validation')
& (Join-Path $PSScriptRoot 'verify-launcher.ps1') -Launcher (Join-Path $stageRoot 'Play Rook Rainbow Quest.exe') -Version $gameVersion -OutputDirectory (Join-Path $stageRoot 'launcher-validation')
Get-ChildItem -LiteralPath $packageDir -Recurse -File | Where-Object { $_.LastWriteTime -lt [datetime]'1980-01-01' } | ForEach-Object { $_.LastWriteTime = [datetime]'1980-01-01' }
$stageArchive = Join-Path $stageRoot "$packageName-Windows-x64.zip"
Compress-Archive -Path (Join-Path $packageDir '*') -DestinationPath $stageArchive
# Publish only after all validation succeeds; a rejected candidate leaves the current launcher alone.
$publishedDir = Join-Path $projectRoot "dist\$packageName"
New-Item -ItemType Directory -Path $publishedDir -Force | Out-Null
Copy-Item -Path (Join-Path $packageDir '*') -Destination $publishedDir -Recurse -Force
$versionedArchive = Join-Path $projectRoot "dist\RookRainbowQuest-$gameVersion-Windows-x64.zip"
Copy-Item -LiteralPath $stageArchive -Destination $versionedArchive -Force
Copy-Item -LiteralPath $versionedArchive -Destination (Join-Path $projectRoot 'dist\RookRainbowQuest-Windows-x64.zip') -Force
Copy-Item -LiteralPath (Join-Path $stageRoot 'Play Rook Rainbow Quest.exe') -Destination (Join-Path $projectRoot 'dist\Play Rook Rainbow Quest.exe') -Force
$checksums = @(
    ((Get-FileHash -LiteralPath (Join-Path $publishedDir 'RookRainbowQuest.exe') -Algorithm SHA256).Hash + "  $packageName/RookRainbowQuest.exe"),
    ((Get-FileHash -LiteralPath $versionedArchive -Algorithm SHA256).Hash + "  $packageName-Windows-x64.zip"),
    ((Get-FileHash -LiteralPath (Join-Path $projectRoot 'dist\Play Rook Rainbow Quest.exe') -Algorithm SHA256).Hash + "  Play Rook Rainbow Quest.exe")
)
$checksums | Set-Content -LiteralPath (Join-Path $projectRoot "dist\$packageName.sha256") -Encoding ascii
$launcher = @"
@echo off
start "" "%~dp0RookRainbowQuest-$gameVersion\RookRainbowQuest.exe"
"@
$launcher | Set-Content -LiteralPath (Join-Path $projectRoot 'dist\Play-RookRainbowQuest.cmd') -Encoding ascii
Write-Host "Ready: $publishedDir\RookRainbowQuest.exe"

