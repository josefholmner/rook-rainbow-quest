param([Parameter(Mandatory=$true)][string]$Destination)
$ErrorActionPreference = 'Stop'
. (Join-Path $PSScriptRoot 'environment.ps1')
$metadataRaw = & cargo metadata --locked --filter-platform x86_64-pc-windows-msvc --format-version 1 --manifest-path (Join-Path $projectRoot 'Cargo.toml')
if ($LASTEXITCODE -ne 0) { throw 'Could not inspect dependency licenses.' }
$metadata = $metadataRaw | ConvertFrom-Json
$licenseDir = Join-Path $Destination 'LICENSES'
New-Item -ItemType Directory -Path $licenseDir -Force | Out-Null
$inventory = [System.Collections.Generic.List[string]]::new()
$inventory.Add('# Windows dependency license inventory')
$inventory.Add('')
foreach ($package in $metadata.packages | Where-Object { $_.source }) {
    $sourceDir = Split-Path -Parent $package.manifest_path
    $destDir = Join-Path $licenseDir "$($package.name)-$($package.version)"
    New-Item -ItemType Directory -Path $destDir -Force | Out-Null
    Copy-Item -LiteralPath $package.manifest_path -Destination (Join-Path $destDir 'Cargo.toml')
    $licenseFiles = @(Get-ChildItem -LiteralPath $sourceDir -Recurse -File | Where-Object { $_.Name -match '^(LICENSE|LICENCE|COPYING|UNLICENSE|COPYRIGHT)' })
    foreach ($file in $licenseFiles) {
        $relative = $file.FullName.Substring($sourceDir.Length).TrimStart('\','/')
        $destFile = Join-Path $destDir $relative
        New-Item -ItemType Directory -Path (Split-Path -Parent $destFile) -Force | Out-Null
        Copy-Item -LiteralPath $file.FullName -Destination $destFile
    }
    if ($licenseFiles.Count -eq 0) {
        $fallback = if ($package.name -in @('cozy-chess','cozy-chess-types')) { 'cozy-chess-MIT.txt' } elseif ($package.name -eq 'quad-rand') { 'quad-rand-MIT.txt' } elseif ($package.name -in @('dasp_frame','dasp_sample','quad-snd')) { 'dasp-LICENSE-APACHE' } else { throw "Missing license text for $($package.name)" }
        Copy-Item -LiteralPath (Join-Path $projectRoot "assets\licenses\$fallback") -Destination $destDir
    }
    $inventory.Add("- $($package.name) $($package.version): $($package.license)")
}
Copy-Item -LiteralPath (Join-Path $projectRoot 'assets\Nunito-LICENSE.txt') -Destination $licenseDir
$inventory | Set-Content -LiteralPath (Join-Path $licenseDir 'INVENTORY.md') -Encoding utf8

