# move-did demo

## use curl

```bash
$ curl --location --request POST 'https://faas3.up.railway.app/api/runner/move-did' \
--header 'Content-Type: application/json' \
--data-raw '{
    "addr" : "0x2df41622c0c1baabaa73b2c24360d205e23e803959ebbcb0e5b80462165893ed"
}'

{
    "name": {
        "id": "did:movedid:0x2df41622c0c1baabaa73b2c24360d205e23e803959ebbcb0e5b80462165893ed",
        "type": "Human",
        "description": "My First DID",
        "verification_methods": [
            {
                "id": "did:movedid:0x2df41622c0c1baabaa73b2c24360d205e23e803959ebbcb0e5b80462165893ed-0x73c7448760517E3E6e416b2c130E3c6dB2026A1d}",
                "internal_id": "1",
                "properties": {
                    "description": "A Test Addr",
                    "chains": [
                        "ethereum"
                    ]
                },
                "type": "EcdsaSecp256k1VerificationKey2019",
                "addr": "0x73c7448760517E3E6e416b2c130E3c6dB2026A1d",
                "pubkey": "",
                "verificated": false,
                "verification": {
                    "msg": "50789538.1.nonce_geek",
                    "signature": "0x"
                },
                "created_at": "1673525423",
                "expired_at": "1705061423"
            }
        ],
        "services": {
            "id": "did:movedid:0x2df41622c0c1baabaa73b2c24360d205e23e803959ebbcb0e5b80462165893ed-github}",
            "description": "leeduckgo's github",
            "verification_url": "https://gist.github.com/0x",
            "url": "https://github.com/leeduckgo"
        }
    }
}
```

## user faas-cli

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
