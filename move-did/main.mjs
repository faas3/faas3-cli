import * as aptos from "aptos";

export async function handler(payload) {
  const NODE_URL = "https://fullnode.testnet.aptoslabs.com";
  const client = new aptos.AptosClient(NODE_URL);

  console.log("Your payload is ");
  console.log(payload);
  const dogAddr = payload.addr;
  const dog = new aptos.HexString(dogAddr);
  const AddrAggregator = await client.getAccountResource(
    dog,
    "0x65f4a0954aa6e68d2381ff98b7676df2fe57beee3ca37a4a8a57fa621c1db872::addr_aggregator::AddrAggregator"
  );

  const {
    key_addr: keyAddr,
    type: rawType,
    description,
    addrs,
    addr_infos_map: { handle },
  } = AddrAggregator.data;

  const key = addrs[0];
  const syntax =
    "did:movedid:0x2df41622c0c1baabaa73b2c24360d205e23e803959ebbcb0e5b80462165893ed";

  const methods = await genVerificationMethods(client, handle, syntax, key);

  const services = await genServices(client, keyAddr, syntax);

  const result = {
    id: syntax,
    type: genType(rawType),
    description,
    verification_methods: methods,
    services,
  };

  return result;
}

async function genVerificationMethods(client, handle, syntax, key) {
  const item = await client.getTableItem(handle, {
    key_type: "0x1::string::String",
    value_type:
      "0x65f4a0954aa6e68d2381ff98b7676df2fe57beee3ca37a4a8a57fa621c1db872::addr_info::AddrInfo",
    key,
  });
  return [
    {
      id: `${syntax}-${key}}`,
      internal_id: item.id,
      properties: {
        description: item.description,
        chains: item.chains,
      },
      type: addrType(item.addr_type),
      addr: item.addr,
      pubkey: item.pubkey,
      verificated: verify(item.signature),
      verification: {
        msg: item.msg,
        signature: item.signature,
      },
      created_at: item.created_at,
      expired_at: item.expired_at,
    },
  ];
}

async function genServices(client, keyAddr, syntax) {
  const ServiceAggregator = await client.getAccountResource(
    keyAddr,
    "0x65f4a0954aa6e68d2381ff98b7676df2fe57beee3ca37a4a8a57fa621c1db872::service_aggregator::ServiceAggregator"
  );
  const {
    names: keys,
    services_map: { handle },
  } = ServiceAggregator.data;

  const item = await client.getTableItem(handle, {
    key_type: "0x1::string::String",
    value_type:
      "0x65f4a0954aa6e68d2381ff98b7676df2fe57beee3ca37a4a8a57fa621c1db872::service_aggregator::Service",
    key: keys[0],
  });

  return {
    id: `${syntax}-${keys[0]}}`,
    description: item.description,
    verification_url: item.verification_url,
    url: item.url,
  };
}

function addrType(type) {
  switch (type) {
    case "0":
      return "EcdsaSecp256k1VerificationKey2019";
    case "1":
      return "Ed25519VerificationKey2020";
    default:
      return "other";
  }
}

function verify(signature) {
  if (signature == "0x") {
    return false;
  }
  return true;
}
function genType(rawType) {
  switch (rawType) {
    case "0":
      return "Human";
    case "1":
      return "DAO";
    case "2":
      return "Bot";
    default:
      return "other";
  }
}
