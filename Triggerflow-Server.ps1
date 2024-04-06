Add-Type -Name User32 -Namespace Win32 -MemberDefinition @"
[DllImport("user32.dll", CharSet=CharSet.Auto, CallingConvention=CallingConvention.StdCall)]
public static extern void mouse_event(uint dwFlags, uint dx, uint dy, uint cButtons, uint dwExtraInfo);
"@

$port = 1337

$udpClient = New-Object System.Net.Sockets.UdpClient($port)

function SimulateMouseClick {
$mouseLeftButtonDown = 0x0002
$mouseLeftButtonUp = 0x0004
[Win32.User32]::mouse_event($mouseLeftButtonDown, 0, 0, 0, 0)
Start-Sleep -Milliseconds 100
[Win32.User32]::mouse_event($mouseLeftButtonUp, 0, 0, 0, 0)
}

function HandleUdpPacket {
param (
[byte[]]$data
)

$receivedData = [System.Text.Encoding]::ASCII.GetString($data)
Write-Host "Received UDP packet with data: $receivedData"

SimulateMouseClick

}

try {
Write-Host "UDP Server is listening on port $port..."

while ($true) {
    $remoteEndPoint = New-Object System.Net.IPEndPoint ([System.Net.IPAddress]::Any, 0)
    $receivedData = $udpClient.Receive([ref]$remoteEndPoint)

    HandleUdpPacket -data $receivedData
}

}
catch {
Write-Host "Error occurred: $_.Exception.Message"
}
finally {
$udpClient.Close()
}
