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

```json
{
  "description": "My First DID",
  "id": "did:movedid:0x2df41622c0c1baabaa73b2c24360d205e23e803959ebbcb0e5b80462165893ed",
  "services": [
    {
      "description": "leeduckgo's github",
      "id": "did:movedid:0x2df41622c0c1baabaa73b2c24360d205e23e803959ebbcb0e5b80462165893ed.github",
      "url": "https://github.com/leeduckgo",
      "verification_url": "https://gist.github.com/0x"
    }
  ],
  "type": "Human",
  "verification_methods": [
    {
      "addr": "0x73c7448760517E3E6e416b2c130E3c6dB2026A1d",
      "created_at": "1673525423",
      "expired_at": "1705061423",
      "id": "did:movedid:0x2df41622c0c1baabaa73b2c24360d205e23e803959ebbcb0e5b80462165893ed#key-1",
      "internal_id": 1,
      "properties": { "chains": ["ethereum"], "description": "A Test Addr" },
      "pubkey": "",
      "type": "EcdsaSecp256k1VerificationKey2019",
      "verificated": false,
      "verification": { "msg": "50789538.1.nonce_geek", "signature": "0x" }
    }
  ]
}
```
