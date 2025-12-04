<#
PowerShell helper for `todo_cli_app`.

Usage (temporary for current session):
. .\scripts\todo.ps1      # dot-source to load the functions into your session
Add-Task "Buy milk and eggs"

Or use the proxy function once loaded:
todo add "Buy milk and eggs"

The script will try to find `target\debug\todo_cli_app.exe` or `target\release\todo_cli_app.exe` in the parent folder (the project root).
If the binary is missing the script will run `cargo build` in the project root to build it.
#>

function Get-TodoExePath {
    $scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Definition
    $repoDir = (Resolve-Path "$scriptDir\..").Path
    $debugExe = Join-Path $repoDir 'target\debug\todo_cli_app.exe'
    if (Test-Path $debugExe) { return $debugExe }
    $releaseExe = Join-Path $repoDir 'target\release\todo_cli_app.exe'
    if (Test-Path $releaseExe) { return $releaseExe }
    return $null
}

function Build-TodoIfMissing {
    param([string]$RepoDir)
    $exe = Get-TodoExePath
    if ($exe) { return $exe }
    Write-Host "Binary not found; running `"cargo build`" in $RepoDir ..."
    Push-Location $RepoDir
    cargo build
    $buildResult = $LASTEXITCODE
    Pop-Location
    if ($buildResult -ne 0) { return $null }
    return Get-TodoExePath
}

function Add-Task {
    param([Parameter(Mandatory=$true)][string]$Text)
    $scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Definition
    $repoDir = (Resolve-Path "$scriptDir\..").Path
    $exe = Build-TodoIfMissing -RepoDir $repoDir
    if (-not $exe) { Write-Error "Could not find or build the todo binary."; return }
    & $exe add $Text
}

function todo {
    param([Parameter(ValueFromRemainingArguments = $true)][string[]]$Args)
    $scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Definition
    $repoDir = (Resolve-Path "$scriptDir\..").Path
    $exe = Build-TodoIfMissing -RepoDir $repoDir
    if (-not $exe) { Write-Error "Could not find or build the todo binary."; return }
    & $exe @Args
}
