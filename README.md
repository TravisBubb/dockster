# Dockster

Dockster is a lightweight, educational container runtime written in Rust. It aims to provide a simplified yet functional alternative to Docker, focusing on learning and understanding containerization, namespaces, cgroups, and other low-level OS features.

## Features

- Containerization Basics: Implements core container functionalities like process isolation, namespaces, and cgroups.

- Image Management: Supports creating and running containers from simple filesystem snapshots.

- Networking: Provides basic networking capabilities for containers.

- Resource Constraints: Leverages cgroups to limit CPU and memory usage.

- CLI Interface: Simple command-line interface to manage containers.

- Self-Contained & Minimal: Designed for educational purposes and easy extensibility.

## Goals

- Learn Rust Programming: Utilize Rust's safety guarantees and performance to build a robust container runtime.

- Understand Container Internals: Gain a deeper knowledge of how containerization works under the hood.

- Provide a Minimal Alternative: Offer a lightweight and self-hosted runtime for experimentation and learning.

## Roadmap

1. Process Isolation: Implement basic process spawning with namespace isolation.

2. Filesystem Isolation: Use chroot and mount namespaces for containerized filesystems.

3. Resource Control: Integrate cgroups for limiting CPU and memory usage.

4. Networking: Implement virtual networking interfaces for containers.

5. Container Lifecycle Management: Start, stop, and inspect containers.

6. Image Support: Basic support for pulling and storing container images.

7. CLI Enhancements: Improve user experience and extend command options.

## Installation

Dockster is in active development. To build and run Dockster from source:

```bash
# Clone the repository
git clone https://github.com/yourusername/dockster.git
cd dockster

# Build the project
cargo build --release

# Run Dockster Daemon
./target/release/docksterd

# Run Dockster CLI
./target/release/dockster
```

## Contributing

Contributions, discussions, and learning together are highly encouraged! Feel free to submit issues, pull requests, or suggestions.

## License

Dockster is licensed under the MIT License.

## Disclaimer

Dockster is an educational project and not intended for production use. Use it at your own risk.
