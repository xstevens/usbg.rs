# Vagrant Box for running USB Gadgets locally in a loopback and a Rust environment.

Vagrant.configure("2") do |config|

  config.vm.box = "ubuntu/zesty64"
  config.vm.provision "shell", privileged: false, inline: <<-SHELL
    sudo apt-get update
    sudo apt-get install --no-install-recommends -y \
      curl \
      build-essential \
      dpkg-dev \
      bc \
      libssl-dev \
      linux-image-extra-virtual \
      linux-headers-generic \

    # curl for rustup.rs
    # build-essential for ... essentials for building -*~stuff~*-
    # dpkg-dev for downloading kernel source
    # bc for Linux's `make prepare`
    # libssl-dev for Linux's `make scripts`
    # linux-image-extra-virtual since the default modules included don't
    # include fun stuff like other gadgets
    # linux-headers-generic for the module's build script

    # setup 'rustup.sh'
    sudo sh -c 'curl https://sh.rustup.rs -sSf > /usr/bin/rustup.sh'
    sudo chmod +x /usr/bin/rustup.sh
    # run 'rustup.sh'
    /usr/bin/rustup.sh -y

    # Unfortunately, two step provisions with a reboot require a plugin.
    # Best to just leave it manually.
    echo "The dummy_hcd kernel module enables USB loopback for gadgets."
    echo "Unfortunately, Ubuntu/Debian does not build it, even in the 'extra' package."
    echo "Run 'vagrant reload' on the host and then 'sudo /vagrant/scripts/build_dummy_hcd.sh' inside the vagrant VM to build and install dummy_hcd."
    echo "This will have to be done on every kernel upgrade."
  SHELL
end
