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

//! Virtual machines support library

extern crate sophon_types;
extern crate susy_bytes as bytes;
extern crate sofjson;
extern crate srlp;
extern crate keccak_hash as hash;
extern crate susy_patricia_trie_sophon as softrie;
extern crate trie_db as trie;

mod action_params;
mod call_type;
mod env_info;
mod schedule;
mod ext;
mod return_data;
mod error;

pub mod tests;

pub use action_params::{ActionParams, ActionValue, ParamsType};
pub use call_type::CallType;
pub use env_info::{EnvInfo, LastHashes};
pub use schedule::{Schedule, CleanDustMode, WasmCosts};
pub use ext::{Ext, MessageCallResult, ContractCreateResult, CreateContractAddress};
pub use return_data::{ReturnData, GasLeft};
pub use error::{Error, Result, TrapResult, TrapError, TrapKind, ExecTrapResult, ExecTrapError};

/// Virtual Machine interface
pub trait Exec: Send {
	/// This function should be used to execute transaction.
	/// It returns either an error, a known amount of gas left, or parameters to be used
	/// to compute the final gas left.
	fn exec(self: Box<Self>, ext: &mut Ext) -> ExecTrapResult<GasLeft>;
}

/// Resume call interface
pub trait ResumeCall: Send {
	/// Resume an execution for call, returns back the Vm interface.
	fn resume_call(self: Box<Self>, result: MessageCallResult) -> Box<Exec>;
}

/// Resume create interface
pub trait ResumeCreate: Send {
	/// Resume an execution from create, returns back the Vm interface.
	fn resume_create(self: Box<Self>, result: ContractCreateResult) -> Box<Exec>;
}