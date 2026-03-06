# Ember APK

Android app for the Ember QUIC client. Download the APK from [Releases](https://github.com/danieljamesbertrand/apk/releases).

## Automated build

This repo includes the Ember source. Push a tag (e.g. `v1.0`) to trigger a build, or run manually: Actions → Build APK → Run workflow. The APK will be attached to the release.

## Build locally

From the ember project directory (`d:\rust\ember`):

```powershell
# 1. Install NDK via Android Studio: SDK Manager → SDK Tools → NDK (Side by side)
# 2. Install cargo-ndk: cargo install cargo-ndk
# 3. Build
.\build-android.ps1
```

The APK is at `ember\android\app\build\outputs\apk\release\app-release-unsigned.apk`.

## Upload to this repo

After building, copy the APK here and push:

```powershell
Copy-Item ..\ember\android\app\build\outputs\apk\release\app-release-unsigned.apk .\ember.apk
git add ember.apk
git commit -m "Add APK"
git push
```

Or create a [GitHub Release](https://github.com/danieljamesbertrand/apk/releases/new) and attach the APK.
