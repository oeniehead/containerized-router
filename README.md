# containerized-router

Attempt at containerizing a router.

To run KPN demo: `sudo docker compose -f kpn-test-compose.yaml up`

To peek in ISP container: `sudo docker compose -f kpn-test-compose.yaml exec isp-kpn sh`

To peek in ingress container: `sudo docker compose -f kpn-test-compose.yaml exec ingress-kpn sh`

To build all containers: `sudo ./build-containers`

Status:
- IPv4 works over PPPoE
- IPv6 works
- There is a non-functional event sender when IPv6-PD has succeeded
- Everything now uses dinit for services, because this actually supports dependencies
- Having all services in separate containers is not viable due to dependencies between services
  - Splitting the ingress container from the rest of the firewall is doable, but this does require an out-of-band channel to communicate the IPv6-PD prefix to the firewall
  - The firewall will run the DHCP and DNS servers
- The ingress container will be linked to the firewall via a DMZ VLAN
  - This means the ingress container will request an IP from the firewall as well
  - This helps with port forwarding on the firewall side
  - This also makes the forwarding between internet and intranet simpler to implement
  - It might be interesting to have an additional DHCPv6 server running that distributes fc:: addresses

TODO:
  - Implement IP forwarding in the ingress container
  - Implement event handler to communicate public IPv4 and IPv6-PD with firewall/DHCP servers
  - Check if all logging is correctly forwarded to the stdout of the container:
    - For ifup this seems not to be the case
  - Start formalizing a configuration scheme for the firewall that handles:
    - IPv4 assignments and IPv6 prefixes
    - Firewall rules
    - Port forwards
    - DHCP servers
    - DNS servers

Notes:
- Kea DDNS requires DDNS server (bind?) for DNS updates
- ISP connection must sync IPv6 prefix to DHCPv6 servers via backstage event bus
- Netavark has an example plugin that moves host network interfaces to the container namespace

# Network design

Network is split into multiple VLANs. Each VLAN has a local IPv4 prefix and a local IPv6 prefix. IPv4 addresses are handed out by DHCP. IPv6 addresses are distributed by SLAAC + RDNSS. IPv6 explicitly only distributes ULA addresses to allow for multihoming. This means that radvd must run on the firewall. DHCP services are moved to a container.

Ingress containers are plugged into the DMZ network and will be used as gateways. The firewall decides which ingress to use for outbound traffic. The ingresses will use NPt to translate from ULA prefixes to GUA prefixes appropriate for the ingress. For IPv4, the ingress will simply use NAT.

Ingresses will always use NAT for IPv4, meaning that they maintain a port forwarding list for incoming traffic on IPv4 and perform NAT for outgoing traffic. For IPv6 either NPt is used if IPv6-PD is supported upstream, otherwise a form of NAT6 can be used (for VPNs for instance).

A benefit of this approach is that little knowledge of external addresses is needed inside the network. It can also provide mostly seamless failover support for both IPv4 and IPv6.
