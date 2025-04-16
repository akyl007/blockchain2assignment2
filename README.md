# Solana Hello World Program

A simple Solana program demonstrating counter functionality.

## Technology Stack

### Core Technologies
- **Solana**: v1.17.15
- **Rust**: v1.72.0
- **Docker**: Ubuntu 22.04
- **BPF (Berkeley Packet Filter)**: For Solana program execution
- **Web3.js**: For client-side interaction

### Project Dependencies
```toml
[dependencies]
solana-program = "1.17.15"
bumpalo = "3.12.0"
arrayref = "0.3.6"
thiserror = "1.0"
```

image.png

## Environment Requirements

### System Requirements
- Docker Desktop for Windows
- Git
- Minimum 4GB RAM
- 20GB free disk space

### Tool Versions
- Docker: Latest stable version
- Solana CLI: v1.17.15
- Rust: v1.72.0
- Ubuntu: 22.04 (in container)

## Installation and Setup

### 1. Clone Repository
```bash
git clone <repository-url>
cd solana-hello-world
```

### 2. Start Docker Container
```bash
docker-compose up -d
```

### 3. Enter Container
```bash
docker exec -it solana_dev bash
```

## Project Structure

```
solana-hello-world/
├── Dockerfile              # Docker image configuration
├── docker-compose.yml      # Docker Compose configuration
├── solana/                 # Program source code
│   ├── Cargo.toml         # Rust dependencies
│   ├── Cargo.lock         # Dependency versions
│   └── src/
│       └── lib.rs         # Main program code
├── scripts/               # Project scripts
│   ├── build.sh          # Build script
│   ├── deploy.sh         # Deploy script
│   └── create_wallet.sh  # Wallet creation script
└── README.md             # Documentation
```

## Development

### Building the Project
```bash
cd /project/solana
cargo build-bpf
```

### Deploying to Devnet
```bash
solana config set --url devnet
solana program deploy target/deploy/solana_hello_world.so
```

### Checking Versions
```bash
solana --version
cargo --version
rustc --version
```

## Web3.js Integration

### Client-Side Setup
```javascript
import { Connection, PublicKey, Transaction } from '@solana/web3.js';

// Connect to devnet
const connection = new Connection('https://api.devnet.solana.com');

// Program ID (replace with your deployed program ID)
const programId = new PublicKey('YOUR_PROGRAM_ID');

// Create transaction
const transaction = new Transaction().add(
    // Add your instruction here
);

// Sign and send transaction
const signature = await connection.sendTransaction(transaction, [payer]);
```

### Example Usage
```javascript
// Initialize counter
const initCounter = async () => {
    const transaction = new Transaction().add(
        // Add initialization instruction
    );
    await connection.sendTransaction(transaction, [payer]);
};

// Increment counter
const incrementCounter = async () => {
    const transaction = new Transaction().add(
        // Add increment instruction
    );
    await connection.sendTransaction(transaction, [payer]);
};
```

## Live Demo Instructions

### Prerequisites
1. Docker Desktop running
2. Solana CLI installed
3. Sufficient SOL on devnet

### Demo Steps
1. **Show Environment Setup**
   ```bash
   solana --version
   solana config get
   ```

2. **Deploy Program**
   ```bash
   # Build program
   cargo build-bpf
   
   # Deploy to devnet
   solana program deploy target/deploy/solana_hello_world.so
   ```

3. **Interact with Program**
   ```bash
   # Show wallet address
   solana address
   
   # Check balance
   solana balance
   
   # Show program logs
   solana logs <PROGRAM_ID>
   ```

4. **Web3.js Demo**
   ```bash
   # Start client application
   npm start
   
   # Show counter initialization
   # Show counter increment
   # Show transaction history
   ```

### Screenshots to Capture
1. Solana CLI version
2. Wallet address
3. Program deployment success
4. Transaction history
5. Counter state changes

## Program Architecture

### Main Components
1. **Counter**: Data structure for storing counter
   - `is_initialized`: Initialization flag
   - `count`: Counter value

2. **Process Instruction**: Program entry point
   - Account owner verification
   - Signature verification
   - Counter increment

### Security
- Program owner verification
- Transaction signature verification
- Counter overflow handling

## Testing

### Local Testing
```bash
cargo test
```

### Devnet Testing
1. Deploy program
2. Create test transactions
3. Verify counter state

## Troubleshooting

### Common Issues
1. **BPF Build Error**
   - Check Rust version
   - Verify all dependencies

2. **Deploy Error**
   - Check wallet balance
   - Verify devnet configuration

3. **Execution Errors**
   - Check permissions
   - Verify data structure

## License

MIT License

## Contact

[Contact information here]

## Changelog

### v1.0.0
- Initial release
- Basic counter functionality
- Docker support 