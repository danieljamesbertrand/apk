# Build the Ember Android app and produce an APK.
# Requires: Rust, cargo-ndk, Android SDK/NDK (via Android Studio)

$ErrorActionPreference = "Stop"
$rootDir = Split-Path -Parent $MyInvocation.MyCommand.Path

# Build Rust library for Android
Write-Host "Building Rust library for Android..." -ForegroundColor Cyan
$jniLibs = Join-Path $rootDir "android\app\src\main\jniLibs"
New-Item -ItemType Directory -Force -Path $jniLibs | Out-Null

Push-Location $rootDir
try {
    cargo ndk -t arm64-v8a -t armeabi-v7a -o $jniLibs build -p ember-client --features android --release
    if ($LASTEXITCODE -ne 0) { throw "cargo ndk failed" }
} finally {
    Pop-Location
}

# Build Android APK
Write-Host "`nBuilding Android APK..." -ForegroundColor Cyan
Push-Location (Join-Path $rootDir "android")
try {
    ./gradlew assembleRelease
    if ($LASTEXITCODE -ne 0) { throw "gradle build failed" }
} finally {
    Pop-Location
}

$apkPath = Join-Path $rootDir "android\app\build\outputs\apk\release\app-release-unsigned.apk"
Write-Host "`nDone! APK: $apkPath" -ForegroundColor Green
Write-Host "For distribution, sign the APK (e.g. with jarsigner or Android Studio)." -ForegroundColor Yellow
