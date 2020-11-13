# lanterns

Private Communications No Holds Barred

## Stability

Experimental

## Branching Strategy

Trunk Based Development

## Architecture

1. offline sender layer-1 encrypts payload
2. offline sender sends payload to online sender lantern
3. online sender lantern layer-2 encrypts payload
4. online sender lantern sends payload to online receiver lantern
5. online receiver lantern layer-2 decrypts message
6. online receiver lantern sends message to offline receiver
7. offline receiver layer-1 decrypts payload

![lanterns architecture](assets/lanterns-architecture.png)

## License

MIT