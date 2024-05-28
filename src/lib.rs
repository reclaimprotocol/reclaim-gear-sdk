#![no_std]
#![allow(non_snake_case)]

use gstd::{collections::HashMap, msg, prelude::*, ActorId};

use claims::*;
use error::*;
use state::*;

mod claims;
mod error;
mod state;

#[derive(Debug, Clone, Default)]
pub struct Reclaim {
    pub owner: ActorId,
    pub current_epoch: u64,
    pub epochs: HashMap<u64, Epoch>,
}

static mut RECLAIM: Option<Reclaim> = None;

impl Reclaim {
    fn init(&mut self) {
        let owner = msg::source();
        let current_epoch = 0_u64;
        let epochs = HashMap::<u64, Epoch>::new();

        self.owner = owner;
        self.current_epoch = current_epoch;
        self.epochs = epochs;
    }

    fn add_epoch(
        &mut self,
        witness: Vec<Witness>,
        minimum_witness: u128,
    ) -> Result<(), ContractError> {
        let source = msg::source();
        // Check if sender is owner
        if source != self.owner {
            return gstd::Err(ContractError::Unauthorized {});
        } else {
            //Increment Epoch number
            let new_epoch = self.current_epoch + 1_u64;

            // Create the new epoch
            let epoch = Epoch {
                id: new_epoch,
                witness,
                timestamp_start: 0_u64,
                timestamp_end: 86400_u64,
                minimum_witness_for_claim_creation: minimum_witness,
            };

            self.epochs.insert(new_epoch, epoch);

            // Save the new epoch
            self.current_epoch = new_epoch;

            gstd::Result::Ok(())
        }
    }

    fn verify_proof(&mut self, proof: Proof) -> Result<(), ContractError> {
        // let fetched_epoch = EPOCHS.get(deps.storage, &msg.proof.signedClaim.claim.epoch.into());
        let fetched_epoch = self.epochs.get(&proof.signedClaim.claim.epoch);

        match fetched_epoch {
            Some(epoch) => {
                // Hash the claims, and verify with identifier hash
                let hashed = proof.claimInfo.hash();
                assert!(proof.signedClaim.claim.identifier == hashed);
                // Fetch witness for claim
                let expected_witness = fetch_witness_for_claim(
                    epoch.clone(),
                    proof.signedClaim.claim.identifier.clone(),
                    0_u64,
                );

                let expected_witness_addresses = Witness::get_addresses(expected_witness);

                let signed_witness = proof.signedClaim.recover_signers_of_signed_claim();

                assert!(expected_witness_addresses.len() == signed_witness.len());

                // Ensure for every signature in the sign, a expected witness exists from the database
                for signed in signed_witness {
                    if !expected_witness_addresses.contains(&signed) {
                        return gstd::Err(ContractError::SignatureErr {});
                    };
                }
            }

            None => return gstd::Err(ContractError::NotFoundErr {}),
        }
        gstd::Result::Ok(())
    }
}

#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct ProofMsg {
    pub proof: Proof,
}

#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum ReAction {
    VerifyProof(Proof),
    AddEpoch {
        witness: Vec<Witness>,
        minimum_witness: u128,
    },
    Init {},
}

#[no_mangle]
extern fn handle() {
    let action: ReAction = msg::load().expect("Could not load Action");
    let re: &mut Reclaim = unsafe { RECLAIM.get_or_insert(Default::default()) };

    match action {
        ReAction::VerifyProof(proof) => re.verify_proof(proof).unwrap(),
        ReAction::AddEpoch {
            witness,
            minimum_witness,
        } => re.add_epoch(witness, minimum_witness).unwrap(),
        ReAction::Init {} => re.init(),
    }
}
