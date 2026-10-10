import * as web3 from "@solana/web3.js";
// Manually initialize variables that are automatically defined in Playground
const PROGRAM_ID = new web3.PublicKey("38hpUbWTZq8887gD7gZ5ds28vYJpu7BQLrRxAJ1qv4tX");
const connection = new web3.Connection("https://api.devnet.solana.com", "confirmed");
const wallet = { keypair: web3.Keypair.generate() };

const tag = Buffer.from([1]);
const text = Buffer.from("Привет из Devnet!", "utf8");

const hello = new web3.TransactionInstruction({
  keys: [],
  programId: new web3.PublicKey(PROGRAM_ID),
  data: Buffer.from([0]),
});
const echo = new web3.TransactionInstruction({
  keys: [],
  programId: new web3.PublicKey(PROGRAM_ID),
  data: Buffer.concat([tag, text]),
});

const tx = new web3.Transaction().add(hello, echo);
const sig = await web3.sendAndConfirmTransaction(connection, tx, [wallet.keypair]);
console.log("Explorer:", `https://explorer.solana.com/tx/${sig}?cluster=devnet`);