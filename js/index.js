import { GearApi, GearKeyring, ProgramMetadata } from '@gear-js/api';
import * as fs from "fs";
import dotenv from "dotenv";
dotenv.config();

const mnemonic = process.env.MNEMONIC;
const gearApi = await GearApi.create({
    providerAddress: 'wss://testnet.vara.network',
});

const keyring = await GearKeyring.fromMnemonic(mnemonic, 'gear-known');

const metadata = '0x' + fs.readFileSync('../target/wasm32-unknown-unknown/debug/reclaim_gear.meta.txt');
const meta = ProgramMetadata.from(metadata)

const programId = "0x0447abd6eafff6c3c6c70c4844aba14ae3510f6e4b80df55e7d57186922b160c"

async function initiate() {

    let extrinsic
    try {
        const message = {
            destination: programId,
            payload: { Init: {} },
            gasLimit: 750_000_000_000,
            value: 0,
        };

        extrinsic = gearApi.message.send(message, meta, meta.types.handle.input);
    } catch (error) {
        console.error(`${error.name}: ${error.message}`);
    }
    try {
        await extrinsic.signAndSend(keyring, (event) => {
            console.log(event.toHuman());
        });
    } catch (error) {
        console.error(`${error.name}: ${error.message}`);
    }
}

initiate().catch(console.error);

async function addEpoch() {

    const programId = "0x0447abd6eafff6c3c6c70c4844aba14ae3510f6e4b80df55e7d57186922b160c"
    let extrinsic
    try {
        const message = {
            destination: programId,
            payload: {
                AddEpoch: {
                    witness: [{ address: "0x244897572368eadf65bfbc5aec98d8e5443a9072", host: "http" }],
                    minimumWitness: 1,
                }
            },
            gasLimit: 750_000_000_000,
            value: 0,
        };

        extrinsic = gearApi.message.send(message, meta, meta.types.handle.input);
    } catch (error) {
        console.error(`${error.name}: ${error.message}`);
    }
    try {
        await extrinsic.signAndSend(keyring, (event) => {
            console.log(event.toHuman());
        });
    } catch (error) {
        console.error(`${error.name}: ${error.message}`);
    }
}

addEpoch().catch(console.error);

async function verifyProof() {

    const programId = "0x0447abd6eafff6c3c6c70c4844aba14ae3510f6e4b80df55e7d57186922b160c"
    let extrinsic

    const owner = "0xe4c20c9f558160ec08106de300326f7e9c73fb7f"

    const claimInfo = {
        "provider": "http",
        "parameters": "{\"body\":\"\",\"geoLocation\":\"in\",\"method\":\"GET\",\"responseMatches\":[{\"type\":\"contains\",\"value\":\"_steamid\\\">Steam ID: 76561199632643233</div>\"}],\"responseRedactions\":[{\"jsonPath\":\"\",\"regex\":\"_steamid\\\">Steam ID: (.*)</div>\",\"xPath\":\"id(\\\"responsive_page_template_content\\\")/div[@class=\\\"page_header_ctn\\\"]/div[@class=\\\"page_content\\\"]/div[@class=\\\"youraccount_steamid\\\"]\"}],\"url\":\"https://store.steampowered.com/account/\"}",
        "context": "{\"contextAddress\":\"user's address\",\"contextMessage\":\"for acmecorp.com on 1st january\"}",
    }

    const identifier = "0x531322a6c34e5a71296a5ee07af13f0c27b5b1e50616f816374aff6064daaf55"

    const signedClaim = {
        "claim": {
            "identifier": identifier,
            "owner": owner,
            "epoch": 1,
            "timestampS": 1710157447
        },
        "signatures": ["0x52e2a591f51351c1883559f8b6c6264b9cb5984d0b7ccc805078571242166b357994460a1bf8f9903c4130f67d358d7d6e9a52df9a38c51db6a10574b946884c1b"],
    }


    const proof = {
        claimInfo: claimInfo,
        signedClaim: signedClaim
    }


    try {
        const message = {
            destination: programId,
            payload: {
                VerifyProof: proof
            },
            gasLimit: 750_000_000_000,
        };

        extrinsic = gearApi.message.send(message, meta, meta.types.handle.input);
    } catch (error) {
        console.error(`${error.name}: ${error.message}`);
    }
    try {
        await extrinsic.signAndSend(keyring, (event) => {
            console.log(event.toHuman());
        });
    } catch (error) {
        console.error(`${error.name}: ${error.message}`);
    }
}

verifyProof().catch(console.error);
