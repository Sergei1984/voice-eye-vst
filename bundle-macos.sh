#!/bin/bash

# Modified lightly from
# https://github.com/RustAudio/vst-rs/blob/master/osx_vst_bundler.sh


SYNTH_NAME=VoiceEye
BUILD_OUTPUT=target/libvoice_eye.dylib

cargo build --target x86_64-apple-darwin --release && cargo build --target aarch64-apple-darwin --release

if [ $? -ne 0 ]
then
    echo "Build failed"
    exit -1
fi

rm -r "${SYNTH_NAME}.vst"

lipo -create ./target/x86_64-apple-darwin/release/libvoice_eye.dylib  ./target/aarch64-apple-darwin/release/libvoice_eye.dylib -output ${BUILD_OUTPUT}

# Make the bundle folder
mkdir -p "$SYNTH_NAME.vst/Contents/MacOS"

# Create the PkgInfo
echo "VoiceEyeBNDL" > "$SYNTH_NAME.vst/Contents/PkgInfo"

#build the Info.Plist
echo "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">
<plist version=\"1.0\">
<dict>
    <key>CFBundleDevelopmentRegion</key>
    <string>English</string>

    <key>CFBundleExecutable</key>
    <string>$SYNTH_NAME</string>

    <key>CFBundleGetInfoString</key>
    <string>vst</string>

    <key>CFBundleIconFile</key>
    <string></string>

    <key>CFBundleIdentifier</key>
    <string>com.serhiitokariev.$SYNTH_NAME</string>

    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>

    <key>CFBundleName</key>
    <string>$SYNTH_NAME</string>

    <key>CFBundlePackageType</key>
    <string>BNDL</string>

    <key>CFBundleVersion</key>
    <string>1.0</string>

    <key>CFBundleSignature</key>
    <string>$((RANDOM % 9999))</string>

    <key>CSResourcesFileMapped</key>
    <string></string>

</dict>
</plist>" > "$SYNTH_NAME.vst/Contents/Info.plist"

# move the provided library to the correct location
cp "$BUILD_OUTPUT" "$SYNTH_NAME.vst/Contents/MacOS/$SYNTH_NAME"

sudo cp -r "$SYNTH_NAME.vst" "/Library/Audio/Plug-Ins/VST/"

echo "Created bundle $SYNTH_NAME.vst"

