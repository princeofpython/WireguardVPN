# Wireguard Connection Program
This Rust code, when compiled, creates an executable which can be run on VM instances. It creates a WireGuard Interface which can be used as a client or a server (soon). A working VPN is establieshed if each instance is added as peer to another (soon). It uses `defguard-wireguard-rs` crate `v4.2.0` and the code is inspired by the examples provided [here](https://github.com/DefGuard/wireguard-rs/tree/v0.4.2/examples).
### Usage
````
sudo ./target/debug/wireguard-vpn [client/server] [up/down] [wg_interface_name]
sudo ./target/debug/wireguard-vpn client up wg1
sudo ./target/debug/wireguard-vpn server down wg2
...
````
After those commands, you can run `sudo wg` to check if the interfaces are initialized or removed.

## To do:
- Implement runServer
- Check correctness of VPN from two instances
- Adding peers via command line
- More documentation
