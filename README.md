# containerized-router

Attempt at containerizing a router.

To run KPN demo: `sudo docker compose -f kpn-test-compose.yaml up`

To peek in ISP container: `sudo docker compose -f kpn-test-compose.yaml exec isp-kpn sh`

To peek in ingress container: `sudo docker compose -f kpn-test-compose.yaml exec ingress-kpn sh`

To build all containers: `sudo ./build-containers`

Status:
- IPv4 works over PPPoE
- IPv6 implementation started
  - ISP has Kea DHCPv6 server to do Prefix Delegation
  - ISP has radvd daemon to answer Router Solicitation packets
  - ppp_bridge interface in ISP seems to swallow all Router Advertisement packets however, unsure why
  - ingress is configured with some dhcpcd config to perform a demo PD request, but it needs RA to work
  - later on the PD part needs to move out of the ingress container and into a place that will distribute IP's to the rest of the network

Notes:
- Kea DDNS requires DDNS server (bind?) for DNS updates
- ISP connection must sync IPv6 prefix to DHCPv6 servers via backstage event bus
- Netavark has an example plugin that moves host network interfaces to the container namespace
