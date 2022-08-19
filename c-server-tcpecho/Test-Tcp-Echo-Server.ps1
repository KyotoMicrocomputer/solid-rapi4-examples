#!/usr/bin/env pwsh

<#
.Synopsis
    Test the operation of a TCP echo server.
.Parameter HostName
    Specifies the host name.
.Parameter Port
    Specifies the port number.
#>

[CmdletBinding()]
param(
    [string]$HostName = $(throw "The HostName parameter is required."),
    [int]$Port = 7777
)
$stream = $null

try {
    $client = [System.Net.Sockets.TcpClient]::new($HostName, $Port)
    $stream = $client.GetStream()

    [byte[]]$buffer = 1..5
    $stream.Write($buffer, 0, $buffer.Length)
    Write-Output "Sent $buffer"

    $i = 0
    while ($i -lt $buffer.Length) {
        $bytesRead = $stream.Read($buffer, $i, $buffer.Length - $i)
        if ($bytesRead -eq 0) {
            throw "Short read ($i)"
        }

        $i += $bytesRead
    }
    Write-Output "Received $buffer"

} finally {
    if ($stream -ne $null) {
        $stream.Close()
    }
    $client.Close()
}
