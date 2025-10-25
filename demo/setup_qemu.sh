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
