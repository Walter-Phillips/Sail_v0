# Sail

## Overview

Sail is a central limit order book built on Fuel. This implementation of an order book uses predicates. This adds some complexity to implementing an order book but has very favorable outcomes when trying to build an order book that tends towards decentralization. We have a client where someone can call a make order or take order function. The make order function dynamically creates a predicate that holds a UTXO of a set amount at a set limit. Takers can then call the take order function fulfilling the given UTXO and transferring the funds to the maker and taker. This approach allows for no state bloat meaning that hardware requirements to run validators are likely to stay low hopefully meaning it's more decentralized. For order-matching, we intend to leave that up to a set of external parties to compete ideally creating the best prices for people making trades on the order book.

## Repository Structure

The following is a visual sample of how the repository is structured.

```
package/
├── rust-client
|    └── Rust client leveraging fuels-rs
├── contracts
|    └── Sway contracts including a logger, script for distributing funds and library for limit orders
└── indexer
     └── Way to view logs from contract

```

## Comments on the code

In it's current state the generation isn't working because of type incompatibility. With some slight refactoring this should not be an issue.

There are othe improvements that can be made to the current implementation. Firstly, the predicate for each order is being created by formatting a string
and using that string to create a sway file and compiling that file. This method ,even though it works, is far from ideal. This can easily be remidied when you
can pass arguments to a predicate from the SDK. Secondly, we are using tests to check for the correct behaviour. We will likely make a testnet deployment
with instructions for how to play around with the orderbook.

## Appreciation

We want to give a huge shoutout to everyone that helped us the last two days. We want to call out Ryan Sproule in particular he gave us some reference code
as this was something he had been hacking on. It's worth taking a look at if this stuff interests you at all. [Ryan's Implementation](https://github.com/BlockchainCap/fuel-order-book)

## Future Development

We've our development to a new  private repo. We'll be open-sourcing everything we're working on in the near future.
If you have any questions for the time being, please [Contact Us](mailto:support@sail.exchange). We'll have updates soon!!!
