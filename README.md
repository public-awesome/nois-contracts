# Nois Contracts

This is the development repo for the Nois contracts, including a fully featured
test environment that allows testing contract to contract IBC communication.

## The chains

There are two CosmWasm-enabled blockchains running locally.

1. **The randomness chain:** This is where randomness is verified and
   distributed from. Uses noisd.
2. **The app chain:** This is where the users deploy their contracts and request
   the randomness from. Currently this uses wasmd. An example for app chains in
   production would be Juno, Terra or Tgrade.

## The contracts

- nois-gateway (runs on the randomness chain; one instance globally) Steps:
  - It receives the beacon request from a relayer.
  - Routes to the request to the router
  - The router checks if the drand round is already in state.
    - If it is not
      - Enqueues a new job
      - Instructs the payment contract to pay for the randomness
      - Waits for the round to come
      - Dequeues the job
      - Sends a callback packet to the proxy
    - If it is then
      - Instructs the payment contract to pay for the randomness
      - Send the beacon as an IBC acknowledgment packet
- nois-payment (runs on the randomness chain; one instance per randomness
  consumer proxy/outpost)
  - This contract gets created during IBC channel creation between a proxy and
    nois-gateway.
  - It holds the funds for a specific customer. It plays the role of the
    randomness credit/balance.
  - If this contract holds enough funds the randomness request is processed else
    it is rejected.
  - The used funds are then partly burnt/ sent to the community pool or used to
    incentivise the relayer.
- nois-drand is the entrypoint to the randomness beacon coming from drand
  through the drand bots. (runs on the randomness chain; one instance globally)
  - it verifies the randomness before it is stored on chain.
  - it initiates the callback messages to consumer DAPPS on randomness consumer
    chains (juno, stargaze ...) whenever the randomness is available. this
    callback is sent to the nois-proxy which is deployed on the consumer chain.
- nois-icecube holds an initial coin supply that helps incentivise certain
  agents. The supply of this contract is designed to fade away throughout the
  time like an icecube(runs on the randomness chain; one instance globally)
  - nois-icecube gets called by nois-drand to request somefunds to incentivise
    drand bots upon submission of the randomness.
  - it has an initial nois coin supply that an admin multisig can delegate,
    unbond or redelegate but can never withdraw.
- nois-proxy (runs on the app chain; one instance per app chain)
  - nois-proxy receives randomness requests from the consumer DAPPs and submits
    those requests to the nois chain via IBC.
  - Once the randomness is available, the nois-proxy receives a callback and
    forwards it to the DAPP
- nois-demo (runs on the app chain; a demo app)
  - nois-demo is a demo/example consumer app used as a test to estimate the
    value of pi.
  - it submits a request to the proxy to get randomness.
  - The randomness is then received to the nois-demo as a callback.

The IBC interaction is only between nois-gateway and nois-proxy, such that the
user (nois-demo) does not need to worry about that part.

## Packages

- nois (standard library for interacting with Nois)<br />
  [![nois on crates.io](https://img.shields.io/crates/v/nois.svg)](https://crates.io/crates/nois)
- nois-protocol (the Nois IBC protocol)

## Compatibility

The nois standard library and the nois-contracts are versioned independently to
avoid unnecessary disruption for the dapp builders. The following table shows
compatibility.

| nois-contracts version | nois version |
| ---------------------- | ------------ |
| 0.13.x                 | ^0.7.0       |
| 0.12.x                 | ^0.6.0       |
| 0.11.x                 | ^0.6.0       |
| 0.10.x                 | ^0.6.0       |
| 0.9.x                  | ^0.6.0       |
| 0.8.x                  | ^0.6.0       |
| 0.7.x                  | ^0.5.1       |
| 0.6.x                  | ^0.5.0       |
| 0.5.x                  | ^0.5.0       |

## Development

Follow all those steps to test things.

### Build the contracts

The repo root is a Rust workspace containing all the contracts. Basic tests can
be run like this:

```
cargo build --all-targets
cargo clippy --all-targets
cargo fmt
```

The production grade Wasm builds are compiled with:

```
#If you are running on OSX you might need to first run "brew install coreutils"
./devtools/build_integration_wasm.sh
```

### Starting/stopping the chains

In terminal 1 run:

```
./ci-scripts/nois/start.sh
```

which will log in `debug-nois.log`.

In terminal 2 run:

```
./ci-scripts/wasmd/start.sh
```

which will log in `debug-wasmd.log`.

In terminal 3 with `docker ps` you can see the running chains.
`docker kill nois` and `docker kill wasmd` allows you to stop them.

### Run tests

The tests are written in JavaScript in the `./tests` folder

In terminal 3 run:

```
cd tests
npm install
npm run test
```

That's it 🎉

## Production build

This is a regular CosmWasm workspace. Use the latest version of
[workspace-optimizer](https://github.com/CosmWasm/rust-optimizer) to build it.

```
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/workspace-optimizer:0.12.13
```
