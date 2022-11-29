{
  # Make sure that we still have admin access to the machine
  services.openssh.enable = true;
  networking.firewall.allowedTCPPorts = [ 22 ];
  users = {
    mutableUsers = false;
    users.root.openssh.authorizedKeys.keyFiles = [ ./teozkr_id_rsa.pub ];
  };
}