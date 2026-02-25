# Btcnode-metrics as a SystemD service

## Prerequisites

- Sudo privilege

## Initialization

Follow these steps to initialize the host before the first time running the service.

- Use _sudo_ to run `create-user_btcnode_metrics.sh`. This script
  - Creates a user & a group named _btcnode_metrics_ for the service.
  - Creates a config dir in _btcnode_metrics_ home dir for securing the config file.
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

- Update the config file
- Update the app
