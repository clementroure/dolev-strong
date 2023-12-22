
# Dolev-Strong Algorithm Simulation in Rust

## Overview
This Rust program simulates the Dolev-Strong algorithm, a protocol used in distributed systems to ensure reliable message delivery, even in the presence of dishonest nodes. The program creates a network of nodes, some of which may be dishonest, and simulates message passing among them to reach a consensus.

## How It Works

### Structures
- **Node**: Represents a network node with an ID, honesty status, and origin status.
- **Message**: Defines the structure of a message with its content and sender's ID.
- **SharedTable**: A shared structure for storing messages sent and received by nodes.

### Main Logic
1. **Initialization**: The network nodes are initialized, and shared resources (message table and synchronization barrier) are set up for use across threads.
2. **Origin Node Thread**: Sends an initial message to all other nodes and simulates the behavior of the origin node.
3. **Node Simulation**: Each non-origin node runs in a separate thread. They send and receive messages, and decide on a consensus based on the majority of received messages.
4. **Final Processing**: After all epochs, the final messages received by each node are printed.

### Thread Synchronization
- **Mutex**: Protects access to the shared message table.
- **Barrier**: Synchronizes threads at the end of each epoch.

## Running the Program

### Prerequisites
- Ensure you have Rust installed on your machine.

### Steps to Run
1. cargo run
   ```

Observe the output in the terminal to see the decisions made by each node and the messages passed in the network.

## Conclusion
This simulation demonstrates the Dolev-Strong algorithm in a multithreaded context, highlighting concepts like concurrency, synchronization, and distributed consensus in Rust.
