# Elys Localnet Setup Guide

This guide provides instructions on how to spin up a new localnet using the Elys network for development purposes. Follow these steps to set up your environment, build contracts, and deploy them to your localnet.

## Prerequisites

- Make sure you have `git`, `make`, Go environment, and `jq` installed on your machine.

## Getting Started

1. **Clone the Elys Repository**

   First, clone the Elys repository to your local machine:

   ```bash
   git clone https://github.com/elys-network/elys.git
   ```

2. **Build the Binary**

   Navigate into the cloned repository and build the binary using:

   ```bash
   make install
   ```

   This command will install the `elysd` daemon.

3. **Download the Latest Snapshot**

   Grab the latest snapshot available for the Elys testnet from the [snapshots](https://polkachu.com/testnets/elys/snapshots) page.

4. **Spin Up the Localnet**

   Use the download snapshot link to spin up your localnet:

   ```bash
   go run ./scripts/upgrade-assure/... <SNAPSHOT_URL> ~/go/bin/elysd ~/go/bin/elysd --skip-proposal
   ```

   Ensure to replace `<SNAPSHOT_URL>` with the actual URL of the downloaded snapshot, e.g., `https://snapshots.polkachu.com/testnet-snapshots/elys/elys_5789265.tar.lz4`.

## Deploying Contracts

1. **Build Contracts**

   From the `bindings` repository, build all three contracts binaries:

   ```bash
   ./scripts/build.sh
   ```
   ***Note***: For Dev and testing purpose you can use a fast build script which create unoptimized but fast build of contract in terms of time.
   
    ```bash
   ./scripts/fast_build.sh
   ```

2. **Deploy Contracts**

   Deploy the contracts to your localnet:

   ```bash
   ./scripts/deploy.sh
   ```

   This script deploys the contracts and provides you with environment variables to set for future interactions.

## Setting Environment Variables

After running `deploy.sh`, set the provided environment variables:

```bash
export NODE=tcp://localhost:26657
export NAME=validator
export FS_CONTRACT_ADDRESS=<FS_CONTRACT_ADDRESS>
export TS_CONTRACT_ADDRESS=<TS_CONTRACT_ADDRESS>
export AH_CONTRACT_ADDRESS=<AH_CONTRACT_ADDRESS>
```

Replace `<FS_CONTRACT_ADDRESS>`, `<TS_CONTRACT_ADDRESS>`, and `<AH_CONTRACT_ADDRESS>` with the actual contract addresses provided by the deployment script.

## Additional Configuration

- When deploying contracts subsequently, running `./scripts/deploy.sh` will migrate instead of initializing the contracts.
- Add any new queries and messages introduced to `scripts/queries.sh` and `scripts/messages.sh` to document their specifications and test them. These scripts use the environment variables set earlier to determine the network and contract addresses. If the variables are not set, it defaults to TestNet and its contract addresses.

Example query command:

```bash
$ ./scripts/queries.sh elys1u8c28343vvhwgwhf29w6hlcz73hvq7lwxmrl46 liquid_assets
```

## Questions & Contributions

For questions or contributions, please open an issue or a pull request in the repository. Your feedback and contributions are welcome!
