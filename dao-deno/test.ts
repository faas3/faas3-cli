// this file is for faas3 run
import { handler } from "./main.ts";

const res = await handler();
console.log(res);    
