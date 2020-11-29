# lanterns

Private Communications No Holds Barred

An experimentation of encryption, comms, and hardware. This project serves as a testbed for securing the transmission of data.

## Stability

Experimental

## Branching Strategy

Trunk Based Development

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

MIT
