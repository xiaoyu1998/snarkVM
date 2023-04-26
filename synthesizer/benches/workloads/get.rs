// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

use super::*;

use std::{marker::PhantomData, str::FromStr};

pub struct StaticGet<N: Network> {
    num_mappings: usize,
    num_commands: usize,
    num_executions: usize,
    num_programs: usize,
    phantom: PhantomData<N>,
}

impl<N: Network> StaticGet<N> {
    pub fn new(num_mappings: usize, num_commands: usize, num_executions: usize, num_programs: usize) -> Self {
        Self { num_mappings, num_commands, num_executions, num_programs, phantom: Default::default() }
    }
}

impl<N: Network> Workload<N> for StaticGet<N> {
    fn setup(&self) -> Vec<Operation<N>> {
        // Initialize storage for the setup operations.
        let mut operations = Vec::with_capacity(self.num_programs);
        // Construct the operations.
        for i in 0..self.num_programs {
            // Initialize the program string.
            let mut program_string =
                format!("program get_{}_mappings_{}_commands_{i}.aleo;", self.num_mappings, self.num_commands);
            // Add the mappings.
            for j in 0..self.num_mappings {
                program_string
                    .push_str(&format!("mapping map_{j};key left as field.public;value right as field.public;"));
            }
            // Construct the init function.
            let mut init_function = "function init:finalize;finalize init:".to_string();
            // Construct the getter function.
            let mut getter_function = "function getter:finalize;finalize getter:".to_string();
            // Add the commands.
            for j in 0..self.num_mappings {
                for k in 0..self.num_commands {
                    init_function.push_str(&format!("set {k}field map_{j}[{k}field];"));
                    getter_function.push_str(&format!("get map_{j}[{k}field] into r{k};"));
                }
            }
            // Add the functions to the program string.
            program_string.push_str(&init_function);
            program_string.push_str(&getter_function);
            // Construct and add the setup operation.
            operations.push(Operation::Deploy(Box::new(Program::from_str(&program_string).unwrap())));
        }
        // Return the setup operations.
        operations
    }

    fn run(&self) -> Vec<Operation<N>> {
        // Initialize storage for the run operations.
        let mut operations = Vec::with_capacity(self.num_programs * self.num_executions);
        // Construct the operations.
        for i in 0..self.num_programs {
            for _ in 0..self.num_executions {
                operations.push(Operation::Execute(
                    format!("get_{}_mappings_{}_commands_{i}.aleo", self.num_mappings, self.num_commands),
                    "getter".to_string(),
                    vec![],
                ));
            }
        }
        // Return the run operations.
        operations
    }
}