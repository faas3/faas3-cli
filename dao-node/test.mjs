// this file is for faas3 run
import { handler } from "./main.mjs";

const res = await handler();
console.log(res);    
