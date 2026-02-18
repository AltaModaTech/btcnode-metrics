Refactor the code in this project to meet the following criteria:

- Convert the btcnode-metrics crate into a module of btcnode-metrics and rename it to btcnode-metrics-gatherer
- Change btcnode-metrics to use the btcnode-metrics-gatherer module and remove the former btcnode-metrics crate
- Ensure the resulting code compiles and tests continue passing
