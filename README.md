# Nadamark

Nadamark is a bookmark manager for minimalists. You can use the provided docker-compose.yml to self-host it on your own server/computer (instructions below).

<p align="center">
  <img src="https://unobserved.io/assets/screenshots/nadamark/nadamark.png" width="600">
</p>

## Features

* Fast
* Minimal
* Self-hostable

What more could you want?

## Install with Docker Compose

The easiest way to install Nadamark is with Docker. Download the [docker-compose.yml](https://github.com/unobserved-io/furtherance-sync/blob/main/docker-compose.yml) and change the envrionment variables in it for your desired PostgreSQL user and password.

In the folder where you downloaded the `docker-compose.yml`, change the data directory in the `volumes` section to the proper path on your machine where you want the data stored, then run `docker-compose up -d` to start the server.

You can then navigate to `localhost:8663` to start using Nadamark right away!

## Install on NixOS (Docker)

You can copy the `nadamark.nix` file to install the Docker container on your NixOS machine. Remember to change the path in the `volumes` section to the data directory on your machine.

Docker must be enabled on your NixOS machine (by adding `virtualisation.docker.enable = true;` to your Nix config).

### Donate
Monetary contributions help this project stay afloat. You can donate via these links:
* [PayPal](https://www.paypal.com/donate/?hosted_button_id=TLYY8YZ424VRL)
* [GitHub](https://github.com/sponsors/rickykresslein)
* [Patreon](https://www.patreon.com/unobserved)
* [Ko-fi](https://ko-fi.com/unobserved)

Thank you so much for your contribution! I truly appreciate it.

### License
This project is licensed under the GNU Affero General Public License v3.0. See the [LICENSE](LICENSE) file for details.

### Author
This project is created and maintained by [Ricky Kresslein](https://kressle.in) under [Unobserved](https://unobserved.io).
