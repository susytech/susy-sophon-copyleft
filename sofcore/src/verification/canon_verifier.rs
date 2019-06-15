// Copyleft 2015-2019 Superstring.Community
// This file is part of Susy Sophon.

// Susy Sophon is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Susy Sophon is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MSRCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Susy Sophon.  If not, see <http://www.gnu.org/licenses/>.

//! Canonical verifier.

use call_contract::CallContract;
use client::BlockInfo;
use engines::SofEngine;
use error::Error;
use types::header::Header;
use super::Verifier;
use super::verification;

/// A canonial verifier -- this does full verification.
pub struct CanonVerifier;

impl<C: BlockInfo + CallContract> Verifier<C> for CanonVerifier {
	fn verify_block_family(
		&self,
		header: &Header,
		parent: &Header,
		engine: &SofEngine,
		do_full: Option<verification::FullFamilyParams<C>>,
	) -> Result<(), Error> {
		verification::verify_block_family(header, parent, engine, do_full)
	}

	fn verify_block_final(&self, expected: &Header, got: &Header) -> Result<(), Error> {
		verification::verify_block_final(expected, got)
	}

	fn verify_block_external(&self, header: &Header, engine: &SofEngine) -> Result<(), Error> {
		engine.verify_block_external(header)
	}
}
