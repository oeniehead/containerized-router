# containerized-router

Attempt at containerizing a router.

To run KPN demo: `sudo docker compose -f kpn-test-compose.yaml up`

To peek in ISP container: `sudo docker compose -f kpn-test-compose.yaml exec isp-kpn sh`

To peek in ingress container: `sudo docker compose -f kpn-test-compose.yaml exec ingress-kpn sh`

To build all containers: `sudo ./build-containers`

Status:
- IPv4 works over PPPoE
- IPv6 implementation started
  - ISP simulator distributes IPv6 prefixes
  - Ingress container has a test config to request prefix with dhcpcd
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
  - Implement DHCPv6-PD service in ingress container
  - Use a reliable service manager for the services, such as supervisord

Notes:
- Kea DDNS requires DDNS server (bind?) for DNS updates
- ISP connection must sync IPv6 prefix to DHCPv6 servers via backstage event bus
- Netavark has an example plugin that moves host network interfaces to the container namespace
