sudo useradd \
  -c "BTC Node Metrics" \   # comment
  -m \                      # Create home dir
  -g btcnode_metrics \      # default group membership
  -G users \                # additional group membership
  btcnode_metrics