# Noah's aRk User Guide

## Building the Node

Install Rust:

```bash
curl https://sh.rustup.rs -sSf | sh
```

Initialize your Wasm Build environment:

```bash
./scripts/init.sh
```

Build Wasm and native code:

```bash
cargo build --release
```

## Running the Node

Always take advantage of the help page.
```bash
./target/release/noahs-ark --help
```

More details about running Substrate nodes are available in the Substrate documentation:

* [Creating your first Substrate chain](https://substrate.dev/docs/en/tutorials/creating-your-first-substrate-chain/)
* [Start a private network with Substrate](https://substrate.dev/docs/en/tutorials/start-a-private-network/)

### Single Node Development Chain

Purge any existing developer chain state:

```bash
./target/release/noahs-ark purge-chain --dev
```

Start a development chain with:

```bash
./target/release/noahs-ark --dev
```

Detailed logs may be shown by running the node with the following environment variables set: `RUST_LOG=debug RUST_BACKTRACE=1 ./target/release/noahs-ark --dev --dev`.

### Multi-Node Local Testnet

If you want to see the multi-node consensus algorithm in action locally, then you can create a local testnet with two validator nodes for Alice and Bob, who are the initial authorities of the genesis chain that have been endowed with testnet units.

Optionally, give each node a name and expose them so they are listed on the Polkadot [telemetry site](https://telemetry.polkadot.io/#/Local%20Testnet).

You'll need two terminal windows open.

We'll start Alice's substrate node first on default TCP port 30333 with her chain database stored locally at `/tmp/alice`. The bootnode ID of her node is `QmRpheLN4JWdAnY7HGJfWFNbfkQCb6tFf4vvA6hgjMZKrR`, which is generated from the `--node-key` value that we specify below:

```bash
./target/release/noahs-ark \
  --base-path /tmp/alice \
  --chain=local \
  --alice \
  --node-key 0000000000000000000000000000000000000000000000000000000000000001 \
  --telemetry-url ws://telemetry.polkadot.io:1024 \
  --validator
```

In the second terminal, we'll start Bob's substrate node on a different TCP port of 30334, and with his chain database stored locally at `/tmp/bob`. We'll specify a value for the `--bootnodes` option that will connect his node to Alice's bootnode ID on TCP port 30333:

```bash
./target/release/noahs-ark \
  --base-path /tmp/bob \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/QmRpheLN4JWdAnY7HGJfWFNbfkQCb6tFf4vvA6hgjMZKrR \
  --chain=local \
  --bob \
  --port 30334 \
  --telemetry-url ws://telemetry.polkadot.io:1024 \
  --validator
```

## Using the Interface

Currently the best inerface is **Polkadot-JS Apps**.

* Hosted UI: https://polkadot.js.org/apps
* Code on github: https://github.com/polkadot-js/apps
* Docs at: https://polkadot.js.org/REPOS.html

Initially, Apps will be connected to the Kusama network. You can change that to your local node on the `Settings` page, or by specifying it in the URL https://polkadot.js.org/apps?rpc=ws://127.0.0.1:9944

Not sure how well this works with our chain yet.

### Claiming aRk

You don't start with any aRk tokens. You claim your aRk by providing a valid signature with the ethereum key that owned them.

Start by creating a new account (keypair) on the `Accounts tab`. This is the account that your claimed aRk will be transferred into. Copy your address by clicking the identicon.

Now navigate to the `Claims` tab. After selectig your account, it will ask you to sign a message of the following form. The public key associated with your account is hex encoded _without_ the 0x prefix.
```
Pay aRk tokens to account:<public key>
```

Detailed docs for the nearly identical process of claiming Kusama tokens:  https://guide.kusama.network/en/latest/start/dot-holders/

Example Ethereum accounts and signatures are provided for the testnet profiles in [demo-keys.md](./demo-keys.md).

### Vesting

Tokens are vested when they are claimed. I'm open to suggestions about the vesting schedule (but it has to be linear unless you code it yourself).

* https://substrate.dev/rustdocs/master/pallet_vesting/index.html
* https://github.com/paritytech/substrate/blob/2e8080e2902fc477bbce36512a8f5bcdc4b49f17/frame/vesting/src/lib.rs

### Making Proposals

* Got to Democracy Tab (https://polkadot.js.org/apps/#/democracy) and click `Submit Preimage`
* Select your proposal from the dropdowns
* Remember the hash, and `Send Transaction`
* Click `Submit Proposal`
* Use the hash from before, and select how many aRk you're willing to stake
* Click `Submit Proposal`

### Seconding Proposals

It's pretty straightforward on the Democracy tab. There is a button for it.

### Voting

Also on the democray tab.

I'm not sure the strings describing the lockup periods will match ours. TBD.

## Valaidating

To be a validator you must be either:

* Specify in the genesis config ([example](https://github.com/rchain-community/noahs-aRk/blob/master/src/chain_spec.rs#L145))
-OR-
* Add via a root call to `vallidator-set::add_validator`

### From Genesis

Start your node with the correct chainspec first. Then use Apps `Toolbox` tab -> `RPC Calls` -> `author` -> `insertKeys`. A tutorial for this technique is available https://substrate.dev/docs/en/tutorials/start-a-private-network/customchain#add-keys-to-keystore.

### After Genesis

* Call `Toolbox` tab -> `RPC Calls` -> `author` -> `rotateKeys` RPC.
* Call `Extrinsics` tab -> `Session` -> `setKeys`. The proof can be blank.

Now that you're _prepared_ to validate, you can [submit a proposal](#making-proposals) to be made a validator.
