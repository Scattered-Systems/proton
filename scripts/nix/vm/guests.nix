let
  machine = {host, port ? 22, name, hostNic, guests ? {}}: {pkgs, lib, ...}@args:
    {
      imports = [ ./baseline.nix ];

      # We still want to be able to boot, adjust as needed based on your setup
      boot = {
        loader = {
          systemd-boot.enable = true;
          efi.canTouchEfiVariables = true;
        };
        kernelParams = [ "nomodeset" ];
      };
      fileSystems = {
        "/" = {
          device = "/dev/disk/by-label/${name}-root";
        };
        "/boot" = {
          device = "/dev/disk/by-label/${name}-boot";
        };
      };
      boot.initrd.availableKernelModules = [ "xhci_pci" "ehci_pci" "ahci" "usbhid" "usb_storage" "sd_mod" ];

      # Tell NixOps how to find the machine
      deployment.targetEnv = "none";
      deployment.targetHost = host;
      deployment.targetPort = port;
      networking.privateIPv4 = host;
    };
in
  {
    # Tell NixOps about the hosts it should manage
    athens = machine {
      host = "192.168.0.2";
      name = "athens";
      hostNic = "enp30s0";
      guests = {
        some-athens-guest = {
          memory = "4"; # GB
          diskSize = "50"; # GB
          mac = "D2:91:69:C0:14:9A";
          ip = "192.168.0.101"; # Ignored, only for personal reference
        };
    };
    rome = machine {
      host = "192.168.0.3";
      name = "rome";
      hostNic = "enp3s0";
    };
  }