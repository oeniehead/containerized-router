//! This is just an example plugin, do not use it in production!

use std::{collections::HashMap, os::fd::AsFd};

use netavark::{
    network::{
        core_utils::{open_netlink_sockets, CoreUtils},
        netlink, types,
    },
    new_error,
    plugin::{Info, Plugin, PluginExec, API_VERSION},
};
use netlink_packet_route::{address::AddressAttribute, link::LinkAttribute};

fn main() {
    let info = Info::new("0.1.0-dev".to_owned(), API_VERSION.to_owned(), None);

    PluginExec::new(Exec {}, info).exec();
}

struct Exec {}

impl Plugin for Exec {
    fn create(
        &self,
        network: types::Network,
    ) -> Result<types::Network, Box<dyn std::error::Error>> {
        if network.network_interface.as_deref().unwrap_or_default() == "" {
            return Err(new_error!("no network interface is specified"));
        }

        Ok(network)
    }

    fn setup(
        &self,
        netns: String,
        opts: types::NetworkPluginExec,
    ) -> Result<types::StatusBlock, Box<dyn std::error::Error>> {
        let (mut host, mut netns) = open_netlink_sockets(&netns)?;

        let name = opts.network.network_interface.unwrap_or_default();

        let link = host.netlink.get_link(netlink::LinkID::Name(name.clone()))?;

        let mut mac_address = String::from("");
        for nla in link.attributes {
            if let LinkAttribute::Address(ref addr) = nla {
                mac_address = CoreUtils::encode_address_to_hex(addr);
            }
        }

        let addresses = host.netlink.dump_addresses()?;
        let mut subnets = Vec::new();
        for address in addresses {
            if address.header.index == link.header.index {
                for nla in address.attributes {
                    if let AddressAttribute::Address(ip) = &nla {
                        let net = ipnet::IpNet::new(*ip, address.header.prefix_len)?;
                        subnets.push(types::NetAddress {
                            gateway: None,
                            ipnet: net,
                        })
                    }
                }
            }
        }

        host.netlink
            .set_link_ns(link.header.index, netns.file.as_fd())?;

        // Retrieve the new interface name from options
        let new_name = opts.network.options
            .as_ref()
            .and_then(|opts| opts.get("new_interface_name"))
            .unwrap_or(&name);

        // Rename the interface in the container's namespace
        netns.netlink.set_link_name(link.header.index, new_name.to_string())?;

        // interfaces map, but we only ever expect one, for response
        let mut interfaces: HashMap<String, types::NetInterface> = HashMap::new();

        let interface = types::NetInterface {
            mac_address,
            subnets: Option::from(subnets),
        };
        interfaces.insert(name, interface);

        //  StatusBlock response
        let response = types::StatusBlock {
            dns_server_ips: None,
            dns_search_domains: None,
            interfaces: Some(interfaces),
        };

        Ok(response)
    }

    fn teardown(
        &self,
        netns: String,
        opts: types::NetworkPluginExec,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // on teardown revert what was done in setup
        let (mut host, mut netns) = open_netlink_sockets(&netns)?;

        // Retrieve the current interface name from the options
        let current_name = opts.network.network_interface.unwrap_or_default();

        // Get the link (network interface) in the container's namespace
        let link = netns.netlink.get_link(netlink::LinkID::Name(current_name.clone()))?;

        netns
            .netlink
            .set_link_ns(link.header.index, host.file.as_fd())?;

        // Retrieve the original interface name from the options
        let original_name = opts.network.options
            .as_ref()
            .and_then(|opts| opts.get("original_interface_name"))
            .unwrap_or(&current_name);

        // Rename the interface back to its original name in the host's namespace
        host.netlink.set_link_name(link.header.index, original_name.to_string())?;

        Ok(())
    }
}
