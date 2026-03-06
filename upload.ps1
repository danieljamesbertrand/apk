# Build Ember APK and copy to this repo for upload.
# Run from the apk repo directory. Requires ember at ..\ember

$ErrorActionPreference = "Stop"
$apkDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$emberDir = Join-Path (Split-Path -Parent $apkDir) "ember"
# Or: $emberDir = "d:\rust\ember"

if (-not (Test-Path $emberDir)) {
    Write-Error "Ember project not found at $emberDir"
}

Write-Host "Building APK from $emberDir..." -ForegroundColor Cyan
Push-Location $emberDir
try {
    & .\build-android.ps1
} finally {
    Pop-Location
}

$srcApk = Join-Path $emberDir "android\app\build\outputs\apk\release\app-release-unsigned.apk"
$dstApk = Join-Path $apkDir "ember.apk"

if (Test-Path $srcApk) {
    Copy-Item $srcApk $dstApk -Force
    Write-Host "`nCopied APK to $dstApk" -ForegroundColor Green
    Write-Host "Run: git add ember.apk && git commit -m 'Add APK' && git push" -ForegroundColor Yellow
} else {
    Write-Error "APK not found at $srcApk"
}
