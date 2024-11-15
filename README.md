# Uniswap Custom Swap Curve

### **A template for writing Uniswap Custom Swap Curves in Stylus**

[`Use this Template`](https://github.com/OpenZeppelin/uniswap-stylus-curve-template/generate)

1. The example curve [lib.rs](src/lib.rs) demonstrates a constant-sum swap curve,
in which tokens are traded exactly 1:1.

> [!TIP]
> You can modify `ICurve` trait based on your needs, or provide your custom algorithm
> for a swap curve calculation.

> [!CAUTION]
> If anything changes in the contract's interface, make sure to update your
> Solidity contract with Uniswap hooks to use the new Solidity ABI Interface of this contract.

## Getting started

Follow the instructions in the [Stylus quickstart](https://docs.arbitrum.io/stylus/stylus-quickstart) to configure your development environment.

You'll also need [Foundry](https://github.com/foundry-rs/foundry) to interact with the contract.

## Check and deploy

You can use [cargo stylus](https://github.com/OffchainLabs/cargo-stylus) to check that your contract is compatible with Stylus by running

```shell
cargo stylus check
```

With the following command you can deploy it to an Arbitrum chain

```shell
cargo stylus deploy --endpoint $RPC_URL --private-key $PRIVATE_KEY
```

## Tests

For unit testing, this example integrates the [motsu](https://github.com/OpenZeppelin/rust-contracts-stylus/tree/main/lib/motsu) library from OpenZeppelin. To run unit tests, you can simply use

```shell
cargo test --locked --features std --lib
```

Alternatively, you can use the bash script available [test-unit.sh](/scripts/test-unit.sh).

## Exporting Solidity ABI Interface

To export the Solidity ABI interface run the following command

```shell
cargo stylus export-abi
```

## How to run a local dev node

Instructions to setup a local dev node can be found [here](https://docs.arbitrum.io/run-arbitrum-node/run-nitro-dev-node).

## Solidity Interface

This is the current Solidity ABI Interface for the contract

```solidity
interface IUniswapCurve {
    function getAmountInForExactOutput(uint256 amount_out, address input, address output, bool zero_for_one) external returns (uint256);

    function getAmountOutFromExactInput(uint256 amount_in, address input, address output, bool zero_for_one) external returns (uint256);

    error CurveCustomError();
}
```
