#!/bin/bash
#
# Sets up a composite HID + RNDIS USB device.
#
# Expects to be run on Debian Jessie base image
# provided by Inverse Path.
#
# The MAC address will be set to the same one
# that the Debian Jessie base image uses, namely
# 1a:55:89:a2:69:41. The host will see the device
# as having the MAC address 1a:55:89:a2:69:42.
# 
# This is a modified version inspired by: 
#     https://github.com/qlyoung/armory-keyboard
# Collin Mulliner <collin AT mulliner.org>
# Quentin Young <qlyoung@qlyoung.net>
# 
# Modified by:
# Xavier Stevens <xavier.stevens AT gmail.com>

# load libcomposite
modprobe libcomposite

# remove all USB Ethernet drivers
modprobe -r g_ether usb_f_rndis u_ether

# insert modules for HID and ECM Ethernet
modprobe usb_f_hid
modprobe usb_f_rndis

target/debug/examples/hidrndis
