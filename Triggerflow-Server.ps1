Add-Type -Name User32 -Namespace Win32 -MemberDefinition @"
[DllImport("user32.dll", CharSet=CharSet.Auto, CallingConvention=CallingConvention.StdCall)]
public static extern void mouse_event(uint dwFlags, uint dx, uint dy, uint cButtons, uint dwExtraInfo);
"@

$port = 1337
$udpClient = New-Object System.Net.Sockets.UdpClient($port)

# Function to simulate mouse click
function SimulateMouseClick {
    $mouseLeftButtonDown = 0x0002
    $mouseLeftButtonUp = 0x0004
    [Win32.User32]::mouse_event($mouseLeftButtonDown, 0, 0, 0, 0)
    [Win32.User32]::mouse_event($mouseLeftButtonUp, 0, 0, 0, 0)
}

# Function to handle UDP packet with timestamp
function HandleUdpPacket {
    param (
        [byte[]]$data
    )

    # Emulate mouse click
    SimulateMouseClick

    # Print received data (optional)
    $receivedData = [System.Text.Encoding]::ASCII.GetString($data)
    Write-Host "Received UDP packet with data: $receivedData"
}

# Asynchronously receive UDP packets
$asyncResult = $udpClient.BeginReceive($null, $null)

try {
    Write-Host "UDP Server is listening on port $port..."

    while ($true) {
        # Check if UDP packet has been received
        if ($asyncResult.AsyncWaitHandle.WaitOne(0)) {
            $remoteEndPoint = New-Object System.Net.IPEndPoint ([System.Net.IPAddress]::Any, 0)
            $receivedData = $udpClient.EndReceive($asyncResult, [ref]$remoteEndPoint)
            
            # Handle UDP packet
            HandleUdpPacket -data $receivedData
            
            # Restart asynchronous receive
            $asyncResult = $udpClient.BeginReceive($null, $null)
        } else {
            # Sleep for a short duration to reduce CPU usage
            Start-Sleep -Milliseconds 1
        }
    }
}
catch {
    Write-Host "Error occurred: $_"
}
finally {
    $udpClient.Close()
}
