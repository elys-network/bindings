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

Spin Up the Localnet
To spin up your localnet, use the provided snapshot URL:
go run ./scripts/upgrade-assure/... <SNAPSHOT_URL> ~/go/bin/elysd ~/go/bin/elysd --skip-proposal
Make sure to replace <SNAPSHOT_URL> with the actual URL of the downloaded snapshot, for example: https://snapshots.polkachu.com/testnet-snapshots/elys/elys_5789265.tar.lz4.

Deploying Contracts
1. Build Contracts
From the bindings repository, build all three contract binaries:
./scripts/build.sh
2. Deploy Contracts
Deploy the contracts to your localnet:
./scripts/deploy.sh
This script deploys the contracts and provides environment variables for future interactions.

Setting Environment Variables
After running deploy.sh, set the provided environment variables:
export NODE=tcp://localhost:26657
export NAME=validator
export FS_CONTRACT_ADDRESS=<FS_CONTRACT_ADDRESS>
export TS_CONTRACT_ADDRESS=<TS_CONTRACT_ADDRESS>
export AH_CONTRACT_ADDRESS=<AH_CONTRACT_ADDRESS>
Replace <FS_CONTRACT_ADDRESS>, <TS_CONTRACT_ADDRESS>, and <AH_CONTRACT_ADDRESS> with the actual contract addresses provided by the deployment script.

Additional Configuration
When deploying contracts subsequently, running ./scripts/deploy.sh will migrate instead of initializing the contracts.
Document any new queries and messages introduced in scripts/queries.sh and scripts/messages.sh to specify and test them. These scripts use the environment variables set earlier to determine the network and contract addresses. If the variables are not set, it defaults to TestNet and its contract addresses.
Example query command:
$ ./scripts/queries.sh elys1u8c28343vvhwgwhf29w6hlcz73hvq7lwxmrl46 liquid_assets

Questions & Contributions
For questions or contributions, please open an issue or a pull request in the repository. Your feedback and contributions are welcome!
