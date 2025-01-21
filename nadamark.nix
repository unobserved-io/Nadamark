{ config, pkgs, ... }:

{

  virtualisation.oci-containers.containers."nadamark" = {
      autoStart = true;
      image = "ghcr.io/unobserved-io/nadamark:latest";
      ports = [
        "8663:8663"
      ];
      volumes = [
        "/path/to/data:/bookmarks"
      ];
      environment = {
        # Replace with your user's UID/GID (find with 'id -u' and 'id -g' commands)
        USER_ID = "1000";
        GROUP_ID = "1000";
      };
  };
}
