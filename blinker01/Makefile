ARCH=arm
RUSTSRC=../rust
RUSTLIBS=../rustlibs

XCOMPILE=$(ARCH)-none-eabi
CC=$(XCOMPILE)-gcc
OBJCOPY=$(XCOMPILE)-objcopy

SFLAGS=-mcpu=arm1176jzf-s -g -fpic -ffreestanding -c 

RUST=rustc
RFLAGS=-C opt-level=2 -Z no-landing-pads --target ../$(XCOMPILE) -g --emit obj
RLFLAGS=-C opt-level=2 -Z no-landing-pads --target ../$(XCOMPILE) -g --crate-type=lib

BOOTSRC=../boot

.PHONY: clean distclean

default: kernel.img

clean:
	-@rm *.o *.elf *.img

distclean: clean
	-@rm $(BOOTSRC)/start.o
	-@rm $(RUSTLIBS)/libcore.rlib

# kernel Image
kernel.img: kernel.elf
	$(OBJCOPY) $^ -O binary $@

# Linked kernel
kernel.elf: kernel.o linker.ld $(BOOTSRC)/start.o
	$(CC) -T linker.ld -ffreestanding -nostdlib -o $@ $< $(BOOTSRC)/start.o
	
# Rust kernel
kernel.o : kernel.rs \
		$(RUSTLIBS)/libcore.rlib
	$(RUST) $(RFLAGS) -L $(RUSTLIBS) -o $@ $<

# Assemble the boot code
$(BOOTSRC)/start.o: $(BOOTSRC)/start.S
	$(CC) $(SFLAGS) -c $^ -o $@
	
# Compile the rust core library
$(RUSTLIBS)/libcore.rlib: $(RUSTSRC)/src/libcore/lib.rs \
		$(wildcard $(RUSTSRC)/*/*.rs)
	$(RUST) $(RLFLAGS) --out-dir $(RUSTLIBS) $<