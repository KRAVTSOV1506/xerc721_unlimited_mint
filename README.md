# xerc721_unlimited_mint

1. Compile wasm
```
cd contracts/xerc721
RUSTFLAGS='-C link-arg=-s' cargo wasm
```
2. Deploy on station https://station.testnet.routerchain.dev/

Initiate message:
```
{
  "name": "NftName",
  "symbol": "Symbol",
  "minter": "$owner_address"
}
```

3. Set remoute contract
```
{
  "extension": {
    "msg": {
      "enroll_remote_contract": {
        "chain_id": "80001",
        "remote_address": "0xc27CE28850774288B3EF678c4550161346944152"
      }
    }
  }
}
```
