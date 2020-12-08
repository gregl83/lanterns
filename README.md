# lanterns

Private Communications No Holds Barred

An experimentation of encryption, comms, and hardware. This project serves as a testbed for securing the transmission of data.

## Stability

Experimental

## Branching Strategy

[Trunk Based Development](https://trunkbaseddevelopment.com/)

## Installation

1. [Install Rust and Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
2. Git clone this repository
3. From cloned repository, run:
   `cargo install --path=.`
4. That's it, `lanterns` should now be installed on your system!

## Usage

Run `lanterns`.

## Problem Statement

In the dawn of mobile communications, distributed systems and IoT, privacy has become an afterthought. Under a guise of enormous complexity, from circuitry to software, governing bodies, both private and civic, have demanded unrestrained access to personal data without checks and balances.

## Research

- Encryption Algorithm Viability
  - Security
    - Strength
    - Vulnerabilities
    - Future Proofing
  - Computational Cost
  - Implementation Cost
  - Multilayered Cost vs Benefit
 - Encryption Candidates
   - Key Exchange
     - Rivest–Shamir–Adleman (RSA)
     - Elliptic-curve Diffie–Hellman (ECDH)
   - Message Exchange
     - Rijndael - Advanced Encryption Standard (AES)
- Data Transmission
  - Bluetooth
    - Range Reduction
    - Server-side attack vector (client target)
  - Multi-wire planar cable (Ribbon Cable)
    - Raspberry Pi Two-wire GPIO
    - Custom Driver
  - Faraday Shield
- Hardware
  - Custom printed circuit board (PCB)
  - Raspberry Pi Zero sans WiFi
  - Mobile device w/Termux 
  - Laptop
  - Device Shield
    - Electromagnetic (Farady)
    - Acoustic

## Development

- CLI GUI
  - Persistent configurations
  - Reactive state management
  - User input handling
  - Components and Pages
- Terminal Chat
- Lantern
- Bluetooth Adapter
  - Search Devices
  - Pair with Device
  - Send / Receive
- WiFi Adapter
  - Search Devices
  - Connect Securely to Device
  - Send / Receive

## Architecture

![lanterns architecture](assets/lanterns-architecture.png)

### Payload Workflow

1. Offline Sender Terminal layer *0* encrypts payload
2. Offline Sender Terminal sends payload to Online Sender Lantern
3. Online Sender Lantern layer *1* encrypts payload
4. Online Sender Lantern sends payload to Online Receiver Lantern
5. Online Receiver Lantern layer *1* decrypts payload
6. Online Receiver Lantern sends payload to Offline Receiver Lantern
7. Offline Receiver Lantern layer *0* decrypts payload

## License

[MIT](LICENSE)
