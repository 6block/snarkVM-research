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

use console::{
    account::{Address, PrivateKey, Signature},
    network::Network,
    types::Field,
};

use anyhow::Result;
use rand::{CryptoRng, Rng};

#[derive(Debug)]
pub enum Authority<N: Network> {
    Beacon(Signature<N>),
    Quorum,
}

impl<N: Network> Authority<N> {
    /// Initializes a new beacon authority.
    pub fn new_beacon<R: Rng + CryptoRng>(
        private_key: &PrivateKey<N>,
        block_hash: Field<N>,
        rng: &mut R,
    ) -> Result<Self> {
        // Sign the block hash.
        let signature = private_key.sign(&[block_hash], rng)?;
        // Return the beacon authority.
        Ok(Self::Beacon(signature))
    }

    /// Initializes a new quorum authority.
    pub fn new_quorum() -> Self {
        Self::Quorum
    }
}

impl<N: Network> Authority<N> {
    /// Initializes a new beacon authority from the given signature.
    pub fn from_beacon(signature: Signature<N>) -> Self {
        Self::Beacon(signature)
    }

    /// Initializes a new quorum authority.
    pub fn from_quorum() -> Self {
        Self::Quorum
    }
}

impl<N: Network> Authority<N> {
    /// Returns `true` if the authority is a beacon.
    pub fn is_beacon(&self) -> bool {
        matches!(self, Self::Beacon(_))
    }

    /// Returns `true` if the authority is a quorum.
    pub fn is_quorum(&self) -> bool {
        matches!(self, Self::Quorum)
    }
}

impl<N: Network> Authority<N> {
    /// Returns address of the authority.
    /// If the authority is a beacon, the address of the signer is returned.
    /// If the authority is a quorum, the address of the leader is returned.
    pub fn to_address(&self) -> Address<N> {
        match self {
            Self::Beacon(signature) => signature.to_address(),
            Self::Quorum => Address::zero(),
        }
    }
}
