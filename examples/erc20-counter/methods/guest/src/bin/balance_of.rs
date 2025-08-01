// Copyright 2025 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![allow(unused_doc_comments)]

use alloy_primitives::{Address, U256};
use alloy_sol_types::{sol, SolValue};
use risc0_steel::{
    ethereum::{EthEvmInput, ETH_SEPOLIA_CHAIN_SPEC},
    Commitment, Contract,
};
use risc0_zkvm::guest::env;

/// Specify the function to call using the [`sol!`] macro.
/// This parses the Solidity syntax to generate a struct that implements the `SolCall` trait.
sol! {
    /// ERC-20 balance function signature.
    interface IERC20 {
        function balanceOf(address account) external view returns (uint);
    }
}

/// ABI encodable journal data.
sol! {
    struct Journal {
        Commitment commitment;
        address tokenAddress;
    }
}

fn main() {
    // Read the input from the guest environment.
    let input: EthEvmInput = env::read();
    let contract: Address = env::read();
    let account: Address = env::read();

    // Converts the input into a `EvmEnv` for execution. It checks that the state matches the state
    // root in the header provided in the input.
    let env = input.into_env(&ETH_SEPOLIA_CHAIN_SPEC);

    // Execute the view call; it returns the result in the type generated by the `sol!` macro.
    let call = IERC20::balanceOfCall { account };
    let returns = Contract::new(contract, &env).call_builder(&call).call();

    // Check that the given account holds at least 1 token.
    assert!(returns >= U256::from(1));

    // Commit the block hash and number used when deriving `view_call_env` to the journal.
    let journal = Journal {
        commitment: env.into_commitment(),
        tokenAddress: contract,
    };
    env::commit_slice(&journal.abi_encode());
}
