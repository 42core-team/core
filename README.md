# Core
This is the "real" server for the CORE game, written in Rust. It handles all game logic, processes actions from the socket, and sends the game state containing all relevant information back to the client.

## Environment Variables

| Variable Name | Description |
|---------------|-------------|
| `PORT`        | Sets the port on which the socket should listen. |
| `LOG`         | Enables logging if set. |
| `SEED`        | Sets the random seed for the game. |
| `TICK_RATE`   | Sets how many ticks per second the server calculates. Also sets the speed through that. |
