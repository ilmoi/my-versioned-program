import * as anchor from "@project-serum/anchor";
import {Program} from "@project-serum/anchor";
import {MyVersProg} from "../target/types/my_vers_prog";
import {
  AddressLookupTableAccount,
  AddressLookupTableProgram,
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  SystemProgram,
  Transaction,
  TransactionInstruction,
  TransactionMessage,
  VersionedTransaction
} from '@solana/web3.js'

const fs = require('fs');

async function waitMS(ms: number) {
  await new Promise((response) =>
    setTimeout(() => {
      response(0);
    }, ms)
  );
}

const sendV0Tx = async (conn: Connection, ixs: TransactionInstruction[], payer: Keypair, lookupTableAccounts?: [AddressLookupTableAccount] | undefined) => {
  const bh = (await conn.getLatestBlockhash()).blockhash;
  const msg = new TransactionMessage({
    payerKey: payer.publicKey,
    recentBlockhash: bh,
    instructions: ixs
  }).compileToV0Message(lookupTableAccounts)
  const tx = new VersionedTransaction(msg);
  tx.sign([payer])
  const sig = await conn.sendTransaction(tx);
  await conn.confirmTransaction(sig)
  console.log('✅ tx successful', sig)
  return sig;
}

const storeSig = async (conn: Connection, sig: string, name: string) => {
  let fetchedTx;
  while (!fetchedTx) {
    fetchedTx = await conn.getParsedTransaction(sig, {
      maxSupportedTransactionVersion: 0,
      commitment: 'confirmed'
    })
    await waitMS(1000);
  }
  // console.log('fetched tx', JSON.stringify(fetchedTx, null, 4))
  fs.writeFileSync(`${name}.json`, JSON.stringify(fetchedTx))
}

describe("my-vers-prog", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);

  const conn = provider.connection;
  const payer = new Keypair();
  const program = anchor.workspace.MyVersProg as Program<MyVersProg>;

  //dumny accounts
  const accs = [];
  for (let i = 0; i < 40; i++) {
    accs.push(Keypair.generate().publicKey)
  }
  const accsObj = {};
  accs.forEach((acc, i) => {
    accsObj[`play${i}`] = acc;
  })

  it("Is initialized!", async () => {
    console.log('// --------------------------------------- fund payer (wallet cant manually sign0')

    const fundIxs: TransactionInstruction[] = [
      SystemProgram.transfer({
        fromPubkey: provider.wallet.publicKey,
        toPubkey: payer.publicKey,
        lamports: LAMPORTS_PER_SOL,
      }),
    ];
    const tx = new Transaction({
      recentBlockhash: (await conn.getLatestBlockhash()).blockhash,
      feePayer: provider.wallet.publicKey
    })
    tx.add(...fundIxs)
    await provider.wallet.signTransaction(tx)
    const sig = await conn.sendRawTransaction(tx.serialize())
    await conn.confirmTransaction(sig)
    console.log('✅ funded', (await conn.getAccountInfo(payer.publicKey)).lamports)

    console.log('// --------------------------------------- create & extend lut')

    const slot = await conn.getSlot("confirmed");

    //create
    const [lookupTableInst, lookupTableAddress] =
      AddressLookupTableProgram.createLookupTable({
        authority: payer.publicKey,
        payer: payer.publicKey,
        recentSlot: slot,
      });
    console.log("lookup table address:", lookupTableAddress.toBase58());

    //add addresses
    const extendInstruction1 = AddressLookupTableProgram.extendLookupTable({
      payer: payer.publicKey,
      authority: payer.publicKey,
      lookupTable: lookupTableAddress,
      addresses: [
        payer.publicKey,
        SystemProgram.programId,
        ...accs.slice(0, 20)
      ],
    });
    await sendV0Tx(conn, [lookupTableInst, extendInstruction1], payer);

    const extendInstruction2 = AddressLookupTableProgram.extendLookupTable({
      payer: payer.publicKey,
      authority: payer.publicKey,
      lookupTable: lookupTableAddress,
      addresses: [
        ...accs.slice(20, 40)
      ],
    });
    await sendV0Tx(conn, [extendInstruction2], payer);

    //fetch
    let lookupTableAccount: AddressLookupTableAccount;
    lookupTableAccount = (await conn.getAddressLookupTable(lookupTableAddress)).value;
    if (!lookupTableAccount) return;
    console.log('✅ created lut', lookupTableAccount)

    //dunno why but have to wait or errors
    await waitMS(2000)

    console.log('// --------------------------------------- fire off tx')

    // Add your test here.
    const ix = await program.methods.initialize().accounts(accsObj).instruction();
    // console.log('ix is', JSON.stringify(ix, null, 4))

    const finalSig = await sendV0Tx(conn, [ix], payer, [lookupTableAccount])
    console.log("✅ fired off a lut tx", finalSig);

    //store tx json for inspection
    await storeSig(conn, finalSig, 'lut_tx.json')
  });
});
