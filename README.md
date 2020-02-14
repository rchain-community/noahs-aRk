# Noah's aRk 🚢

* 🕊️ Hope for the Rchain Community
* 🪂 An airdrop to rhoc holders
* 🗳️ An experiment in on-chain governance
* 🔗 Rchain main-net
* 💩 A shitcoin of our own
* 👊 A censure of Lgreg Meredith

## Motivation

Since at least Summer 2018 and probably earlier, we, the Rchain community, a group of people united by our belief in the importance of concurrency in the blockchain and our holdings in the rhoc token, have suffered the poor decision making and dishonesty of Lgreg Meredith, president of the Rchain cooperative, a legal entity in the legacy legal system of the State of Washington.

Being part of the blockchain community, the Rchain community has known all along that we have the power to fork off, but due partly to Stockholm Syndrome, partly to apathy, and largely to inability to coordinate our efforts, have allowed Lgreg to hold the reigns.

Recently, despite a very clear plan to issue rev 1:1 to rhoc, Lgreg has tried to cut out various stakeholders he regards as "scammers" while providing insufficient details about the scams perpetrated. The community has struggled to agree on the issuance of the rev token and is, more than ever, in need of a mechanism of social coordination. For years Lgreg has promised that Rchain will provide this means, but it never launches and the status quo remains.

Noah's aRk, a blockchain built on the [Substrate](https://substrate.dev) framework, will provide token based voting to settle the rev issuance, and any other disputes arising in the community.

aRk tokens will be issued 1:1 with rhoc tokens from ethereum block height 9371743, the same block the coop announced. Motions may be made to cancel any existing tokens, mint new tokens, run taint analyses, and more. The critical difference is that these motions will pass or fail based on token-backed voting. The exact mechanism comes from Substrate's [Democracy pallet](https://substrate.dev/rustdocs/master/pallet_democracy/index.html).

## Disclaimer

This project is in no way supported or endorsed by Parity Technologies.

## Short Term Technical Plan

The code in this repository is currently just a clone of the [Substrate Node Template](https://github.com/substrate-developer-hub/substrate-node-template). It will be developed into a working chain that meets the needs of the Rchain community February 14-16 at ETH Denver. Development will be led by [Joshy Orndorff](https://github.com/joshorndorff/) and anyone who wishes to contribute. A testnet will be launched shortly thereafter, and the testnet will become mainnet if it is able to represent the will of the community.

### The Airdrop

The aRk token will be managed by Substrate's [Balances Pallet](https://substrate.dev/rustdocs/master/pallet_balances/index.html). aRk must be claimed by a process similar to claiming dots on Polkadot or ksm on Kusama. The code that handles these claims is at https://github.com/paritytech/polkadot/blob/master/runtime/common/src/claims.rs but can be simplified to remove the vesting, and updated to use a more recent Substrate API.

### Consensus

Block authoring will be handled by the Babe algorithm with a set of trusted authorities. Authorities will have no external incentive to author blocks, rather they will author because they believe in the project. Authorities may be added to the set by the same governance means as described above. Initially there will be three authorities all run by Joshy, unless other trustworthy and interested parties step forward. The code to manage the validator set is at https://github.com/gautamdhameja/substrate-validator-set and can be easily installed here.

### Governance

As described above, the democracy pallet will be used for gevernance. Initially there will also be a sudo key controlled by Joshy to affect upgrades on short notice as necessary. This sudo key can be revoked by the token holders at any time according to the democracy. The holder of the sudo key can also chose to revoke it at any time.

## Long Term Technical Possibilities

Initially this project strives only to help the community navigate the turbulence that it has experienced over the past few years. It is entirely possible that it will serve that purpose in a few weeks or months and then die. It is also possible that the community will come to value this chain and the simplicity of developing in Rust on Subststrate to the point that it evolves into #TheRealRchain mainnet.

### Consensus

Substrate provides a robust API for plugging consensus algorithms. It is possible to implement CBC Casper, Casanova, or other consensus algorithms in Substrate. Further the chain can be upgraded to use these algorithms after launch.

### Rholang

Substrate provides a robust API for writing runtime logic. The RhoVM could be written in rust, and added to the chain through the same governance process described above.

### Limitless

Anything the community can code in rust, and gain social support for, can be installed on the chain. Let's experiement and see who we really R.







## Build

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

## Run

### Single Node Development Chain

Purge any existing developer chain state:

```bash
./target/release/node-template purge-chain --dev
```

Start a development chain with:

```bash
./target/release/node-template --dev
```

Detailed logs may be shown by running the node with the following environment variables set: `RUST_LOG=debug RUST_BACKTRACE=1 cargo run -- --dev`.

### Multi-Node Local Testnet

If you want to see the multi-node consensus algorithm in action locally, then you can create a local testnet with two validator nodes for Alice and Bob, who are the initial authorities of the genesis chain that have been endowed with testnet units.

Optionally, give each node a name and expose them so they are listed on the Polkadot [telemetry site](https://telemetry.polkadot.io/#/Local%20Testnet).

You'll need two terminal windows open.

We'll start Alice's substrate node first on default TCP port 30333 with her chain database stored locally at `/tmp/alice`. The bootnode ID of her node is `QmRpheLN4JWdAnY7HGJfWFNbfkQCb6tFf4vvA6hgjMZKrR`, which is generated from the `--node-key` value that we specify below:

```bash
cargo run -- \
  --base-path /tmp/alice \
  --chain=local \
  --alice \
  --node-key 0000000000000000000000000000000000000000000000000000000000000001 \
  --telemetry-url ws://telemetry.polkadot.io:1024 \
  --validator
```

In the second terminal, we'll start Bob's substrate node on a different TCP port of 30334, and with his chain database stored locally at `/tmp/bob`. We'll specify a value for the `--bootnodes` option that will connect his node to Alice's bootnode ID on TCP port 30333:

```bash
cargo run -- \
  --base-path /tmp/bob \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/QmRpheLN4JWdAnY7HGJfWFNbfkQCb6tFf4vvA6hgjMZKrR \
  --chain=local \
  --bob \
  --port 30334 \
  --telemetry-url ws://telemetry.polkadot.io:1024 \
  --validator
```

Additional CLI usage options are available and may be shown by running `cargo run -- --help`.
