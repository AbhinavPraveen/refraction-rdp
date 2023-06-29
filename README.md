# About

refraction-rdp is a secure Remote Desktop Solution. refraction-rdp is a simple wrapper around [Sunshine](https://github.com/LizardByte/Sunshine) and [Moonlight](https://github.com/moonlight-stream/moonlight-qt) which provides either with its own exclusive [Wireguard](https://www.wireguard.com/) interface.

# Advantages

- [Wireguard](https://www.wireguard.com/), a state-of-the-art VPN protocol, is responsible for authentication and encryption hence providing state-of-the-art security and [high performance](https://www.wireguard.com/performance/).
- [Sunshine](https://github.com/LizardByte/Sunshine) and [Moonlight](https://github.com/moonlight-stream/moonlight-qt) provide very high performance (hardware-encoded/decoded) streams.
- Only the Wireguard port on the host has to be port forwarded and allowed through the firewall.
- Only Sunshine and Moonlight have access to their respective Wireguard interfaces.

# Dependencies

- surf
- iproute2
- wireguard-tools
- sunshine
- moonlight

# Documentation

Further documentation can be found in [docs](docs).
