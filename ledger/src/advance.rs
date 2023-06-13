// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the snarkVM library.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at:
// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::*;

impl<N: Network, C: ConsensusStorage<N>> Ledger<N, C> {
    /// Returns a candidate for the next block in the ledger.
    pub fn prepare_advance_to_next_block<R: Rng + CryptoRng>(
        &self,
        private_key: &PrivateKey<N>,
        candidate_transactions: Vec<Transaction<N>>,
        candidate_solutions: Option<Vec<ProverSolution<N>>>,
        rng: &mut R,
    ) -> Result<Block<N>> {
        // Retrieve the latest state root.
        let latest_state_root = *self.latest_state_root();
        // Retrieve the latest block.
        let latest_block = self.latest_block();
        // Retrieve the latest round.
        let latest_round = latest_block.round();
        // Retrieve the latest height.
        let latest_height = latest_block.height();
        // Retrieve the latest total supply in microcredits.
        let latest_total_supply = latest_block.total_supply_in_microcredits();
        // Retrieve the latest cumulative weight.
        let latest_cumulative_weight = latest_block.cumulative_weight();

        // Construct the coinbase solution.
        let (coinbase, coinbase_accumulator_point, cumulative_proof_target) = match &candidate_solutions {
            Some(solutions) => {
                // Accumulate the prover solutions into a coinbase solution.
                let coinbase = self.coinbase_puzzle.accumulate_unchecked(&self.latest_epoch_challenge()?, solutions)?;
                // Compute the accumulator point of the coinbase solution.
                let coinbase_accumulator_point = coinbase.to_accumulator_point()?;
                // Compute the cumulative proof target. Using '.sum' here is safe because we sum u64s into a u128.
                let cumulative_proof_target =
                    solutions.iter().map(|s| Ok(s.to_target()? as u128)).sum::<Result<u128>>()?;
                // Output the coinbase solution, coinbase accumulator point, and cumulative proof target.
                (Some(coinbase), coinbase_accumulator_point, cumulative_proof_target)
            }
            None => (None, Field::<N>::zero(), 0u128),
        };

        // Compute the next round number.
        let next_round = latest_round.saturating_add(1);
        // Compute the next height.
        let next_height = latest_height.saturating_add(1);
        // Compute the next cumulative weight.
        let next_cumulative_weight = latest_cumulative_weight.saturating_add(cumulative_proof_target);

        // Construct the finalize state.
        let state = FinalizeGlobalState::new(next_height);
        // Select the transactions from the memory pool.
        let transactions = self.vm.speculate(state, candidate_transactions.iter())?;

        // Compute the next total supply in microcredits.
        let next_total_supply_in_microcredits = update_total_supply(latest_total_supply, &transactions)?;

        // Checkpoint the timestamp for the next block.
        let next_timestamp = OffsetDateTime::now_utc().unix_timestamp();

        // TODO (raychu86): Pay the provers. Currently we do not pay the provers with the `credits.aleo` program
        //  and instead, will track prover leaderboards via the `coinbase_solution` in each block.
        if let Some(prover_solutions) = candidate_solutions {
            // Calculate the coinbase reward.
            let coinbase_reward = coinbase_reward(
                latest_block.last_coinbase_timestamp(),
                next_timestamp,
                next_height,
                N::STARTING_SUPPLY,
                N::ANCHOR_TIME,
            )? as u128;

            // Initialize a vector to store the prover rewards.
            let mut prover_rewards: Vec<(Address<N>, u64)> = Vec::new();

            // Calculate the rewards for the individual provers.
            for prover_solution in prover_solutions {
                // Prover compensation is defined as:
                //   1/2 * coinbase_reward * (prover_target / cumulative_prover_target)
                //   = (coinbase_reward * prover_target) / (2 * cumulative_prover_target)

                // Compute the numerator.
                let numerator = coinbase_reward
                    .checked_mul(prover_solution.to_target()? as u128)
                    .ok_or_else(|| anyhow!("Prover reward numerator overflowed"))?;
                // Compute the denominator.
                let denominator = cumulative_proof_target
                    .checked_mul(2)
                    .ok_or_else(|| anyhow!("Prover reward denominator overflowed"))?;
                // Compute the quotient.
                let quotient =
                    numerator.checked_div(denominator).ok_or_else(|| anyhow!("Prover reward quotient overflowed"))?;

                // Cast the prover reward as a u64.
                let prover_reward = u64::try_from(quotient)?;
                // Ensure the prover reward is within a safe bound.
                ensure!(prover_reward <= 1_000_000_000, "Prover reward is too large");
                // Append the prover reward to the vector.
                prover_rewards.push((prover_solution.address(), prover_reward));
            }
        }

        // Construct the next coinbase target.
        let next_coinbase_target = coinbase_target(
            latest_block.last_coinbase_target(),
            latest_block.last_coinbase_timestamp(),
            next_timestamp,
            N::ANCHOR_TIME,
            N::NUM_BLOCKS_PER_EPOCH,
            N::GENESIS_COINBASE_TARGET,
        )?;

        // Construct the next proof target.
        let next_proof_target = proof_target(next_coinbase_target, N::GENESIS_PROOF_TARGET);

        // Construct the next last coinbase target and next last coinbase timestamp.
        let (next_last_coinbase_target, next_last_coinbase_timestamp) = match coinbase {
            Some(_) => (next_coinbase_target, next_timestamp),
            None => (latest_block.last_coinbase_target(), latest_block.last_coinbase_timestamp()),
        };

        // Construct the metadata.
        let metadata = Metadata::new(
            N::ID,
            next_round,
            next_height,
            next_total_supply_in_microcredits,
            next_cumulative_weight,
            next_coinbase_target,
            next_proof_target,
            next_last_coinbase_target,
            next_last_coinbase_timestamp,
            next_timestamp,
        )?;

        // Construct the header.
        let header = Header::from(
            latest_state_root,
            transactions.to_transactions_root()?,
            transactions.to_finalize_root()?,
            coinbase_accumulator_point,
            metadata,
        )?;

        // Construct the new block.
        Block::new(private_key, latest_block.hash(), header, transactions, coinbase, rng)
    }

    /// Adds the given block as the next block in the ledger.
    pub fn advance_to_next_block(&self, block: &Block<N>) -> Result<()> {
        // Acquire the write lock on the current block.
        let mut current_block = self.current_block.write();
        // Update the VM.
        self.vm.add_next_block(block)?;
        // Update the current block.
        *current_block = block.clone();
        // Drop the write lock on the current block.
        drop(current_block);

        // If the block is the start of a new epoch, or the epoch challenge has not been set, update the current epoch challenge.
        if block.height() % N::NUM_BLOCKS_PER_EPOCH == 0 || self.current_epoch_challenge.read().is_none() {
            // Update the current epoch challenge.
            self.current_epoch_challenge.write().clone_from(&self.get_epoch_challenge(block.height()).ok());
        }

        Ok(())
    }
}