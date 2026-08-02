Stop-Process -Name catisen -Force -ErrorAction SilentlyContinue
param(
    [string]$CatisenPath = ".\target\debug\catisen.exe",
    [string]$TorProxyOverride = "",
    [string]$TorExePath = "",
    [string]$TorrcPath = ".\docs\torrc.example",
    [string]$StartUrl = "",
    [switch]$NoTor
)

$ErrorActionPreference = "Stop"

function Test-LocalPort {
    param([int]$Port)

    try {
        $client = New-Object System.Net.Sockets.TcpClient
        $iar = $client.BeginConnect("127.0.0.1", $Port, $null, $null)
        $ok = $iar.AsyncWaitHandle.WaitOne(400)
        if (-not $ok) {
            $client.Close()
            return $false
        }
        $client.EndConnect($iar)
        $client.Close()
        return $true
    } catch {
        return $false
    }
}

function Resolve-TorProxy {
    param([string]$Override)

    if (-not [string]::IsNullOrWhiteSpace($Override)) {
        return [pscustomobject]@{ Proxy = $Override; Source = "Override" }
    }

    if (Test-LocalPort -Port 9150) {
        return [pscustomobject]@{ Proxy = "socks5h://127.0.0.1:9150"; Source = "Tor Browser" }
    }

    if (Test-LocalPort -Port 9050) {
        return [pscustomobject]@{ Proxy = "socks5h://127.0.0.1:9050"; Source = "Local tor" }
    }

    return $null
}

function Find-TorExe {
    param([string]$Preferred)

    $candidates = @()
    if (-not [string]::IsNullOrWhiteSpace($Preferred)) {
        $candidates += $Preferred
    }

    $candidates += @(
        "$env:ProgramFiles\Tor Browser\Browser\TorBrowser\Tor\tor.exe",
        "$env:ProgramFiles(x86)\Tor Browser\Browser\TorBrowser\Tor\tor.exe",
        "$env:LOCALAPPDATA\Tor Browser\Browser\TorBrowser\Tor\tor.exe",
        ".\tor\tor.exe"
    )

    foreach ($candidate in $candidates) {
        if (-not [string]::IsNullOrWhiteSpace($candidate) -and (Test-Path $candidate)) {
            return $candidate
        }
    }

    return $null
}

if (-not (Test-Path $CatisenPath)) {
    throw "Catisen executable not found at $CatisenPath. Run cargo build first."
}

$torProxy = $null
$torSource = "Disabled"

if (-not $NoTor) {
    $resolved = Resolve-TorProxy -Override $TorProxyOverride
    if ($resolved) {
        $torProxy = $resolved.Proxy
        $torSource = $resolved.Source
    } else {
        $torExe = Find-TorExe -Preferred $TorExePath
        if ($torExe) {
            Write-Host "Starting local tor.exe: $torExe" -ForegroundColor Yellow

            $torArgs = @()
            if (Test-Path $TorrcPath) {
                $torArgs += @("-f", (Resolve-Path $TorrcPath).Path)
            }

            Start-Process -FilePath $torExe -ArgumentList $torArgs -WindowStyle Hidden | Out-Null

            $deadline = [DateTime]::UtcNow.AddSeconds(20)
            while ([DateTime]::UtcNow -lt $deadline) {
                if (Test-LocalPort -Port 9050) {
                    $torProxy = "socks5h://127.0.0.1:9050"
                    $torSource = "Local tor (auto-started)"
                    break
                }
                [System.Threading.Thread]::Sleep(500)
            }
        }
    }
}

$args = @()
if (-not [string]::IsNullOrWhiteSpace($StartUrl)) {
    $args += @("--url", $StartUrl)
}

if (-not [string]::IsNullOrWhiteSpace($torProxy)) {
    $env:CATISEN_TOR_PROXY = $torProxy
    $args += "--tor"
    Write-Host "Tor: ON via $torProxy ($torSource)" -ForegroundColor Green
} else {
    Write-Host "Tor: OFF (no proxy detected/launched)" -ForegroundColor DarkYellow
}

Write-Host "Launching Catisen: $CatisenPath $($args -join ' ')" -ForegroundColor Cyan
Start-Process -FilePath $CatisenPath -ArgumentList $args -WindowStyle Hidden | Out-Null


exit $LASTEXITCODE
