// Client
console.log("My address:", pg.wallet.publicKey.toString());

const balance = await pg.connection.getBalance(pg.wallet.publicKey);
console.log(`My balance: ${balance / web3.LAMPORTS_PER_SOL} SOL`);

// PDA de la tienda
const [tiendaPda] = web3.PublicKey.findProgramAddressSync(
  [
    Buffer.from("tienda"),
    pg.wallet.publicKey.toBuffer(),
  ],
  pg.program.programId
);

console.log("PDA tienda:", tiendaPda.toString());


// CREAR TIENDA
await pg.program.methods
  .crearTienda("Tienda Belleza")
  .accounts({
    owner: pg.wallet.publicKey,
    tienda: tiendaPda,
    systemProgram: web3.SystemProgram.programId,
  })
  .rpc();

console.log("Tienda creada!");


// AGREGAR COSMETICO
await pg.program.methods
  .agregarProducto("Labial Rojo", 150)
  .accounts({
    owner: pg.wallet.publicKey,
    tienda: tiendaPda,
  })
  .rpc();

console.log("Cosmético agregado!");


// VER PRODUCTOS
await pg.program.methods
  .verProductos()
  .accounts({
    owner: pg.wallet.publicKey,
    tienda: tiendaPda,
  })
  .rpc();


// TOTAL PRODUCTOS
await pg.program.methods
  .totalProductos()
  .accounts({
    owner: pg.wallet.publicKey,
    tienda: tiendaPda,
  })
  .rpc();
