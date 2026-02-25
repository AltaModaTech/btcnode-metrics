# This script requires root privileges
if [[ $(/usr/bin/id -u) -ne 0 ]]; then
  echo "This script must run as root. Use sudo."
  exit
fi

# Does the config dir exist?
config_dir = "/home/btcnode_metrics/.config/btcnode_metrics"
if [ ! -d ] "$config_dir"; then
  echo "The config dir does not exist"
  exit
fi

# Does the local config file exist?
config_file = "../config.local.toml"
if [ -f ] $config_file; then
  sudo cp $config_file $config_dir
fi
# Ensure btcnode_metric's user and default group own the config file
sudo chown -R btcnode_metrics:btcnode_metrics /home/btcnode_metrics/.config/btcnode_metrics
