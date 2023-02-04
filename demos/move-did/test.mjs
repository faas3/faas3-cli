// this file is for faas3 run
import { handler } from "./main.mjs";
const res = await handler({
  addr: "0x2df41622c0c1baabaa73b2c24360d205e23e803959ebbcb0e5b80462165893ed",
});
console.log(res);
