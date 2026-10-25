param(
    [Parameter(Mandatory=$true)][string]$Executable,
    [Parameter(Mandatory=$true)][string]$OutputDirectory
)
$ErrorActionPreference = 'Stop'
$Executable = (Resolve-Path -LiteralPath $Executable).Path
New-Item -ItemType Directory -Path $OutputDirectory -Force | Out-Null
$OutputDirectory = (Resolve-Path -LiteralPath $OutputDirectory).Path
$cases = @(
    @{Name='dead-position'; Arguments=@('--dead-test','--screenshots',('"'+$OutputDirectory+'"')); Timeout=20000},
    @{Name='regular-chess'; Arguments=@('--chess-test','--screenshots',('"'+$OutputDirectory+'"')); Timeout=20000},
    @{Name='checkmate'; Arguments=@('--mate-test','--screenshots',('"'+$OutputDirectory+'"')); Timeout=20000},
    @{Name='menu'; Arguments=@('--menu-test','--screenshots',('"'+$OutputDirectory+'"')); Timeout=20000},
    @{Name='retry'; Arguments=@('--retry-test','--screenshots',('"'+$OutputDirectory+'"')); Timeout=10000},
    @{Name='settings'; Arguments=@('--settings-test','--screenshots',('"'+$OutputDirectory+'"')); Timeout=60000},
    @{Name='exit-windowed'; Arguments=@('--exit-test','--exit-windowed'); Timeout=10000},
    @{Name='exit-fullscreen'; Arguments=@('--exit-test'); Timeout=10000}
)
foreach ($case in $cases) {
    $report = Join-Path $OutputDirectory "$($case.Name)-passed.txt"
    if (Test-Path -LiteralPath $report) { Remove-Item -LiteralPath $report }
    $arguments = @($case.Arguments) + @('--exit-report',('"'+$report+'"'))
    $process = Start-Process -FilePath $Executable -ArgumentList $arguments -WorkingDirectory (Split-Path -Parent $Executable) -WindowStyle Hidden -RedirectStandardOutput (Join-Path $OutputDirectory "$($case.Name)-stdout.txt") -RedirectStandardError (Join-Path $OutputDirectory "$($case.Name)-stderr.txt") -PassThru
    if (-not $process.WaitForExit($case.Timeout)) {
        # This is the helper launched immediately above; never target unrelated games.
        try { $process.Kill() } catch { Write-Warning $_.Exception.Message }
        throw "$($case.Name) froze or exceeded its timeout."
    }
    if ($process.ExitCode -ne 0) { throw "$($case.Name) failed with exit code $($process.ExitCode)." }
    if (-not (Test-Path -LiteralPath $report)) { throw "$($case.Name) exited without passing its handler checks." }
    if ((Get-Content -Raw -LiteralPath $report) -ne 'graceful-exit-ok') { throw "$($case.Name) produced an invalid report." }
    Write-Host "Passed: $($case.Name) (normal process exit)."
}
