Stop-Process -Name catisen -Force -ErrorAction SilentlyContinue
# Headless Testing Utilities
# Provides cross-script headless execution and resource monitoring

function Get-SystemMetrics {
    <#
    .SYNOPSIS
    Captures current system resource usage (CPU, RAM, GPU)
    #>
    $metrics = @{
        Timestamp = Get-Date
        CPUPercent = 0
        RAMPercent = 0
        GPUPercent = 0
    }
    
    try {
        # CPU usage (last second average)
        $cpuCounter = Get-WmiObject win32_processor | Measure-Object -Property LoadPercentage -Average
        $metrics.CPUPercent = [math]::Round($cpuCounter.Average, 2)
    } catch {
        $metrics.CPUPercent = -1  # unavailable
    }
    
    try {
        # RAM usage
        $memInfo = Get-WmiObject win32_operatingsystem
        $usedMemMB = ($memInfo.TotalVisibleMemorySize - $memInfo.FreePhysicalMemory) / 1KB
        $totalMemMB = $memInfo.TotalVisibleMemorySize / 1KB
        $metrics.RAMPercent = [math]::Round(($usedMemMB / $totalMemMB) * 100, 2)
    } catch {
        $metrics.RAMPercent = -1
    }
    
    try {
        # GPU usage via nvidia-smi if available
        $smiOutput = nvidia-smi --query-gpu=utilization.gpu --format=csv,noheader,nounits 2>$null
        if ($smiOutput) {
            $metrics.GPUPercent = [int]$smiOutput[0]
        }
    } catch {
        $metrics.GPUPercent = 0  # No GPU or nvidia-smi unavailable
    }
    
    return $metrics
}

function Get-FormattedTimestamps {
    <#
    .SYNOPSIS
    Returns current time in both IST and Pacific time
    #>
    $utcNow = [System.DateTime]::UtcNow
    
    # IST (UTC+5:30)
    $istTz = [System.TimeZoneInfo]::FindSystemTimeZoneById("India Standard Time")
    $istTime = [System.TimeZoneInfo]::ConvertTime($utcNow, $istTz)
    
    # Pacific (UTC-7 or UTC-8 depending on DST)
    $pstTz = [System.TimeZoneInfo]::FindSystemTimeZoneById("Pacific Standard Time")
    $pstTime = [System.TimeZoneInfo]::ConvertTime($utcNow, $pstTz)
    
    # Unix timestamp (seconds since epoch)
    $epoch = [System.DateTime]::new(1970, 1, 1, 0, 0, 0, [System.DateTimeKind]::Utc)
    $unixTime = [int64]($utcNow - $epoch).TotalSeconds
    
    return @{
        IST = $istTime.ToString("yyyy-MM-dd HH:mm:ss.fff zzz")
        PST = $pstTime.ToString("yyyy-MM-dd HH:mm:ss.fff zzz")
        Unix = $unixTime
    }
}

function Kill-AllCatisenProcesses {
    <#
    .SYNOPSIS
    Forcefully kills all running Catisen and related processes
    #>
    Write-Host "Cleaning up residual processes..." -ForegroundColor Yellow
    
    try {
        Get-Process -Name "catisen" -ErrorAction SilentlyContinue | ForEach-Object {
            Write-Host "  Killing catisen PID $($_.Id)" -ForegroundColor Gray
            Stop-Process -Id $_.Id -Force -ErrorAction SilentlyContinue
        }
    } catch {
        # Silently continue if no processes found
    }
    
    # Give time for cleanup
    Start-Sleep -Milliseconds 200
}

function Start-CatisenHeadless {
    <#
    .SYNOPSIS
    Starts Catisen in headless mode with environment variables
    
    .PARAMETER CatisenPath
    Path to the catisen.exe executable
    
    .PARAMETER Url
    Initial URL to load
    
    .PARAMETER Tor
    Whether to enable Tor
    
    .PARAMETER GeoLocation
    Geographic spoofing location
    
    .PARAMETER Profile
    Browser profile to emulate
    
    .PARAMETER ViewMode
    View mode (source, visual, text)
    #>
    param(
        [string]$CatisenPath,
        [string]$Url,
        [bool]$Tor = $false,
        [string]$GeoLocation = "Disabled",
        [string]$Profile = "Windows",
        [string]$ViewMode = "source",
        [string]$TorProxy = "socks5h://127.0.0.1:9150"
    )
    
    $env:CATISEN_TEST_MODE = "1"
    $env:CATISEN_LOG_FILE = "target/catisen-debug.log"
    $env:CATISEN_GEO_LOCATION = $GeoLocation
    $env:CATISEN_BROWSER_PROFILE = $Profile
    $env:CATISEN_VIEW_MODE = $ViewMode
    $env:CATISEN_TOR_PROXY = $TorProxy
    
    $args = @("--url", $url, "--headless")
    if ($Tor) {
        $args += "--tor"
    }
    
    # Start process with Hidden window style
    $startTime = Get-Date
    $proc = Start-Process -FilePath $CatisenPath -ArgumentList $args -WindowStyle Hidden -PassThru
    
    return @{
        Process = $proc
        StartTime = $startTime
        URL = $Url
    }
}

