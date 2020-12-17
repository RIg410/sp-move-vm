#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate alloc;

pub mod dvm;
pub mod gas_schedule;
pub mod types;

use crate::types::{Gas, ModuleTx, ScriptTx, VmResult};

pub trait Vm {
    fn publish_module(&self, gas: Gas, module: ModuleTx) -> VmResult;
    fn execute_script(&self, gas: Gas, tx: ScriptTx) -> VmResult;
    fn clear(&mut self);
}