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
  };
}
