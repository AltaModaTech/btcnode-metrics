# Btcnode-metrics as a SystemD service

## Prerequisites

- Sudo privilege
- Current dir is the root of this project, i.e., btcnode-metrics.

## Initialization

Follow these steps to initialize the host before the first time running the service.

- Make the helper scripts executable

  `chmod +x ./linux/*.sh`

- Use the `create-user_btcnode_metrics.sh` script to:
  - Create a user & a group named _btcnode_metrics_ for the service.
  - Create a config dir in _btcnode_metrics_ home dir for the btcnode-metrics config file.

  `sudo cp ./linux/create-user_btcnode_metrics.sh`

- Copy service file

  `sudo cp ./linux/etc/systemd/system/btcnode-metrics.service /etc/systemd/system/`

## Updating

Follow these instructions for updating the btcnode-metrics application or its config file.

### Updating the btcnode-metrics app

- Install the latest version

  `cargo install btcnode-metrics --force`

- Copy the app to the shared location

  `sudo cp $HOME/.cargo/bin/btcnode-metrics /usr/local/bin`

### Updating the btcnode-metrics config

  `sudo cp ./config.local.toml /home/btcnode_metrics/.config/btcnode-metrics/`
  `sudo chown btcnode_metrics:btcnode_metrics /home/btcnode_metrics/.config/btcnode-metrics/config.local.toml`
