#!/bin/bash

LOCK=compiled.lock
LINUX_REPO=https://github.com/torvalds/linux.git

# Clone the latest release candidate (For testing only)
[ -d linux ] || git clone --depth 1 $LINUX_REPO

# Setup linux for building
pushd ./linux
if [ ! -f $LOCK ]; then
    make menuconfig # General setup --> Rust support [*]
    make rustavailable && make -j$(nproc) # and wait...
    touch $LOCK
fi
popd
