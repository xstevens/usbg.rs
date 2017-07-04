# usbg.rs
USB Gadget Configfs Library

# Building the HID + ECM example
Rust has built-in support to compile examples. You can build the HID + ECM example like so:

```
cargo build --example hidecm
```

# Demo
[![asciicast](https://asciinema.org/a/4oc2n1za4o9nseny70ufjldo6.png)](https://asciinema.org/a/4oc2n1za4o9nseny70ufjldo6)

# Vagrant

A Vagrant box is available that pre-installs packages needed to compile a loopback dummy_hcd kernel module. Be aware that the provisioning step is a *two-step process* and requires a `vagrant reload` and execution of the script mentioned at the end of the first provision to build and install the kernel module.

Before running the shell scripts accompanying the examples, run `sudo modprobe dummy_hcd` to load the loopback.
