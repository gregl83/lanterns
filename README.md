# lanterns

Private Communications No Holds Barred

## Stability

Experimental

## Branching Strategy

Trunk Based Development

## Architecture

![lanterns architecture](assets/lanterns-architecture.png)

### Payload Workflow

1. Offline Sender layer *n* encrypts payload
2. Offline Sender sends payload to Online Sender (Bluetooth)
3. Online Sender layer *0* encrypts payload
4. Online Sender sends payload to Online Receiver
5. Online Receiver layer *0* decrypts payload
6. Online Receiver sends payload to Offline Receiver (Bluetooth)
7. Offline Receiver layer *n* decrypts payload

## License

MIT