# -*- mode: ruby -*-
# vi: set ft=ruby :

Vagrant.configure(2) do |config|

  # Determine available host resources
  mem_ratio = 3/4
  cpu_exec_cap = 75
  host = RbConfig::CONFIG['host_os']
  # Give VM 3/4 system memory & access to all cpu cores on the host
  if host =~ /darwin/
    cpus = `sysctl -n hw.ncpu`.to_i
    # sysctl returns Bytes and we need to convert to MB
    mem = `sysctl -n hw.memsize`.to_i / 1024^2 * mem_ratio
  elsif host =~ /linux/
    cpus = `nproc`.to_i
    # meminfo shows KB and we need to convert to MB
    mem = `grep 'MemTotal' /proc/meminfo | sed -e 's/MemTotal://' -e 's/ kB//'`.to_i / 1024 * mem_ratio
  else # Windows folks
    cpus = `wmic cpu get NumberOfCores`.split("\n")[2].to_i
    mem = `wmic OS get TotalVisibleMemorySize`.split("\n")[2].to_i / 1024 * mem_ratio
  end

  config.vm.define :ueransim do |ueransim|
    ueransim.vm.box = "ubuntu/focal64"
    ueransim.vm.hostname = "ueransim"

    machine_ip = "192.168.60.200"
    ueransim.vm.network "private_network", ip: machine_ip

    ueransim.vm.provider "virtualbox" do |vb|
      vb.customize ["modifyvm", :id, "--memory", "2048"]
      vb.customize ["modifyvm", :id, "--cpus", "1"]

    end

    ueransim.vm.provision "ansible" do |ansible|
      ansible.compatibility_mode = "2.0"
      ansible.host_key_checking = false
      ansible.playbook = "ansible/ueransim.yml"
      ansible.raw_arguments = ['--timeout=20', '--connection=paramiko']
      ansible.verbose = 'v'
      ansible.extra_vars = {
        colte_ip: machine_ip,
        prompt_color: "yellow",
        tmux_color: "yellow"
      }
    end

  end

  config.vm.define :colte1 do |colte|
    colte.vm.box = "ubuntu/focal64"
    colte.vm.hostname = "colte1"

    machine_ip = "192.168.56.101"
    colte.vm.network "private_network", ip: machine_ip

    colte.vm.provider "virtualbox" do |vb|
      vb.customize ["modifyvm", :id, "--memory", "2048"]
      vb.customize ["modifyvm", :id, "--cpus", "1"]
    end

    colte.vm.provision "ansible" do |ansible|
      ansible.compatibility_mode = "2.0"
      ansible.host_key_checking = false
      ansible.playbook = "ansible/colte.yml"
      ansible.raw_arguments = ['--timeout=20', '--connection=paramiko']
      ansible.verbose = 'v'
      ansible.extra_vars = {
        colte_ip: machine_ip,
        test_net_index: 1,
        prompt_color: "cyan",
        tmux_color: "cyan"
      }
    end

  end

  config.vm.define :colte2 do |colte|
    colte.vm.box = "ubuntu/focal64"
    colte.vm.hostname = "colte2"

    machine_ip = "192.168.56.102"
    colte.vm.network "private_network", ip: machine_ip

    colte.vm.provider "virtualbox" do |vb|
      vb.customize ["modifyvm", :id, "--memory", "2048"]
      vb.customize ["modifyvm", :id, "--cpus", "1"]
    end

    colte.vm.provision "ansible" do |ansible|
      ansible.compatibility_mode = "2.0"
      ansible.host_key_checking = false
      ansible.playbook = "ansible/colte.yml"
      ansible.raw_arguments = ['--timeout=20', '--connection=paramiko']
      ansible.verbose = 'v'
      ansible.extra_vars = {
        colte_ip: machine_ip,
        test_net_index: 2,
        prompt_color: "green",
        tmux_color: "cyan"
      }
    end
  end

  config.vm.define :directory do |colte|
    colte.vm.box = "ubuntu/focal64"
    colte.vm.hostname = "directory"

    machine_ip = "192.168.56.250"
    colte.vm.network "private_network", ip: machine_ip

    colte.vm.provider "virtualbox" do |vb|
      vb.customize ["modifyvm", :id, "--memory", "1024"]
      vb.customize ["modifyvm", :id, "--cpus", "1"]
    end

    colte.vm.provision "ansible" do |ansible|
      ansible.compatibility_mode = "2.0"
      ansible.host_key_checking = false
      ansible.playbook = "ansible/directory.yml"
      ansible.raw_arguments = ['--timeout=20', '--connection=paramiko']
      ansible.verbose = 'v'
      ansible.extra_vars = {
        colte_ip: machine_ip,
        prompt_color: "red",
        tmux_color: "red"
      }
    end
  end

  config.vm.define :dauthDev do |colte|
    colte.vm.box = "ubuntu/focal64"
    colte.vm.hostname = "dauthDev"

    machine_ip = "192.168.56.2"
    colte.vm.network "private_network", ip: machine_ip

    colte.vm.synced_folder '.', '/vagrant', disabled: true

    if Vagrant::Util::Platform.windows? then
      colte.vm.synced_folder "./infra/" , "/home/vagrant/infra", type: "virtualbox"
      colte.vm.synced_folder "./services/" , "/home/vagrant/services", type: "virtualbox"
      colte.vm.synced_folder "./open5gs/" , "/home/vagrant/open5gs", type: "virtualbox"
      colte.vm.synced_folder "./protos/" , "/home/vagrant/protos", type: "virtualbox"
      colte.vm.synced_folder "./ueransim/" , "/home/vagrant/ueransim", type: "virtualbox"
    else
      colte.vm.synced_folder "./infra/" , "/home/vagrant/infra", type: "nfs", nfs_version: 4
      colte.vm.synced_folder "./services/" , "/home/vagrant/services", type: "nfs", nfs_version: 4
      colte.vm.synced_folder "./open5gs/" , "/home/vagrant/open5gs", type: "nfs", nfs_version: 4
      colte.vm.synced_folder "./protos/" , "/home/vagrant/protos", type: "nfs", nfs_version: 4
      colte.vm.synced_folder "./ueransim/" , "/home/vagrant/ueransim", type: "nfs", nfs_version: 4
    end

    colte.vm.provider "virtualbox" do |vb|
      vb.customize ["modifyvm", :id, "--memory", "4096"]
      vb.customize ["modifyvm", :id, "--cpus", cpus]
      vb.customize ["modifyvm", :id, "--cpuexecutioncap", 75]
    end

    colte.vm.provision "ansible" do |ansible|
      ansible.compatibility_mode = "2.0"
      ansible.host_key_checking = false
      ansible.playbook = "ansible/dauth_dev.yml"
      ansible.raw_arguments = ['--timeout=20', '--connection=paramiko']
      ansible.verbose = 'v'
      ansible.extra_vars = {
        colte_ip: machine_ip,
        prompt_color: "blue",
        tmux_color: "green"
      }
    end
  end
end
