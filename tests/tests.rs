use gclient::{GearApi, Result};
use gstd::prelude::*;
use gtest::{Program, System};
use reclaim_io::*;
use std::fs;

#[test]
fn proof_full_cycle() {
    let system = System::new();

    system.init_logger();

    let program = Program::current(&system);

    let mut result = program.send(0, ReAction::Init {});

    assert!(!result.main_failed());

    let mut witness = Vec::new();
    let wit = Witness {
        address: "0x244897572368eadf65bfbc5aec98d8e5443a9072".to_string(),
        host: "http".to_string(),
    };
    witness.push(wit);

    result = program.send(
        0,
        ReAction::AddEpoch {
            witness,
            minimum_witness: 1,
        },
    );
    assert!(!result.main_failed());

    let claim_info = ClaimInfo {
        provider: "http".to_string(),
         parameters:"{\"body\":\"\",\"geoLocation\":\"in\",\"method\":\"GET\",\"responseMatches\":[{\"type\":\"contains\",\"value\":\"_steamid\\\">Steam ID: 76561199632643233</div>\"}],\"responseRedactions\":[{\"jsonPath\":\"\",\"regex\":\"_steamid\\\">Steam ID: (.*)</div>\",\"xPath\":\"id(\\\"responsive_page_template_content\\\")/div[@class=\\\"page_header_ctn\\\"]/div[@class=\\\"page_content\\\"]/div[@class=\\\"youraccount_steamid\\\"]\"}],\"url\":\"https://store.steampowered.com/account/\"}".to_string(),
          context: "{\"contextAddress\":\"user's address\",\"contextMessage\":\"for acmecorp.com on 1st january\"}".to_string()
        };

    let complete_claim_data = CompleteClaimData {
        identifier: "0x531322a6c34e5a71296a5ee07af13f0c27b5b1e50616f816374aff6064daaf55"
            .to_string(),
        owner: "0xe4c20c9f558160ec08106de300326f7e9c73fb7f".to_string(),
        epoch: 1,
        timestampS: 1710157447,
    };

    let sig = "0x52e2a591f51351c1883559f8b6c6264b9cb5984d0b7ccc805078571242166b357994460a1bf8f9903c4130f67d358d7d6e9a52df9a38c51db6a10574b946884c1b".to_string();
    let mut sigs = Vec::new();
    sigs.push(sig);

    let signed_claim = SignedClaim {
        claim: complete_claim_data,
        signatures: sigs,
    };

    let proof = Proof {
        claimInfo: claim_info,
        signedClaim: signed_claim,
    };

    result = program.send(0, ReAction::VerifyProof(proof));

    assert!(!result.main_failed());
    // result = program.send(2, PingPong::Pong);

    // assert!(!result.main_failed());

    // Querying the state using the `pingers` metafunction
}

#[tokio::test]
async fn gclient_test() -> Result<()> {
    let _wasm_binary =
        fs::read("target/wasm32-unknown-unknown/debug/reclaim_gear.opt.wasm").unwrap();
    let client = GearApi::dev_from_path("target/tmp/gear").await?;
    let _listener = client.subscribe().await?;

    // let mut gas_limit = client
    //     .calculate_upload_gas(None, wasm_binary.clone(), vec![], 0, true)
    //     .await?
    //     .min_limit;
    // let (mut message_id, program_id, _) = client
    //     .upload_program_bytes(
    //         wasm_binary,
    //         gclient::now_micros().to_le_bytes(),
    //         [],
    //         gas_limit,
    //         0,
    //     )
    //     .await?;

    // assert!(listener.message_processed(message_id).await?.succeed());

    // gas_limit = client
    //     .calculate_handle_gas(None, program_id, PingPong::Ping.encode(), 0, true)
    //     .await?
    //     .min_limit;
    // (message_id, _) = client
    //     .send_message(program_id, PingPong::Ping, gas_limit, 0)
    //     .await?;

    // let (_, raw_reply, _) = listener.reply_bytes_on(message_id).await?;

    Ok(())
}
