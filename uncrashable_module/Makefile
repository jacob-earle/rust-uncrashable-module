KDIR ?= /lib/modules/$(shell uname -r)/build

all:
	$(MAKE) -C $(KDIR) M=$(CURDIR) KBUILD_EXTMOD=`pwd`

clean:
	$(MAKE) -C $(KDIR) M=$(CURDIR) clean
