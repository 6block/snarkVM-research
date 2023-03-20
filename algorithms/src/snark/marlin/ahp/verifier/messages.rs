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

use snarkvm_fields::PrimeField;

use crate::snark::marlin::{CircuitId, witness_label, MarlinMode};
use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct BatchCombiners<F> {
    pub circuit_combiner: F,
    pub instance_combiners: Vec<F>,
}

/// First message of the verifier.
#[derive(Clone, Debug)]
pub struct FirstMessage<'a, F: PrimeField> {
    /// Query for the random polynomial.
    pub alpha: F,
    /// Randomizer for the lincheck for `B`.
    pub eta_b: F,
    /// Randomizer for the lincheck for `C`.
    pub eta_c: F,
    /// Randomizers for combining vectors from the batch
    pub batch_combiners: BTreeMap<&'a CircuitId, BatchCombiners<F>>,
}

/// Second verifier message.
#[derive(Copy, Clone, Debug)]
pub struct SecondMessage<F> {
    /// Query for the second round of polynomials.
    pub beta: F,
}

/// Third message of the verifier.
#[derive(Clone, Debug)]
pub struct ThirdMessage<F> {
    /// Randomizers for the h-polynomial for `A_i`, `B_i`, `C_i` for circuit i.
    pub r_a: Vec<F>,
    pub r_b: Vec<F>,
    pub r_c: Vec<F>,
}

/// Query set of the verifier.
#[derive(Clone, Debug)]
pub struct QuerySet<'a, F: PrimeField> {
    pub batch_sizes: BTreeMap<&'a CircuitId, usize>,
    pub g_1_query: (String, F),
    pub z_b_query: (String, F),
    pub lincheck_sumcheck_query: (String, F),

    pub g_a_query: (String, F),
    pub g_b_query: (String, F),
    pub g_c_query: (String, F),
    pub matrix_sumcheck_query: (String, F),
}

impl<'a, F: PrimeField> QuerySet<'a, F> {
    pub fn new<MM: MarlinMode>(state: &super::State<'a, F, MM>) -> Self {
        let beta = state.second_round_message.unwrap().beta;
        let gamma = state.gamma.unwrap();
        // For the first linear combination
        // Lincheck sumcheck test:
        //   s(beta) + r(alpha, beta) * (sum_M eta_M z_M(beta)) - t(beta) * z(beta)
        // = h_1(beta) * v_H(beta) + beta * g_1(beta)
        //
        // Note that z is the interpolation of x || w, so it equals x + v_X * w
        // We also use an optimization: instead of explicitly calculating z_c, we
        // use the "virtual oracle" z_a * z_b
        Self {
            batch_sizes: state.circuit_specific_states.iter().map(|(c,s)|(*c, s.batch_size)).collect::<BTreeMap<_,_>>(),
            g_1_query: ("beta".into(), beta),
            z_b_query: ("beta".into(), beta),
            lincheck_sumcheck_query: ("beta".into(), beta),

            g_a_query: ("gamma".into(), gamma),
            g_b_query: ("gamma".into(), gamma),
            g_c_query: ("gamma".into(), gamma),
            matrix_sumcheck_query: ("gamma".into(), gamma),
        }
    }

    /// Returns a `BTreeSet` containing elements of the form
    /// `(polynomial_label, (query_label, query))`.
    pub fn to_set(&self) -> crate::polycommit::sonic_pc::QuerySet<'_, F> {
        let mut query_set = crate::polycommit::sonic_pc::QuerySet::new();
        for (circuit_id, &batch_size) in self.batch_sizes.iter() {
            for i in 0..batch_size {
                query_set.insert((witness_label(&circuit_id, "z_b", i), self.z_b_query.clone()));
                query_set.insert((witness_label(&circuit_id, "g_a", i), self.g_a_query.clone()));
                query_set.insert((witness_label(&circuit_id, "g_b", i), self.g_b_query.clone()));
                query_set.insert((witness_label(&circuit_id, "g_c", i), self.g_c_query.clone()));
            }
        }
        query_set.insert(("g_1".into(), self.g_1_query.clone()));
        query_set.insert(("lincheck_sumcheck".into(), self.lincheck_sumcheck_query.clone()));
        query_set.insert(("matrix_sumcheck".into(), self.matrix_sumcheck_query.clone()));
        query_set
    }
}
