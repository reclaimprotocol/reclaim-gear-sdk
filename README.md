
# Reclaim - Gear

## Deployments

| Chain Name | Deployed address | Explorer Link |
|:-----------|:-----------------|:--------------|
| Vara Testnet | 0x383951634434f94d4bf23ab1d7825bff90fe6a00ec2c3f389e7b8d771b053af8 | https://idea.gear-tech.io/programs/0x383951634434f94d4bf23ab1d7825bff90fe6a00ec2c3f389e7b8d771b053af8?node=wss%3A%2F%2Ftestnet.vara.network |



## Environment

```sh
cargo b --workspace
```

### Testing

Run all tests, except `gclient` ones:
```sh
cargo t --workspace -- --skip gclient
```

Run all tests:
```sh
# Download the node binary.
cargo xtask node
cargo t --workspace
```

### Client 

Populate your `.env` and run:
```sh
cd js
npm install
npm run start
```


