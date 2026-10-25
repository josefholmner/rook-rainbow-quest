param(
    [Parameter(Mandatory=$true)][string]$Launcher,
    [Parameter(Mandatory=$true)][string]$Version,
    [Parameter(Mandatory=$true)][string]$OutputDirectory
)
$ErrorActionPreference='Stop'
$Launcher=(Resolve-Path -LiteralPath $Launcher).Path
New-Item -ItemType Directory -Path $OutputDirectory -Force | Out-Null
$OutputDirectory=(Resolve-Path -LiteralPath $OutputDirectory).Path
$started=Get-Date
$hash=(Get-FileHash -LiteralPath $Launcher).Hash
$blocked=Get-Content -LiteralPath (Join-Path $PSScriptRoot 'blocked-release-sha256.txt') | Where-Object { $_ -match '^[0-9A-Fa-f]{64}$' }
if ($hash -in $blocked) { throw 'Release blocked: launcher matches a quarantined hash.' }
$scanner=Get-ChildItem -LiteralPath "$env:ProgramData\Microsoft\Windows Defender\Platform" -Filter MpCmdRun.exe -Recurse -File | Sort-Object FullName -Descending | Select-Object -First 1
if (-not $scanner) { throw 'Defender scanner was not found.' }
function Scan-Launcher([string]$Name) {
    $result=& $scanner.FullName -Scan -ScanType 3 -File $Launcher 2>&1
    $scanExit=$LASTEXITCODE
    $result | Set-Content -LiteralPath (Join-Path $OutputDirectory "$Name.txt")
    if ($scanExit -ne 0 -or ($result -join "`n") -notmatch 'found no threats') { throw 'Launcher Defender scan failed.' }
}
Scan-Launcher 'defender-before'
# A GUI subsystem prevents a console flash, regardless of how the launcher is opened.
$reader=[IO.BinaryReader]::new([IO.File]::OpenRead($Launcher))
try {
    $reader.BaseStream.Position=0x3c
    $pe=$reader.ReadInt32()
    $reader.BaseStream.Position=$pe+92
    if ($reader.ReadUInt16() -ne 2) { throw 'Launcher is not a Windows GUI executable.' }
} finally { $reader.Dispose() }
$metadata=[Diagnostics.FileVersionInfo]::GetVersionInfo($Launcher)
if ($metadata.ProductVersion -ne $Version) { throw 'Launcher version metadata is incorrect.' }
Add-Type -TypeDefinition @'
using System.Runtime.InteropServices;
public static class RookRainbowQuestIconCheck {
    [DllImport("shell32.dll", CharSet=CharSet.Unicode, EntryPoint="ExtractIconExW")]
    public static extern uint Count(string file, int index, System.IntPtr large, System.IntPtr small, uint size);
}
'@
if ([RookRainbowQuestIconCheck]::Count($Launcher,-1,[IntPtr]::Zero,[IntPtr]::Zero,0) -lt 1) { throw 'Launcher icon is missing.' }
$game=Join-Path (Split-Path -Parent $Launcher) "RookRainbowQuest-$Version\RookRainbowQuest.exe"
if ([RookRainbowQuestIconCheck]::Count($game,-1,[IntPtr]::Zero,[IntPtr]::Zero,0) -lt 1) { throw 'Game icon is missing.' }
function Test-Launch([string]$Program,[string]$Name) {
    $report=Join-Path $OutputDirectory "$Name-child-exit.txt"
    $process=Start-Process -FilePath $Program -ArgumentList '--exit-test','--exit-windowed','--exit-report',('"'+$report+'"') -WorkingDirectory $OutputDirectory -WindowStyle Hidden -RedirectStandardOutput (Join-Path $OutputDirectory "$Name-stdout.txt") -RedirectStandardError (Join-Path $OutputDirectory "$Name-stderr.txt") -PassThru
    if (-not $process.WaitForExit(10000)) {
        try { $process.Kill() } catch { Write-Warning $_.Exception.Message }
        throw 'Launcher did not finish handing off to the game.'
    }
    if ($process.ExitCode -ne 0) { throw "Launcher failed ($($process.ExitCode))." }
    $deadline=[DateTime]::UtcNow.AddSeconds(10)
    while (-not (Test-Path -LiteralPath $report) -and [DateTime]::UtcNow -lt $deadline) { Start-Sleep -Milliseconds 100 }
    if (-not (Test-Path -LiteralPath $report) -or (Get-Content -Raw -LiteralPath $report) -ne 'graceful-exit-ok') { throw 'Game did not start and exit through the launcher.' }
}
Test-Launch $Launcher 'adjacent-release'
$fixture=Join-Path $OutputDirectory 'Start från en mapp with spaces'
New-Item -ItemType Directory -Path (Join-Path $fixture "RookRainbowQuest-$Version") -Force | Out-Null
$fixtureLauncher=Join-Path $fixture 'Play Rook Rainbow Quest.exe'
Copy-Item -LiteralPath $Launcher -Destination $fixtureLauncher
Copy-Item -LiteralPath $game -Destination (Join-Path $fixture "RookRainbowQuest-$Version\RookRainbowQuest.exe")
Test-Launch $fixtureLauncher 'unicode-folder'
# The ZIP keeps the launcher beside RookRainbowQuest.exe, rather than above a release folder.
Test-Launch (Join-Path (Split-Path -Parent $game) 'Play Rook Rainbow Quest.exe') 'extracted-layout'
Scan-Launcher 'defender-after'
try {
    $events=@(Get-WinEvent -FilterHashtable @{LogName='Microsoft-Windows-Windows Defender/Operational'; Id=1116} -ErrorAction Stop | Where-Object {
        $_.TimeCreated -ge $started -and ($_.Message.IndexOf($Launcher,[StringComparison]::OrdinalIgnoreCase) -ge 0 -or $_.Message.IndexOf($fixture,[StringComparison]::OrdinalIgnoreCase) -ge 0)
    })
} catch {
    if ($_.FullyQualifiedErrorId -notlike 'NoMatchingEventsFound*') { throw }
    $events=@()
}
if ($events.Count) { throw 'Defender real-time protection detected the launcher candidate.' }
if ((Get-FileHash -LiteralPath $Launcher).Hash -ne $hash) { throw 'Launcher changed during validation.' }
"Launcher passed: GUI subsystem, icon/version resources, adjacent release folder, extracted ZIP layout, Unicode/spaced folder, graceful child exit and pre/post scans. SHA-256: $hash" | Set-Content -LiteralPath (Join-Path $OutputDirectory 'validation.txt')
Write-Host 'Passed: native launcher, icon, version, relative paths and game handoff.'
