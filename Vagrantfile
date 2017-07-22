# -*- mode: ruby -*-
# vi: set ft=ruby ts=2 sw=2 expandtab :

PROJECT = "rust-statistics"
HOME_DIRECTORY = "/home/vagrant"
PROJECT_DIRECTORY = "#{HOME_DIRECTORY}/#{PROJECT}"

DOCKER_ENV = {
  "HOST_USER_UID" => Process.euid,
  "HOME_DIRECTORY" => "#{HOME_DIRECTORY}",
  "PROJECT_DIRECTORY" => "#{PROJECT_DIRECTORY}",
  "APP_PATH" => "#{PROJECT_DIRECTORY}/rust-statistics",
  "OPENSSL_LIB_DIR" => "/usr/lib/x86_64-linux-gnu",
  "OPENSSL_INCLUDE_DIR" => "/usr/include/openssl",
}

ENV['VAGRANT_NO_PARALLEL'] = 'yes'
ENV['VAGRANT_DEFAULT_PROVIDER'] = 'docker'
Vagrant.configure(2) do |config|

  config.ssh.insert_key = false
  config.vm.define "dev", primary: true do |app|
    app.vm.provider "docker" do |d|
      d.image = "jean553/rust-dev-docker"
      d.name = "#{PROJECT}_dev"
      d.has_ssh = true
      d.env = DOCKER_ENV
      d.volumes =  [
        "#{ENV['PWD']}/:#{PROJECT_DIRECTORY}",
      ]
      # required when kcod runs into a docker container,
      # details: https://github.com/jean553/rust-dev-docker
      d.create_args = ["--security-opt=seccomp=unconfined"]
    end
    app.ssh.username = "vagrant"
  end
end
