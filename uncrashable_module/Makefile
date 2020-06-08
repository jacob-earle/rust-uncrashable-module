KDIR ?= /lib/modules/$(shell uname -r)/build

all:
	$(MAKE) -C $(KDIR) M=$(CURDIR) KBUILD_EXTMOD=`pwd`/../src
	$(MAKE) -C $(KDIR) M=$(CURDIR) KBUILD_EXTRA_SYMBOLS=`pwd`/../src/Module.symvers
	
load:
	`pwd`/load_module.sh

unload:
	`pwd`/unload_module.sh

clean:
	$(MAKE) -C $(KDIR) M=$(CURDIR) clean
