#!/bin/bash

none=0
while [[ "$#" -gt 0 ]]; do
    case $1 in
        -n|--none) none=1 ;;
        --mac) mac=1 ;;
        --win|--windows) win=1 ;;
        --android) android=1 ;;
        *) echo "Unknown parameter passed: $1"; exit 1 ;;
    esac
    shift
done

export TAURI_SIGNING_PRIVATE_KEY="$(pwd)/keys/flavorapp.key"
export TAURI_SIGNING_PRIVATE_KEY_PASSWORD=$(<"$(pwd)/keys/password.txt")

if [[ $none == 0 ]]; then
    rm -r ./build/out/
    mkdir ./build/out/
fi

if [[ $none == 0 ]] || [[ $mac == 1 ]]; then
    ./build/mac.sh
fi

if [[ $none == 0 ]] || [[ $win == 1 ]]; then
    ./build/win.sh
fi

if [[ $android == 1 ]]; then
    ./build/android.sh
fi

bun ./build/latest.ts