use gstd::prelude::*;

#[derive(Debug, Clone, Default, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Witness {
    pub address: String,
    pub host: String,
}

impl Witness {
    pub fn get_addresses(witness: Vec<Witness>) -> Vec<String> {
        let mut vec_addresses = vec![];
        for wit in witness {
            vec_addresses.push(format!("0x{}",wit.address));
        }
        vec_addresses
    }
}

#[derive(Debug, Clone, Default)]
pub struct Epoch {
    pub id: u64,
    pub timestamp_start: u64,
    pub timestamp_end: u64,
    pub minimum_witness_for_claim_creation: u128,
    pub witness: Vec<Witness>,
}
