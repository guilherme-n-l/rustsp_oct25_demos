## demo/setup_linux.sh

```sh
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
```

## demo/setup_qemu.sh

```sh
#!/bin/bash

DEB_REPO=https://deb.debian.org/debian
DEB=fs
IMG=image.img
LOCK=mounted.lock
MOD=rustsp.ko

# Clone a debian filesystem
[ -d $DEB ] || sudo debootstrap \
    --arch $([ $(uname -m) = "aarch64" ] && echo arm64 || echo amd64 ) \
    bookworm $DEB $DEB_REPO
[ -f ${DEB}.tar ] || sudo tar -C $DEB -cf ${DEB}.tar .

# Create a raw img to store in "virtual disk"
[ -f $IMG ] || qemu-img create \
    -f raw \
    $IMG \
    4G \
    && mkfs.ext4 $IMG

# Mount IMG into /mnt to copy filesystem to it
MNT=/mnt
[ -d $MNT ] || sudo mkdir $MNT
[ -f ${MNT}/${LOCK} ] || sudo mount -o loop $IMG $MNT \
    && touch ${MNT}/${LOCK} \
    && sudo tar -C $MNT -xf ${DEB}.tar \
    && cp $MOD /mnt \
    && umount /mnt

# --- INSIDE VM ---
# mount -t proc proc /proc
# mount -t sysfs sysfs /sys
# insmod $MOD
# rmmmod $MOD
```

## demo/rustsp.rs

```rs
//! Simple Rust module for the Linux Kernel for demonstration

use kernel::prelude::*; // Basic utilities

module! {
    type: RustSPModule,
    name: "rust_sp",
    authors: ["Rust SP Team"],
    description: "Sample demonstration module",
    license: "GPL",
}

struct RustSPModule {
    greeting: &'static str,
    farewell: &'static str,
}

impl kernel::Module for RustSPModule {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        let this = RustSPModule {
            greeting: "Hello from the kernel. With love, Rust",
            farewell: "Goodbye. Thank you for participating",
        };
        pr_info!("{:?}\n", this.greeting);
        Ok(this)
    }
}

impl Drop for RustSPModule {
    fn drop(&mut self) {
        pr_info!("{:?}\n", self.farewell);
    }
}
```

