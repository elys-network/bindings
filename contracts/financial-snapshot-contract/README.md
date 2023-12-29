# Financial Snapshot Contract

Financial Snapshot Contract is a state-of-the-art CosmWasm contract designed to aggregate and deliver comprehensive financial data regarding blockchain users, particularly focusing on Elys Network portfolios, assets, rewards, and liquidity positions.

## Overview

This contract offers a robust solution for obtaining aggregated insights into user financial data within Elys Network, including real-time updates on portfolio balances, asset values, and unclaimed rewards. It is developed to provide a seamless interface for users and platforms to access intricate financial data efficiently and accurately.

## Features

### 1. Dashboard for $ Value
- Calculates the $ value as the product of the Amount and the price per unit.
  
### 2. Total Balance
- Computes the sum of Portfolio and Rewards.

### 3. Portfolio
- **Portfolio Balance:** Aggregates the $ value of Liquid Assets, Staked/Committed Assets, Liquidity Positions, Leverage LP, Margin Positions, USDC Earn, and Borrows.
- **Liquid Assets:** Represents the $ value of idle assets in the user wallet on the DEX.
- **Staked/Committed Assets:** Accumulates the $ value of various staked and committed assets.
- **Liquidity Positions:** Aggregates the $ value of all open liquidity positions in liquidity pools.
- **Leverage LP & Margin Positions:** Calculates the $ value of all open leverage LP and margin positions.
- **USDC Earn:** Represents the $ value of USDC deposited in the USDC earn program.
- **Borrows:** Represents the negative $ value of the sum of all borrowed assets.
  
### 4. Rewards
- Provides $ values of unclaimed rewards in USDC, EDEN, and External Rewards.
- Represents the amount of unclaimed EDEN-Boost rewards.
  
### 5. Liquid Assets Tab
- Shows the price, 24 hr change, total amount, available tokens, and tokens in order, with $ values for oracle activated pool or Elys AMM price for fixed weight pools.
  
### 6. Liquidity Position Tab
- Displays the pool rate, APR, balance, and rewards with $ value of unclaimed rewards allocated for the specific pool.

## Specifications

### Price Calculation
- Oracle price for oracle activated pool or Elys AMM price for fixed weight pools.

### APR Calculation
- Aggregated sum of Fee APR, Inflationary EDEN rewards, and external rewards APR.

### Pool Rate
- Ratio of each asset in the specific pool. They add up to 100.

### Current TVL
- $ value of the sum of all assets inside a pool.

## Usage

This contract can be utilized by developers and platforms to integrate aggregated financial data regarding Elys users in their applications. It can serve as a reliable source for obtaining real-time updates on various financial parameters, enhancing user experience and providing valuable insights into the DeFi ecosystem.

## Development & Contribution

Developers are welcome to contribute to the enhancement and optimization of this contract. Please follow the standard pull request process, ensuring that any additions or modifications are well-documented and tested.
