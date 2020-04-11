# Overview

This is a Rust implementation providing a server for Philips Streamium enabled devices (e.g. the MCI200) written in Rust.

## Features

* Scan music library and make them available on Streamium devices
* Favorites for easier access to your most loved music
* Network streams - both local and internet - for WebRadio support
* A WebUI for managing your favorites and your streams

## Limitations

* Currently only works with PostgreSQL databases
* It has only been tested on Linux (both `AMD64` and `armhf`)
* The feature set is very limited at the moment
  * If you'd like to propose a new feature, please open an issue

# Setup

## Required steps

* Create a new database user and database on PostgreSQL
  * Using an existing user and database should work as well, though I **highly recommend creating dedicated** ones
* Grab the latest binary from https://github.com/j-be/rust-streamium/releases
  * Make sure the executable flag is set: `sudo chmod 0111 streamium-server`
  * If you use a dedicated Linux user (see below) you can even go for `sudo chmod 0100 streamium-server`
* Grab the `templates.zip` from https://github.com/j-be/rust-streamium/releases and extract it to the same folder
* Create a file called `.env` in the same folder. The content looks like:
```
# Server specific stuff
ROCKET_ADDRESS=YourServer'sIpAddress
ROCKET_PORT=42591

# Database stuff
DATABASE_URL=postgres://yourDbUser:yourDbPassword@localhost/yourDb
ROCKET_DATABASES="{streamium_db={url=\"${DATABASE_URL}\"}}"

# Library specific stuff
LIB_DIR=/path/to/your/music
BASE_URL=http://${ROCKET_ADDRESS}:${ROCKET_PORT}/files/
```
  * Replace the following
    * `YourServer'sIpAddress` the IP address, where the server can be reached (e.g. `ROCKET_ADDRESS=192.168.1.123` for local network only, or `ROCKET_ADDRESS=0.0.0.0` for from everywhere)
      * Do **NOT** change `ROCKET_PORT` - this seems to be hardcoded in the Streamium protocol (or at least I did not find a way to use any other port then `42591`)
    * `yourDbUser` the user on PostgreSQL
    * `yourDbPassword` the password for `yourDbUser`
    * `/path/to/your/music`

Now you should have the following directory structure:
```
/path/to/streamium
  |- streamium-server
  |- templates
  |- .env
```

## Optional, though highly recommended

* Create a dedicated Linux user for the service
  * It does not need any privileges, nor does it require any persistence other then the database
* If your are using a Linux Distro with `systemd` you may use the following service (the later steps assume this will be `streamium.service`):
```
[Unit]
Description=Streamium Server
After=syslog.target
After=network.target
After=postgresql.service

[Service]
RestartSec=2s
Type=simple
User=streamium
Group=streamium
StandardOutput=syslog
StandardError=syslog
SyslogIdentifier=streamium
WorkingDirectory=/path/to/streamium
ExecStart=/path/to/streamium/streamium-server
Restart=always
Environment=

[Install]
WantedBy=multi-user.target
```
  * Replace the following
    * `User=streamium` the Linux user the service should run as
    * `WorkingDirectory=/path/to/streamium` and `ExecStart=/path/to/streamium/streamium-server` put the actual path to the binary
  * Enable and start the service
    * `sudo systemctl enable streamium.service`
    * `sudo systemctl start streamium.service`
  * Check the status
    * `sudo systemctl status streamium.service`
* **Hint** If you use the systemd service, the server will log to syslog. Logs can be viewd using `journalctl -u streamium -xe`

## First steps

* Open http://yourServerIp:42591
* Click the refresh button on the bottom left - this will scan your music library
  * **Gotcha:** The import will **NEITHER** drop your existing database, **NOR** will it update existing entries. It will just reimport everything - again. So if you want to rescan your library, you have to first `drop` the database in PostgreSQL. **NOTE:** If you drop the database, you will loose your favorites and streams as well. If you have a good idea on how to avoid this in code, please open an issue. I'm happy to implement it, I'm just not sure yet on how to handle this in a reliable way.

## Done

We are done :beers:! Your music library should now be found by your Streamium device.

# Build from source

On instructions on how to build, please consult https://github.com/j-be/rust-streamium/blob/master/.github/workflows/build-and-release.yml and mind rust-lang/rust#68264

If there are still open questions, please open an issue.

# The furture

Currently, I have the following things on my TODO list:

* [ ] Provide an Ansible role
  * [x] Write it
  * [ ] bring it to Github
* [ ] Find a way how to update the database on rescans
  * If you know a solution and are a Rust developer (or would like to become one) feel free to open a PR
