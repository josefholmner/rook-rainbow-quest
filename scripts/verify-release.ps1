param(
    [Parameter(Mandatory=$true)][string]$Executable,
    [Parameter(Mandatory=$true)][string]$OutputDirectory
)
$ErrorActionPreference = 'Stop'
$Executable = (Resolve-Path -LiteralPath $Executable).Path
New-Item -ItemType Directory -Path $OutputDirectory -Force | Out-Null
$OutputDirectory = (Resolve-Path -LiteralPath $OutputDirectory).Path
$hash = (Get-FileHash -LiteralPath $Executable -Algorithm SHA256).Hash
$blocked = Get-Content -LiteralPath (Join-Path $PSScriptRoot 'blocked-release-sha256.txt') | Where-Object { $_ -match '^[0-9A-Fa-f]{64}$' }
if ($hash -in $blocked) { throw "Release blocked: SHA-256 $hash was previously quarantined. Microsoft review is required before reconsidering this binary." }
$defender = Get-ChildItem -LiteralPath "$env:ProgramData\Microsoft\Windows Defender\Platform" -Filter MpCmdRun.exe -Recurse -File | Sort-Object FullName -Descending | Select-Object -First 1
if (-not $defender) { throw 'Cannot validate the Windows release: Microsoft Defender scanner was not found.' }
$started = Get-Date
function Invoke-ReleaseScan([string]$Name) {
    $scan = & $defender.FullName -Scan -ScanType 3 -File $Executable 2>&1
    $scanExit = $LASTEXITCODE
    $scan | Set-Content -LiteralPath (Join-Path $OutputDirectory "$Name.txt") -Encoding utf8
    if ($scanExit -ne 0 -or ($scan -join "`n") -notmatch 'found no threats' -or -not (Test-Path -LiteralPath $Executable)) { throw "Defender validation failed ($scanExit). See $OutputDirectory\$Name.txt. Candidate has not been published." }
}
Invoke-ReleaseScan 'defender-before'
$campaign = Join-Path $OutputDirectory 'campaign'
New-Item -ItemType Directory -Path $campaign -Force | Out-Null
$process = Start-Process -FilePath $Executable -ArgumentList '--campaign-test','--screenshots',('"'+$campaign+'"') -WorkingDirectory (Split-Path -Parent $Executable) -WindowStyle Hidden -RedirectStandardOutput (Join-Path $OutputDirectory 'campaign-stdout.txt') -RedirectStandardError (Join-Path $OutputDirectory 'campaign-stderr.txt') -PassThru
if (-not $process.WaitForExit(180000)) {
    try { $process.Kill() } catch { Write-Warning $_.Exception.Message }
    throw 'Release campaign test exceeded its timeout.'
}
if ($process.ExitCode -ne 0) { throw "Release campaign failed ($($process.ExitCode))." }
if ((Get-Content -Raw -LiteralPath (Join-Path $OutputDirectory 'campaign-stdout.txt')) -notmatch 'Campaign passed: 60 levels') { throw 'Release campaign did not confirm all sixty levels.' }
& (Join-Path $PSScriptRoot 'native-regressions.ps1') -Executable $Executable -OutputDirectory (Join-Path $OutputDirectory 'regressions')
Invoke-ReleaseScan 'defender-after'
# Include real-time detections, since an initial on-demand scan previously missed this candidate.
try {
    $events = @(Get-WinEvent -FilterHashtable @{LogName='Microsoft-Windows-Windows Defender/Operational'; Id=1116} -ErrorAction Stop | Where-Object { $_.TimeCreated -ge $started -and $_.Message.IndexOf($Executable, [StringComparison]::OrdinalIgnoreCase) -ge 0 })
} catch {
    if ($_.FullyQualifiedErrorId -notlike 'NoMatchingEventsFound*') { throw }
    $events = @()
}
if ($events.Count) {
    $events | Select-Object TimeCreated,Message | Format-List | Out-String | Set-Content -LiteralPath (Join-Path $OutputDirectory 'defender-detections.txt') -Encoding utf8
    throw 'Defender real-time protection detected the release candidate. Publication is blocked.'
}
if ((Get-FileHash -LiteralPath $Executable -Algorithm SHA256).Hash -ne $hash) { throw 'Executable changed during validation.' }
"Validated SHA-256: $hash`r`nNative campaign and eight native regressions passed; pre/post Defender scans passed. This is local validation, not a Microsoft malware-analysis verdict." | Set-Content -LiteralPath (Join-Path $OutputDirectory 'validation.txt') -Encoding utf8
