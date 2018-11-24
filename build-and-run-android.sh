#!/bin/bash

./build-android-toolchain.sh ~/dev/android-ndk/android-ndk-r17b ~/dev/android-ndk ~/dev/android-sdk
~/dev/android-sdk/platform-tools/adb install ./android/Minigame/app/build/outputs/apk/app-debug.apk
