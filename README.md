# Sail

## Overview
Sail is a central limit orderbook built on Fuel. This implementation of an order book uses predicates. This adds some complexity to implemnting an orderbook 
but has very favouable outcomes when trying to build an orderbook that tends towards decentralization. We have a client where someone can call a make order
or take order function. The make order function dynamically creates a predicate that holds a UTXO of a set amount at a set limit. Takers can then call the
take order function fullfiling the given UTXO tranferring the funds to the maker and taker. This approach allows for no state bloat meaning that hardware
requirements to run validators are likely to stay low hopefully meaning it's more decentralized. For order-matching we intend to leave that up to a set of 
external parties to compete ideal creating the best prices for people making trades on the orderbook.

## Repository Structure

The following is a visual sample of how the repository is structured.

```
package/
├── client
|    └── Rust client leveraging fuels-rs
├── contracts
|    └── Sway contracts including a logger, script for distributing funds and library for limit orders
└── indexer
     └── Way to view logs from contract

```
## Comments on the code
There are many improvements that can be made to the current implementation. Firstly, the predicate for each order is being created by formatting a string
using that string to create a sway file and compiling that file. This method even though it works is far from ideal. This can easily be remidied when you
can pass arguments to a predicate from the SDK. Secondly, we are using tests to check for the correct behaviour. We will likely make a testnet deployment
with instructions for how to play around with the orderbook.


## Road ahead
There are a multitude of different things that could be built into and around this base implementation.

## Appreciation
We want to give a huge shoutout to everyone that helped us the last two days. We want to call out Ryan Sproule in particular he gave us some reference code
as this was something he had been hacking on. It's worth taking a look at if this stuff interests you at all. [Ryan's Implementation](https://github.com/BlockchainCap/fuel-order-book)


