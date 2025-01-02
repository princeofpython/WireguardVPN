use std::{net::SocketAddr, str::FromStr, env};

use defguard_wireguard_rs::{
    host::Peer, key::Key, net::IpAddrMask, InterfaceConfiguration, WGApi, WireguardInterfaceApi,
};
// use x25519_dalek::{EphemeralSecret, PublicKey};


fn main() {

    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if sufficient arguments are provided
    if args.len() != 4 {
        eprintln!("Please check! Correct Usage: {} <server/client> <up/down> <wg_interface_name> ...", args[0]);
        std::process::exit(1);
    }
    let arg1 = &args[1];
    let b_is_server: bool = match arg1.as_str() {
        "server" => true,
        "client" => false,
        _ => {
            eprintln!("Invalid argument 1: '{}'. Use 'server' or 'client'.", arg1);
            std::process::exit(1);
        }
    };

    let arg2 = &args[2];
    let b_up: bool = match arg2.as_str() {
        "up" => true,
        "down" => false,
        _ => {
            eprintln!("Invalid argument 2: '{}'. Use 'up' or 'down'.", arg2);
            std::process::exit(1);
        }
    };

    let wg_interface_name :String = args[3].to_string();

    if b_is_server {
        let _ = run_server();
    } else {
        let _ = run_client(b_up, wg_interface_name);
    }
}

fn run_client(b_up: bool, wg_interface_name: String) -> Result<(), Box<dyn std::error::Error>> {
    // Create new API object for interface
    let wgapi = WGApi::new(wg_interface_name.clone(), false)?;

    println!("RunClient ====>"); 

    if !b_up{
        wgapi.remove_interface()?;
        println!("RunClient: Interface {} removed", wg_interface_name);
        println!("RunClient <====");
        return Ok(());
    }

    wgapi.create_interface()?;
    println!("RunClient: Interface created"); 
    // create interface

    // Peer configuration
    //let secret = EphemeralSecret::random();
    //let key = PublicKey::from(&secret);
    // Peer secret key
    //let peer_key: Key = key.as_ref().try_into().unwrap();

    let server_public_key = "<SERVER_PUBLIC_KEY>";
    // Convert Base64 public key into Key type
    let peer_key: Key = server_public_key.parse()?; // Parses Base64-encoded key
    let mut peer = Peer::new(peer_key.clone());

    log::info!("endpoint");
    // Your WireGuard server endpoint which client connects to
    let endpoint: SocketAddr = "<SERVER_PRIVATE_IP>".parse().unwrap();
    // Peer endpoint and interval
    peer.endpoint = Some(endpoint);
    peer.persistent_keepalive_interval = Some(25);              // to make sure the VPN doesn't disconnect due to inactivity
    peer.allowed_ips.push(IpAddrMask::from_str("0.0.0.0/24")?); // to allow all the traffic to route through the VPN to server
    
    println!("RunClient: Server added as peer"); 

    // interface configuration
    let interface_config = InterfaceConfiguration {
        name: "aws-client-instance".to_string(),
        prvkey: "<CLIENT_PRIVATE_KEY>".to_string(),
        address: "10.0.0.2".to_string(),
        port: 51280,
        peers: vec![peer],
    };

    println!("RunClient: Interface Configured"); 

    wgapi.configure_interface(&interface_config)?;

    wgapi.configure_peer_routing(&interface_config.peers)?;
    
    println!("RunClient: Running Client"); 
    println!("RunClient <===="); 
    Ok(())
}

fn run_server(){
    println!("RunServer: Done"); 
}