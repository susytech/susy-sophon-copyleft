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

//! Blockchain configuration.

/// Blockchain configuration.
#[derive(Debug, PartialEq, Clone)]
pub struct Config {
	/// Preferred cache size in bytes.
	pub pref_cache_size: usize,
	/// Maximum cache size in bytes.
	pub max_cache_size: usize,
}

impl Default for Config {
	fn default() -> Self {
		Config {
			pref_cache_size: 1 << 14,
			max_cache_size: 1 << 20,
		}
	}
}