function Wait-CatisenCompletion {
    <#
    .SYNOPSIS
    Waits for Catisen to complete loading a URL with timeout
    #>
    param(
        [int]$ProcessId,
        [string]$DebugLogPath,
        [string]$Url,
        [int]$TimeoutSec,
        [int]$StartLogCount
    )
    
    $hostFragment = ""
    try {
        $hostFragment = ([uri]$Url).Host.ToLowerInvariant()
    } catch {
        $hostFragment = $Url.ToLowerInvariant()
    }
    
    $deadline = (Get-Date).AddSeconds($TimeoutSec)
    while ((Get-Date) -lt $deadline) {
        Start-Sleep -Milliseconds 350
        
        $content = Get-Content -Path $DebugLogPath -ErrorAction SilentlyContinue
        if ($null -eq $content) { continue }
        $tail = $content | Select-Object -Skip $StartLogCount
        
        foreach ($line in $tail) {
            if ($line -match '^\[REQ\]') {
                $lowerLine = $line.ToLowerInvariant()
                if ($line.Contains($Url) -or $lowerLine.Contains($hostFragment) -or $lowerLine.Contains("mode=error")) {
                    return @{
                        Found = $true
                        LogLine = $line
                        Timestamp = (Get-Date)
                    }
                }
            }
        }
        
        if (-not (Get-Process -Id $ProcessId -ErrorAction SilentlyContinue)) {
            return @{
                Found = $false
                Timestamp = (Get-Date)
                Error = "Process exited before log entry"
            }
        }
    }
    
    return @{
        Found = $false
        Timestamp = (Get-Date)
        Error = "Timeout waiting for telemetry"
    }
}

function Cleanup-CatisenProcess {
    <#
    .SYNOPSIS
    Ensures a Catisen process is cleaned up
    #>
    param(
        [int]$ProcessId
    )
    
    $proc = Get-Process -Id $ProcessId -ErrorAction SilentlyContinue
    if ($proc) {
        if (-not $proc.HasExited) {
            Stop-Process -Id $ProcessId -Force -ErrorAction SilentlyContinue
        }
    }
    
    # Verify cleanup
    Start-Sleep -Milliseconds 100
    if (Get-Process -Id $ProcessId -ErrorAction SilentlyContinue) {
        # Aggressive cleanup
        Kill-AllCatisenProcesses
    }
}

function Parse-RequestMetrics {
    <#
    .SYNOPSIS
    Parses a [REQ] log line and extracts metrics
    #>
    param(
        [string]$LogLine
    )
    
    $metrics = @{
        Success = $false
        Status = 0
        Bytes = 0
        Mode = "unknown"
        Tor = $false
        TTFB = 0
        TTFC = 0
        TTFr = 0
        TTI = 0
        FullLoad = 0
        URL = ""
        Headers = ""
        RawLine = $LogLine
    }
    
    if ($LogLine -match '\[REQ\]') {
        # Parse key=value pairs
        $fields = @{
            Status = [regex]::Match($LogLine, 'status=(\d+)').Groups[1].Value
            Bytes = [regex]::Match($LogLine, 'bytes=(\d+)').Groups[1].Value
            Mode = [regex]::Match($LogLine, 'mode=(\w+)').Groups[1].Value
            Tor = [regex]::Match($LogLine, 'tor=(\w+)').Groups[1].Value
            TTFB = [regex]::Match($LogLine, 'ttfb=(\d+)').Groups[1].Value
            TTFC = [regex]::Match($LogLine, 'ttfc=(\d+)').Groups[1].Value
            TTFr = [regex]::Match($LogLine, 'ttfr=(\d+)').Groups[1].Value
            TTI = [regex]::Match($LogLine, 'tti=(\d+)').Groups[1].Value
            FullLoad = [regex]::Match($LogLine, 'full=(\d+)').Groups[1].Value
            URL = [regex]::Match($LogLine, 'url=(\S+)').Groups[1].Value
            Headers = [regex]::Match($LogLine, 'headers=(.+)$').Groups[1].Value
        }
        
        $metrics.Status = [int]$fields.Status
        $metrics.Bytes = [int]$fields.Bytes
        $metrics.Mode = $fields.Mode
        $metrics.Tor = ($fields.Tor -eq "true")
        $metrics.TTFB = [int]$fields.TTFB
        $metrics.TTFC = [int]$fields.TTFC
        $metrics.TTFr = [int]$fields.TTFr
        $metrics.TTI = [int]$fields.TTI
        $metrics.FullLoad = [int]$fields.FullLoad
        $metrics.URL = $fields.URL
        $metrics.Headers = $fields.Headers
        $metrics.Success = $true
    }
    
    return $metrics
}

Export-ModuleMember -Function @(
    'Get-SystemMetrics',
    'Get-FormattedTimestamps',
    'Kill-AllCatisenProcesses',
    'Start-CatisenHeadless',
    'Wait-CatisenCompletion',
    'Cleanup-CatisenProcess',
    'Parse-RequestMetrics'
) -ErrorAction SilentlyContinue
