#!/bin/sh

# Compile and Install the Dummy HCD driver which isn't pre-included.

set -e

if [ -f /var/run/reboot-required ]; then
    echo "dummy_hcd Build Aborted. Reboot Required detected."
    echo "Build dependent on building while running latest available kernel."
    echo "Run 'vagrant reload' on the host."
    exit 0
fi

KERNEL_RELEASE=$(uname -r)
KERNEL_SHORT_VER=$(echo "$KERNEL_RELEASE" | cut -d"-" -f1)

echo "Downloading Linux sources for $KERNEL_RELEASE and building dummy_hcd."

apt-get source linux-image-"$KERNEL_RELEASE"
cd linux-"$KERNEL_SHORT_VER" || exit
cp /boot/config-"$KERNEL_RELEASE" .config
cp /usr/src/linux-headers-"$KERNEL_RELEASE"/Module.symvers .
sed -i -r 's/^(CONFIG_USB_DUMMY_HCD=.*|# CONFIG_USB_DUMMY_HCD is not set)/CONFIG_USB_DUMMY_HCD=m/' .config
make prepare
sed -i -r "s/^#define UTS_RELEASE \".*\"/#define UTS_RELEASE \"$KERNEL_RELEASE\"/" include/generated/utsrelease.h
make scripts
make M=drivers/usb/gadget/udc
sudo cp drivers/usb/gadget/udc/dummy_hcd.ko /lib/modules/"$KERNEL_RELEASE"/kernel/drivers/usb/gadget/udc
sudo depmod --all
