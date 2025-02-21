# Nadamark

Nadamark is a bookmark manager for minimalists. You can use the provided docker-compose.yml to self-host it on your own server/computer (instructions below).

<p align="center">
    [![Nadamark Intro Video](http://i.ytimg.com/vi/nfH1TrSqyVM/hqdefault.jpg)](https://www.youtube.com/watch?v=nfH1TrSqyVM)
</p>

## Disclaimer
⚠️ Nadamark is new and under active development. It is ready for production, but there may be breaking changes in future versions. I recommend regularly exporting your bookmarks to HTML as a backup.

## Features

* Fast
* Minimal
* Self-hostable

What more could you want?

## Install with Docker Compose

The easiest way to install Nadamark is with Docker. Download the [docker-compose.yml](docker-compose.yml) and change the envrionment variables in it for your desired PostgreSQL user and password.

In the folder where you downloaded the `docker-compose.yml`, change the data directory in the `volumes` section to the proper path on your machine where you want the data stored. Then change the `USER_ID` and `GROUP_ID` environment variables to match your user. To find your user ID and group ID, run these commands:
```bash
id -u  # prints your user ID
id -g  # prints your group ID
```

Once complete, run `docker-compose up -d` to start the server.

You can then navigate to `localhost:8663` to start using Nadamark right away!

## Install on NixOS (Docker)

You can copy the `nadamark.nix` file to install the Docker container on your NixOS machine. Remember to change the path in `volumes`, as well as the `USER_ID` and `GROUP_ID` environment variables according to the instructions above.

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
