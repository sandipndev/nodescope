# NodeScope — Total Visibility for your Bitcoin Nodes.

Running a Bitcoin node is powerful, but also opaque. You know it's syncing, you know it's connected, but *what exactly is going on under the hood?*

## What is NodeScope?

NodeScope brings full visibility to your Bitcoin nodes.
You can monitor which peers your node is connected to, observe real-time P2P gossip, and track how your node participates in the network.

Historical data about peer connections, message flows, and data exchange helps you understand network topology, peer quality, and your node’s behavior over time.

NodeScope uses a lightweight proxy, log ingestion, and RPC data collection to give you a complete picture of your node’s network activity.

## Features

- Historical records of peers your node connects to
- Messages sent and received from each peer, with timestamps
- Peer versions and services, and handshakes tracking (even those that fail)

## Getting Started

Eventually (not yet there):

```bash
cargo install nodescope
nodescope daemon --config nodescope.toml
```

We also plan on offering NodeScope binaries as downloads on GitHub Releases, Homebrew, Nix, and other package managers.

## Development

NodeScope is open source! We welcome contributions from the community.
Feel free to open issues, submit pull requests, or suggest new features.
For development setup, please refer to the [contributing](./contributing.md) guide.

## License

NodeScope is open source and available under the MIT License.
See the [license](./license) file for more details.
