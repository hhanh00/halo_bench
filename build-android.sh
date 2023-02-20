FLUTTER_VERSION=3.7.3
pushd $HOME

curl https://dl.google.com/android/repository/commandlinetools-linux-7583922_latest.zip --output cmd-tools.zip
mkdir -p Android/sdk
export ANDROID_SDK_ROOT=$HOME/Android/sdk
export ANDROID_NDK_ROOT=$ANDROID_SDK_ROOT/ndk/25.1.8937393
export ANDROID_NDK_HOME=$ANDROID_NDK_ROOT

unzip cmd-tools.zip
pushd cmdline-tools/bin
yes | ./sdkmanager --sdk_root=$ANDROID_SDK_ROOT --licenses
yes | ./sdkmanager --sdk_root=$ANDROID_SDK_ROOT "platform-tools" "cmdline-tools;latest"
yes | ./sdkmanager --sdk_root=$ANDROID_SDK_ROOT "build-tools;30.0.3" "cmake;3.18.1"
yes | ./sdkmanager --sdk_root=$ANDROID_SDK_ROOT "ndk;25.1.8937393"
yes | ./sdkmanager --sdk_root=$ANDROID_SDK_ROOT "platforms;android-33"
popd

git clone -b $FLUTTER_VERSION --depth 1 https://github.com/flutter/flutter.git flutter
export PATH=$PATH:$HOME/flutter/bin
flutter doctor -v

rustup target add aarch64-linux-android armv7-linux-androideabi
cargo install --force cargo-ndk
popd

flutter build apk

mv build/app/outputs/flutter-apk/app-release.apk halo-bench.apk

echo "BUILD_DIR=$PWD" >> $GITHUB_OUTPUT
