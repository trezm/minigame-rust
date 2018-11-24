#!/bin/bash

###
#
# $1 is the ndk location
# $2 is the output for platform dependent toolchains
# $3 is the android sdk
#
# Example
# ./build-android-toolchain.sh ~/dev/android-ndk/android-ndk-r17b ~/dev/android-ndk ~/dev/android-sdk
# ~/dev/android-sdk/platform-tools/adb install ./android/Minigame/app/build/outputs/apk/app-debug.apk
export ANDROID_NDK_HOME=$1
export ANDROID_HOME=$3
echo "Building toolchain..."
$1/build/tools/make_standalone_toolchain.py --arch arm --install-dir $2/android-ndk-arm
$1/build/tools/make_standalone_toolchain.py --arch arm64 --install-dir $2/android-ndk-arm64
$1/build/tools/make_standalone_toolchain.py --arch x86 --install-dir $2/android-ndk-x86
$1/build/tools/make_standalone_toolchain.py --arch x86_64 --install-dir $2/android-ndk-x86_64

echo "Configuring..."
echo "
[target.armv7-linux-androideabi]
linker = \"$2/android-ndk-arm/bin/arm-linux-androideabi-gcc\"

[target.aarch64-linux-android]
linker = \"$2/android-ndk-arm64/bin/aarch64-linux-android-gcc\"

[target.i686-linux-android]
linker = \"$2/android-ndk-x86/bin/i686-linux-android-gcc\"

[target.x86_64-linux-android]
linker = \"$2/android-ndk-x86_64/bin/x86_64-linux-android-gcc\"
" > ./.cargo/config

echo "Building Android SDL..."
cd android/Minigame/sdl
../gradlew assemble
cd ../../..

echo "Building Rust Library..."
CC=$2/android-ndk-arm/bin/arm-linux-androideabi-gcc CXX=$2/android-ndk-arm/bin/arm-linux-androideabi-clang++ AR=$2/android-ndk-arm/bin/arm-linux-androideabi-ar cargo build --no-default-features --target armv7-linux-androideabi --lib
CC=$2/android-ndk-arm64/bin/aarch64-linux-android-gcc CXX=$2/android-ndk-arm64/bin/aarch64-linux-android-clang++ AR=$2/android-ndk-arm64/bin/aarch64-linux-android-ar cargo build --no-default-features --target aarch64-linux-android --lib
CC=$2/android-ndk-x86/bin/i686-linux-android-gcc CXX=$2/android-ndk-x86/bin/i686-linux-android-clang++ AR=$2/android-ndk-x86/bin/i686-linux-android-ar cargo build --no-default-features --target i686-linux-android --lib
CC=$2/android-ndk-x86_64/bin/x86_64-linux-android-gcc CXX=$2/android-ndk-x86_64/bin/x86_64-linux-android-clang++ AR=$2/android-ndk-x86_64/bin/x86_64-linux-android-ar cargo build --no-default-features --target x86_64-linux-android --lib

echo "Copying files..."
cp target/armv7-linux-androideabi/debug/libminigame.so android/Minigame/app/src/main/jniLibs/armeabi/
cp target/armv7-linux-androideabi/debug/libminigame.so android/Minigame/app/src/main/jniLibs/armeabi-v7a/
cp target/i686-linux-android/debug/libminigame.so android/Minigame/app/src/main/jniLibs/x86/
cp target/x86_64-linux-android/debug/libminigame.so android/Minigame/app/src/main/jniLibs/x86_64/

cp target/aarch64-linux-android/debug/libminigame.so android/Minigame/app/src/main/jniLibs/arm64-v8a/
cp $2/android-ndk-arm64/aarch64-linux-android/lib/libc++_shared.so android/Minigame/app/src/main/jniLibs/arm64-v8a/

echo "Building android app..."
cd android/Minigame/app
../gradlew assemble
cd ../../..
