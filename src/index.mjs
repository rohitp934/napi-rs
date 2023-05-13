import { divide, sum } from "../index.js";

console.log("Running sum in rust: ", sum(25,25));

try {
  console.log("Running divide in rust: ", divide(125, 0));
} catch (err) {
  console.error("Divide by zero error in rust.");
}
