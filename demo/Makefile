# source: https://github.com/Rust-for-Linux/rust-out-of-tree-module
IMG = image.img
KDIR ?= ./linux


all: module qemu

module: linux
	$(MAKE) -C $(KDIR) M=$$PWD

linux:
	-@bash setup_linux.sh

modules_install: default
	$(MAKE) -C $(KDIR) M=$$PWD modules_install

clean:
	-sudo rm fs/
	-rm *.o *.ko *.c *.symvers *.order *.mod .rustsp* .module* ..module* .Module*

qemu: module
	@bash setup_qemu.sh

run: $(IMG)
	@qemu-system-x86_64 \
		-kernel $(KDIR)/arch/x86/boot/bzImage \
		-hda $(IMG) \
		-append "root=/dev/sda rw console=ttyS0 init=/bin/bash" \
		-nographic \
		-m 1024


.PHONY: linux clean qemu run
