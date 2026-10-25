param([switch]$Staged)
$ErrorActionPreference='Stop'
$projectRoot=Split-Path -Parent $PSScriptRoot
$gitOptions=@('-c',"safe.directory=$($projectRoot.Replace('\','/'))",'-C',$projectRoot)
$raw=& git @gitOptions ls-files --cached --others --exclude-standard -z
if ($LASTEXITCODE -ne 0) { throw 'Could not enumerate repository files.' }
$paths=(($raw -join "`n").Split([char]0) | Where-Object { $_ } | Sort-Object -Unique)
$binaryExtensions=@('.png','.jpg','.jpeg','.gif','.webp','.bmp','.ico','.rgba','.tga','.tif','.tiff','.dds','.ttf','.otf','.woff','.woff2','.wav','.ogg','.mp3','.flac','.mp4','.webm','.avi','.exe','.dll','.pdb','.lib','.obj','.res','.zip','.7z','.gz','.tar','.pdf','.docx','.pptx','.xlsx')
$strictUtf8=[Text.UTF8Encoding]::new($false,$true)
$count=0
foreach ($relative in $paths) {
    $path=Join-Path $projectRoot $relative
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) { continue }
    $bytes=[IO.File]::ReadAllBytes($path)
    $binary=[IO.Path]::GetExtension($relative).ToLowerInvariant() -in $binaryExtensions
    if (-not $binary -and $bytes.Length -gt 0) {
        $binary=[Array]::IndexOf($bytes,[byte]0,0,[Math]::Min(8000,$bytes.Length)) -ge 0
        if (-not $binary) { try { $null=$strictUtf8.GetString($bytes) } catch { $binary=$true } }
    }
    if (-not $binary) { continue }
    $count++
    $attribute=& git @gitOptions check-attr filter -- $relative
    if ($LASTEXITCODE -ne 0 -or $attribute -notlike '*: filter: lfs') { throw "Binary file is not covered by Git LFS: $relative" }
    if ($bytes.Length -lt 1024 -and [Text.Encoding]::UTF8.GetString($bytes).StartsWith('version https://git-lfs.github.com/spec/v1')) {
        throw "Missing LFS content for $relative. Run git lfs pull before building."
    }
    if ($Staged) {
        $size=& git @gitOptions cat-file -s ":$relative"
        if ($LASTEXITCODE -ne 0) { throw "Stage $relative with Git LFS before checking in." }
        if ([long]$size -gt 1024) { throw "Staged binary is not an LFS pointer: $relative. Run git add --renormalize --all." }
        $pointer=(& git @gitOptions show ":$relative") -join "`n"
        $hash=(Get-FileHash -LiteralPath $path -Algorithm SHA256).Hash.ToLowerInvariant()
        if (-not $pointer.StartsWith('version https://git-lfs.github.com/spec/v1') -or $pointer -notmatch "(?m)^oid sha256:$hash$" -or $pointer -notmatch "(?m)^size $($bytes.Length)$") {
            throw "Staged LFS pointer does not match the working binary: $relative"
        }
    }
}
if ($count -eq 0) { throw 'No binary assets were audited; check the repository contents.' }
$mode=if ($Staged) { 'attributes, real content and staged SHA-256/size pointers' } else { 'attributes and real working content' }
Write-Host "LFS audit passed: $count binary files ($mode)."
