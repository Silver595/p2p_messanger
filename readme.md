# Iron Messenger CLI

Iron Messenger CLI is a peer-to-peer chat application built using the `iroh-gossip` library. It allows users to create or join chat rooms and exchange messages over a gossip protocol.

## Features

- Create or join chat rooms using a ticket system.
- Broadcast messages to all participants in a chat room.
- Peer-to-peer communication using the `iroh` and `iroh-gossip` libraries.
- Asynchronous and concurrent message handling.

## Installation

1. Ensure you have [Rust](https://www.rust-lang.org/) installed.
2. Clone this repository:
   ```bash
   git clone https://github.com/Silver595/p2p_messanger.git
   cd iron-messenger-cli
   ```
3. Build the project:
   ```bash
   cargo build --release
   ```

## Usage

### Open a Chat Room

To create a new chat room and generate a ticket for others to join:
```bash
./iron-messenger-cli open
```
or
```
cargo run open
```
### Join a Chat Room

To join an existing chat room using a ticket:
```bash
./iron-messenger-cli join --ticket <ticket>
```
or 

```
cargo run join <ticket>
```

### Set a Nickname

You can set your nickname using the `--name` option:
```bash
./iron-messenger-cli --name "YourName" open
```
or

```
 cargo run -- --name user
```


### Example Workflow

1. **User A** creates a chat room:
   ```bash
   ./iron-messenger-cli open
   ```
   Output:
   ```
   > opening chat room for topic <topic>
   > ticket to join us: <ticket>
   ```

2. **User B** joins the chat room using the ticket:
   ```bash
   ./iron-messenger-cli join --ticket <ticket>
   ```

3. Both users can now exchange messages.

## Development

### Dependencies

- `anyhow`: Error handling.
- `clap`: Command-line argument parsing.
- `data-encoding`: Encoding/decoding tickets in Base32.
- `futures-lite`: Asynchronous streams.
- `iroh` and `iroh-gossip`: Peer-to-peer networking.
- `rand`: Random number generation.
- `serde` and `serde_json`: Serialization and deserialization.
- `tokio`: Asynchronous runtime.

### Code Structure

- `src/main.rs`: Contains the entire application logic, including:
  - CLI parsing.
  - Gossip protocol setup.
  - Message broadcasting and receiving.
  - Ticket encoding/decoding.

## License

This project is licensed under the MIT License. See the LICENSE file for details.