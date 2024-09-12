<div>
    <div>
        <img src="https://raw.githubusercontent.com/reclaimprotocol/.github/main/assets/banners/Gear-SDK.png"  />
    </div>
</div>

## Deployments

| Chain Name | Deployed Address | Explorer Link |
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

## Contributing to Our Project

We're excited that you're interested in contributing to our project! Before you get started, please take a moment to review the following guidelines.

## Code of Conduct

Please read and follow our [Code of Conduct](https://github.com/reclaimprotocol/.github/blob/main/Code-of-Conduct.md) to ensure a positive and inclusive environment for all contributors.

## Security

If you discover any security-related issues, please refer to our [Security Policy](https://github.com/reclaimprotocol/.github/blob/main/SECURITY.md) for information on how to responsibly disclose vulnerabilities.

## Contributor License Agreement

Before contributing to this project, please read and sign our [Contributor License Agreement (CLA)](https://github.com/reclaimprotocol/.github/blob/main/CLA.md).

## Indie Hackers

For Indie Hackers: [Check out our guidelines and potential grant opportunities](https://github.com/reclaimprotocol/.github/blob/main/Indie-Hackers.md)

## License

This project is licensed under a [custom license](https://github.com/reclaimprotocol/.github/blob/main/LICENSE). By contributing to this project, you agree that your contributions will be licensed under its terms.

Thank you for your contributions!

