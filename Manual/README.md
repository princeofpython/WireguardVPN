# Manual Wireguard VPN Connection

### Install WireGuard

Execute the following command on the Server and Client.

```bash
sudo apt update
sudo apt install wireguard
```
Verify the Installation: After installation, verify the `wg` command works:

```bash
wg --version
```

### Generate private and public keypairs on both Server and Client

```bash
wg genkey | tee privatekey | wg pubkey > publickey
```

Fill the generated public and private keys in the config files (`.conf`) in both `client` and `server` folders. This makes each instance as a peer to another.

### Initialize the WireGuard VPN

- Run `setup.sh` first in the server, which enables IP forwarding. 
- Then run `start.sh` or `stop.sh` to initialize or remove the interface.
- PostUp rules implemented in `.conf` file are vital for the VPN connection. 
- IP Rule `iptables -A FORWARD -i wg0 -j ACCEPT` ensures packets are forwarded to server from wireguard interface without being blocked by firewall.
- IP rule `iptables -t nat -A POSTROUTING -o ens5 -j MASQUERADE` applies masquerading, modifying outgoing network traffic to use the server's public IP address, allowing VPN clients to access the internet through the server.

### Checking if VPN is working
- ping 8.8.8.8 should be working in both server and client
- curl ipaddress.ai in client should be public IP of server.
- tcpdump in server should show the packets when ping 8.8.8.8 is ran in client






