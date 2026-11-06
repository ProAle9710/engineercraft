# Bumps patch version, builds release, packs MSI.
# -Publish: crea GitHub Release vX.Y.Z y sube el MSI.
# Antes de publicar: entrada nueva en src/news.rs.
param([switch]$Publish)
$ErrorActionPreference = "Stop"
$root = Split-Path -Parent $PSScriptRoot
Set-Location $root

$toml = Get-Content (Join-Path $root "Cargo.toml") -Raw
if ($toml -notmatch '(?m)^version\s*=\s*"(\d+)\.(\d+)\.(\d+)"') {
    throw "No encontre version en Cargo.toml"
}
$major = [int]$Matches[1]
$minor = [int]$Matches[2]
$patch = [int]$Matches[3] + 1
$version = "$major.$minor.$patch"
$toml = [regex]::Replace($toml, '(?m)^version\s*=\s*"\d+\.\d+\.\d+"', "version = `"$version`"")
Set-Content -Path (Join-Path $root "Cargo.toml") -Value $toml -NoNewline
Write-Host "version -> $version"

cargo build --release
if ($LASTEXITCODE -ne 0) { throw "cargo build failed" }

$meta = cargo metadata --format-version 1 --no-deps | ConvertFrom-Json
$exe = Join-Path $meta.target_directory "release\javacraft.exe"
if (-not (Test-Path $exe)) { throw "no exe: $exe" }

$stage = Join-Path $root "dist\stage"
if (Test-Path $stage) { Remove-Item $stage -Recurse -Force }
New-Item -ItemType Directory -Path (Join-Path $stage "exercises") | Out-Null
Copy-Item $exe (Join-Path $stage "javacraft.exe")
Copy-Item (Join-Path $root "exercises\*") (Join-Path $stage "exercises") -Recurse

function New-Id([string]$s) {
    $h = [System.BitConverter]::ToString([System.Security.Cryptography.SHA1]::Create().ComputeHash([Text.Encoding]::UTF8.GetBytes($s))).Replace("-", "").Substring(0, 16)
    return "I$h"
}

$icon = Join-Path $root "assets\icon.ico"
$sb = New-Object System.Text.StringBuilder
[void]$sb.AppendLine('<?xml version="1.0" encoding="UTF-8"?>')
[void]$sb.AppendLine('<Wix xmlns="http://wixtoolset.org/schemas/v4/wxs">')
[void]$sb.AppendLine("  <Package Name=`"JavaCraft`" Manufacturer=`"Alejandro Infante`" Version=`"$version`" UpgradeCode=`"7F3A1C2E-9B44-4D18-A6E1-2C8F0B7D4A11`" Scope=`"perMachine`">")
[void]$sb.AppendLine('    <MajorUpgrade AllowSameVersionUpgrades="yes" DowngradeErrorMessage="Ya hay una version mas nueva de JavaCraft." />')
[void]$sb.AppendLine('    <MediaTemplate EmbedCab="yes" />')
[void]$sb.AppendLine("    <Icon Id=`"AppIcon`" SourceFile=`"$icon`" />")
[void]$sb.AppendLine('    <Property Id="ARPPRODUCTICON" Value="AppIcon" />')
[void]$sb.AppendLine('    <StandardDirectory Id="ProgramFiles64Folder">')
[void]$sb.AppendLine('      <Directory Id="INSTALLFOLDER" Name="JavaCraft">')
[void]$sb.AppendLine('        <Directory Id="EXERCISES" Name="exercises">')

function Emit-Dir($fsPath, $indent) {
    Get-ChildItem $fsPath -Directory | Sort-Object Name | ForEach-Object {
        $id = New-Id $_.FullName
        [void]$script:sb.AppendLine("$indent<Directory Id=`"$id`" Name=`"$($_.Name)`">")
        Emit-Dir $_.FullName ("$indent  ")
        Get-ChildItem $_.FullName -File | Sort-Object Name | ForEach-Object {
            $cid = New-Id $_.FullName
            [void]$script:sb.AppendLine("$indent  <Component Id=`"$cid`" Guid=`"*`">")
            [void]$script:sb.AppendLine("$indent    <File Source=`"$($_.FullName)`" KeyPath=`"yes`" />")
            [void]$script:sb.AppendLine("$indent  </Component>")
        }
        [void]$script:sb.AppendLine("$indent</Directory>")
    }
}
Emit-Dir (Join-Path $stage "exercises") "          "
[void]$sb.AppendLine('        </Directory>')
[void]$sb.AppendLine('        <Component Id="AppExe" Guid="*">')
[void]$sb.AppendLine("          <File Id=`"JavaCraftExe`" Source=`"$stage\javacraft.exe`" KeyPath=`"yes`">")
[void]$sb.AppendLine('            <Shortcut Name="JavaCraft" Directory="ProgramMenuFolder" Advertise="no" WorkingDirectory="INSTALLFOLDER" Icon="AppIcon" />')
[void]$sb.AppendLine('          </File>')
[void]$sb.AppendLine('        </Component>')
[void]$sb.AppendLine('      </Directory>')
[void]$sb.AppendLine('    </StandardDirectory>')
[void]$sb.AppendLine('    <Feature Id="Main" Title="JavaCraft" Level="1">')
[void]$sb.AppendLine('      <ComponentGroupRef Id="All" />')
[void]$sb.AppendLine('    </Feature>')
[void]$sb.AppendLine('    <ComponentGroup Id="All" Directory="INSTALLFOLDER">')
[void]$sb.AppendLine('      <ComponentRef Id="AppExe" />')
[void]$sb.AppendLine('    </ComponentGroup>')
[void]$sb.AppendLine('  </Package>')
[void]$sb.AppendLine('</Wix>')

$wxs = $sb.ToString()
$refs = [regex]::Matches($wxs, 'Component Id="(I[A-F0-9]+)"') | ForEach-Object { $_.Groups[1].Value } | Select-Object -Unique
$refXml = ($refs | ForEach-Object { "      <ComponentRef Id=`"$_`" />" }) -join "`n"
$wxs = $wxs.Replace("      <ComponentRef Id=`"AppExe`" />", "      <ComponentRef Id=`"AppExe`" />`n$refXml")
$wixDir = Join-Path $root "wix"
New-Item -ItemType Directory -Force -Path $wixDir | Out-Null
Set-Content -Path (Join-Path $wixDir "JavaCraft.wxs") -Value $wxs -Encoding UTF8

$env:Path = [System.Environment]::GetEnvironmentVariable("Path", "Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path", "User")
$out = Join-Path $root "dist\JavaCraft-$version.msi"
New-Item -ItemType Directory -Force -Path (Join-Path $root "dist") | Out-Null
wix build (Join-Path $wixDir "JavaCraft.wxs") -arch x64 -out $out
if ($LASTEXITCODE -ne 0) { throw "wix build failed" }
Get-Item $out | Format-List FullName, Length
Write-Host "MSI $version listo: $out"

if ($Publish) {
    gh release create "v$version" $out --repo ProAle9710/javacraft --title "JavaCraft $version" --notes "Instalador Windows. Actualiza una instalacion previa (MajorUpgrade)."
    if ($LASTEXITCODE -ne 0) { throw "gh release create failed" }
    Write-Host "Release v$version publicado"
}
