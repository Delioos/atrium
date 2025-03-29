# Dynamic LP Hook

A smart contract that enhances Uniswap V4 liquidity provision by automatically reallocating inactive liquidity to lending protocols. Built with Arbitrum Stylus in Rust.

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## Overview

The Dynamic LP Hook is designed to maximize yield for liquidity providers by creating a positive feedback loop:
1. When price moves outside a configured range, liquidity is moved to lending protocols
2. Lending fees compound while liquidity is inactive
3. When price returns to range, liquidity moves back to LP with increased size
4. Larger positions generate more fees, creating a virtuous cycle

## Features

- **TWAP-based Price Monitoring**: Uses time-weighted average price to determine optimal reallocation points
- **Dynamic Reallocation**: Automatically moves liquidity between LP and lending positions
- **Fee Optimization**: Compounds lending fees while maintaining LP exposure
- **Gas Efficient**: Built with Arbitrum Stylus for optimal performance
- **Configurable Parameters**: Adjustable observation period, price range, and reallocation timing

## Architecture

For detailed information about the project's architecture, components, and design decisions, see the [Architecture Documentation](docs/architecture.md).

## Getting Started

### Prerequisites

- Rust toolchain (see `rust-toolchain.toml`)
- Arbitrum Stylus CLI
- Git

### Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/dynamic-lp-hook.git
cd dynamic-lp-hook
```

2. Build the project:
```bash
cargo build
```

3. Run tests:
```bash
cargo test --locked --features std --lib
```

4. Check contract compatibility:
```bash
cargo stylus check
```

### Deployment

To deploy the contract to Arbitrum Stylus:

```bash
cargo stylus deploy --private-key <your-private-key>
```

## Usage

### Initialization

```rust
let hook = DynamicLPHook::new();
hook.initialize(
    lending_protocol_address,
    observation_period,    // e.g., 3600 (1 hour)
    min_reallocation_time, // e.g., 1800 (30 minutes)
    price_range,          // e.g., 100 (1% in basis points)
)?;
```

### Key Functions

- `update_twap(price)`: Update the time-weighted average price
- `check_and_reallocate(current_price)`: Check if position needs reallocation
- `collect_lending_fees()`: Collect and compound lending fees
- `move_to_lp_if_in_range(current_price)`: Return liquidity to LP if price is in range

## Testing

The project includes comprehensive tests for all major functionality:

```bash
cargo test --locked --features std --lib
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Arbitrum Stylus](https://docs.arbitrum.io/stylus)
- [Uniswap V4](https://docs.uniswap.org/contracts/v4/overview)
- [Rust](https://www.rust-lang.org/)
