# -*- mode: ruby -*-
# vi: set ft=ruby :
ENV["LC_ALL"] = "en_US.UTF-8"

Vagrant.configure("2") do |config|
  # Change the "./test_doc" folder to your book's folder
  config.vm.synced_folder "./test_doc", "/book"

  # # Disable auto update of VirtualBox Guest Additions
  # config.vbguest.auto_update = false

  config.vm.define "mdbook-pdf" do |subconfig|
    # Set box and hostname
    subconfig.vm.box = "ubuntu/jammy64"
    subconfig.vm.hostname = "mdbook-pdf"
    # subconfig.vm.provider "virtualbox" do |vb|
    #   # Custom VM memory and CPU cores
    #   vb.customize ["modifyvm", :id, "--memory", "4096"]
    #   vb.customize ["modifyvm", :id, "--cpus", "14"]
    # end

    # Add provisioning scripts
    subconfig.vm.provision "shell", privileged: false, inline: <<-SHELL
      sudo apt update
      sudo apt install -y snapd build-essential python3 python3-pip
      sudo snap install chromium
      sudo pip3 install /vagrant
      curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
      source "$HOME/.cargo/env"
      cargo install mdbook
      cargo install --path /vagrant
      cp -r /book ~/book
      cd ~/book && mdbook build
      rm -r /book/book
      mv book /book
      cd .. && rm -r book
    SHELL
  end
end
