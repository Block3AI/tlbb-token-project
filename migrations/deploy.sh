#!/bin/bash

# Build the smart contract
anchor build

# Deploy to Solana Devnet
anchor deploy --provider.cluster devnet