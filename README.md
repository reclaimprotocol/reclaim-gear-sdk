
# Reclaim in Gear

### Building

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


