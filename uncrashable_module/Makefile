KDIR ?= /lib/modules/$(shell uname -r)/build

all:
	$(MAKE) -C $(KDIR) M=$(CURDIR) KBUILD_EXTMOD=`pwd`/../src
	$(MAKE) -C $(KDIR) M=$(CURDIR) KBUILD_EXTRA_SYMBOLS=`pwd`/../src/Module.symvers
	

clean:
	$(MAKE) -C $(KDIR) M=$(CURDIR) clean
