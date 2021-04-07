ARCH:=x86_64
TARGET_DIR:=target
BOOTLOADER:=uefi
BOOTLOADER_TARGET:=$(ARCH)-unknown-$(BOOTLOADER)
KERNEL:=kernel
KERNEL_TARGET:=$(ARCH)-unknown-none
KERNEL_BUILD_TYPE:=debug
QEMU_DIR:=qemu
UEFI_BOOTLOADER_TARGET:=x64
QEMU_OPTION:=-m 1G -smp 4
QEMU_UEFI_OPTION:=-drive if=pflash,format=raw,file=$(QEMU_DIR)/OVMF.fd -drive file=fat:rw:$(QEMU_DIR)/fs,format=raw
QEMU_GDB_OPTION:=-S -s
UEFI_KERNEL_PATH:=boot/$(KERNEL)
QEMU:=qemu-system-$(ARCH)

qemu-efi: $(QEMU_DIR) $(QEMU_DIR)/OVMF.fd $(QEMU_DIR)/fs/EFI/BOOT/BOOT$(UEFI_BOOTLOADER_TARGET).EFI \
          $(QEMU_DIR)/fs/$(UEFI_KERNEL_PATH)
	$(QEMU) $(QEMU_OPTION) $(QEMU_UEFI_OPTION)

qemu-efi-gdb: $(QEMU_DIR) $(QEMU_DIR)/OVMF.fd $(QEMU_DIR)/fs/EFI/BOOT/BOOT$(UEFI_BOOTLOADER_TARGET).EFI \
          $(QEMU_DIR)/fs/$(UEFI_KERNEL_PATH)
	$(QEMU) $(QEMU_OPTION) $(QEMU_UEFI_OPTION) $(QEMU_GDB_OPTION)

qemu-efi-gdb-bg: $(QEMU_DIR) $(QEMU_DIR)/OVMF.fd $(QEMU_DIR)/fs/EFI/BOOT/BOOT$(UEFI_BOOTLOADER_TARGET).EFI \
          $(QEMU_DIR)/fs/$(UEFI_KERNEL_PATH)
	$(QEMU) $(QEMU_OPTION) $(QEMU_UEFI_OPTION) $(QEMU_GDB_OPTION) &

$(QEMU_DIR):
	mkdir -p $@/fs/EFI/BOOT/
	mkdir -p $@/fs/boot/

$(QEMU_DIR)/OVMF.fd: $(OVMF_PATH)
	cp /usr/share/ovmf/OVMF.fd $@

$(QEMU_DIR)/fs/EFI/BOOT/BOOT$(UEFI_BOOTLOADER_TARGET).EFI: \
           $(TARGET_DIR)/bootloader/$(BOOTLOADER)/$(BOOTLOADER_TARGET)/release/$(BOOTLOADER).efi
	cp $< $@

$(QEMU_DIR)/fs/$(UEFI_KERNEL_PATH): \
           $(TARGET_DIR)/$(KERNEL)/$(KERNEL_TARGET)/$(KERNEL_BUILD_TYPE)/$(KERNEL)
	cp $< $@

$(TARGET_DIR)/bootloader/$(BOOTLOADER)/$(BOOTLOADER_TARGET)/release/$(BOOTLOADER).efi: FORCE
	cd bootloader/$(BOOTLOADER) && cargo build --release --target-dir=../../$(TARGET_DIR)/bootloader/$(BOOTLOADER)

$(TARGET_DIR)/$(KERNEL)/$(KERNEL_TARGET)/release/$(KERNEL): FORCE
	cd $(KERNEL) && cargo build --release --target-dir=../$(TARGET_DIR)/$(KERNEL)

$(TARGET_DIR)/$(KERNEL)/$(KERNEL_TARGET)/debug/$(KERNEL): FORCE
	cd $(KERNEL) && cargo build --target-dir=../$(TARGET_DIR)/$(KERNEL)

clean:
	rm -rf $(TARGET)
	rm -rf $(QEMU_DIR)

FORCE:
.PHONY: clean FORCE qemu-efi qemu-efi-gdb  qemu-efi-gdb-bg