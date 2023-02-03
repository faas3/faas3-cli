# move-did demo

```bash
$ cargo run -- call move-did --body '{"addr" : "0x2df41622c0c1baabaa73b2c24360d205e23e803959ebbcb0e5b80462165893ed"}'

✅ Your resp is:
 Object {
    "name": Object {
        "id": String("did:movedid:0x2df41622c0c1baabaa73b2c24360d205e23e803959ebbcb0e5b80462165893ed"),
        "type": String("Human"),
        "description": String("My First DID"),
        "verification_methods": Array [
            Object {
                "id": String("did:movedid:0x2df41622c0c1baabaa73b2c24360d205e23e803959ebbcb0e5b80462165893ed-0x73c7448760517E3E6e416b2c130E3c6dB2026A1d}"),
                "internal_id": String("1"),
                "properties": Object {
                    "description": String("A Test Addr"),
                    "chains": Array [
                        String("ethereum"),
                    ],
                },
                "type": String("EcdsaSecp256k1VerificationKey2019"),
                "addr": String("0x73c7448760517E3E6e416b2c130E3c6dB2026A1d"),
                "pubkey": String(""),
                "verificated": Bool(false),
                "verification": Object {
                    "msg": String("50789538.1.nonce_geek"),
                    "signature": String("0x"),
                },
                "created_at": String("1673525423"),
                "expired_at": String("1705061423"),
            },
        ],
        "services": Array [],
    },
}

```
