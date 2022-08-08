#!/usr/bin/env pwsh

<#
.Synopsis
    Set the blinking period of a running instance of the rust-blinky-pac-tokio
    application.
.Parameter HostName
    Specifies the host name.
.Parameter Period
    Specifies the new period in microseconds.
#>

[CmdletBinding()]
param(
    [string]$HostName = $(throw "The HostName parameter is required."),
    [int]$Period = $(throw "The Period parameter is required.")
)

$periodString = [System.Text.Encoding]::UTF8.GetBytes($Period.ToString())

[System.Net.Sockets.UdpClient]::new().Send($periodString, $periodString.Length, $HostName, 52000)
