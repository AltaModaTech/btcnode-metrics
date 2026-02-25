# This script requires root privileges
if [[ $(/usr/bin/id -u) -ne 0 ]]; then
  echo "This script must run as root. Use sudo."
  exit
fi


# Create user group
if getent group btcnode_metrics > /dev/null; then
  echo "Group btcnode_metrics exists"
else
  echo "Creating group btcnode_metrics..."
  sudo groupadd btcnode_metrics
fi

# Create user
if getent user btcnode_metrics > /dev/null; then
  echo "User btcnode_metrics exists"
else
  echo "Creating user btcnode_metrics..."

  sudo useradd \
    -c "BTC Node Metrics" \   # comment
    -m \                      # Create home dir
    -g btcnode_metrics \      # default group
    -G users \                # group membership
    btcnode_metrics
fi


# Init home dir; create config dir for app
config_dir = "/home/btcnode_metrics/.config/btcnode_metrics"
if [ ! -d ] "$config_dir"; then
  echo "Creating config dir"
  sudo mkdir -p $config_dir
fi
# Ensure btcnode_metric's user and default group own the config dir
sudo chown btcnode_metrics:btcnode_metrics /home/btcnode_metrics/.config/btcnode_metrics
