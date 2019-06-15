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

//! Actions on ancestry blocks when working on a new block.

use sophon_types::H256;

#[derive(Debug, PartialEq, Eq, Clone)]
/// Actions on a live block's parent block. Only committed when the live block is committed. Those actions here must
/// respect the normal blockchain reorganization rules.
pub enum AncestryAction {
	/// Mark an ancestry block as finalized.
	MarkFinalized(H256),
}
