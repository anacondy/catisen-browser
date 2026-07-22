$b = [IO.File]::ReadAllText('fixes.b64')
$x = ($b -replace '\s','')
$bytes = [Convert]::FromBase64String($x)
[IO.File]::WriteAllBytes('fixes.patch', $bytes)
'BYTES=' + (Get-Item fixes.patch).Length
'SHA256=' + (Get-FileHash fixes.patch -Algorithm SHA256).Hash