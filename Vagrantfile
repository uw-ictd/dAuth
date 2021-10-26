# -*- mode: ruby -*-
# vi: set ft=ruby :

Vagrant.configure(2) do |config|

  config.vm.define :ueransim do |ueransim|
    ueransim.vm.box = "ubuntu/focal64"
    ueransim.vm.hostname = "ueransim"

    ueransim.vm.network "private_network", ip: "192.168.40.200"

    ueransim.vm.provider "virtualbox" do |vb|
      vb.customize ["modifyvm", :id, "--memory", "2048"]
      vb.customize ["modifyvm", :id, "--cpus", "3"]

    end

    ueransim.vm.provision "ansible" do |ansible|
      ansible.compatibility_mode = "2.0"
      ansible.host_key_checking = false
      ansible.playbook = "ansible/ueransim.yml"
      ansible.raw_arguments = ['--timeout=20', '--connection=paramiko']
      ansible.verbose = 'v'
    end

  end

  config.vm.define :colte1 do |colte|
    colte.vm.box = "ubuntu/focal64"
    colte.vm.hostname = "colte1"

    colte.vm.network "private_network", ip: "192.168.41.200"

    colte.vm.provider "virtualbox" do |vb|
      vb.customize ["modifyvm", :id, "--memory", "2048"]
      vb.customize ["modifyvm", :id, "--cpus", "3"]
    end

    colte.vm.provision "ansible" do |ansible|
      ansible.compatibility_mode = "2.0"
      ansible.host_key_checking = false
      ansible.playbook = "ansible/colte.yml"
      ansible.raw_arguments = ['--timeout=20', '--connection=paramiko']
      ansible.verbose = 'v'
      ansible.extra_vars = {
        colte_ip: "192.168.41.200"
      }
    end

  end

  config.vm.define :colte2 do |colte|
    colte.vm.box = "ubuntu/focal64"
    colte.vm.hostname = "colte2"

    colte.vm.network "private_network", ip: "192.168.42.200"

    colte.vm.provider "virtualbox" do |vb|
      vb.customize ["modifyvm", :id, "--memory", "2048"]
      vb.customize ["modifyvm", :id, "--cpus", "3"]
    end

    colte.vm.provision "ansible" do |ansible|
      ansible.compatibility_mode = "2.0"
      ansible.host_key_checking = false
      ansible.playbook = "ansible/colte.yml"
      ansible.raw_arguments = ['--timeout=20', '--connection=paramiko']
      ansible.verbose = 'v'
      ansible.extra_vars = {
        colte_ip: "192.168.42.200"
      }
    end
  end

  config.vm.define :authDev do |colte|
    colte.vm.box = "ubuntu/focal64"
    colte.vm.hostname = "authDev"

    colte.vm.network "private_network", ip: "192.168.50.200"

    colte.vm.synced_folder '.', '/vagrant', disabled: true

    if Vagrant::Util::Platform.windows? then
      colte.vm.synced_folder "./open5gs/" , "/home/vagrant/open5gs", type: "virtualbox"
      colte.vm.synced_folder "./ueransim/" , "/home/vagrant/ueransim", type: "virtualbox"
    else
      colte.vm.synced_folder "./open5gs/" , "/home/vagrant/open5gs", type: "nfs", nfs_version: 4
      colte.vm.synced_folder "./ueransim/" , "/home/vagrant/ueransim", type: "nfs", nfs_version: 4
    end

    colte.vm.provider "virtualbox" do |vb|
      vb.customize ["modifyvm", :id, "--memory", "2048"]
      vb.customize ["modifyvm", :id, "--cpus", "3"]
    end

    colte.vm.provision "ansible" do |ansible|
      ansible.compatibility_mode = "2.0"
      ansible.host_key_checking = false
      ansible.playbook = "ansible/colte.yml"
      ansible.raw_arguments = ['--timeout=20', '--connection=paramiko']
      ansible.verbose = 'v'
      ansible.extra_vars = {
        colte_ip: "192.168.42.200"
      }
    end
  end
end