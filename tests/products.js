const assert = require("assert");
const anchor = require('@project-serum/anchor');

describe('products', () => {

  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Products;
  
  const owner1 = anchor.web3.Keypair.generate();
  const delegateTo = anchor.web3.Keypair.generate();


  it("Is runs the constructor", async () => {
    await provider.connection.confirmTransaction(
        await provider.connection.requestAirdrop(provider.wallet.publicKey, 10000000000),
           "confirmed"
   );
    await program.state.rpc.new({});

    // Fetch the products struct from the network.
    const state = await program.state.fetch();

    assert.equal(state.products.length, 10);

    await program.state.rpc.init({});

    const state2 = await program.state.fetch();

    assert.equal(state2.products.length, 0);

   });

  

  
  it("Executes createProduct method on the program", async () => {
    
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(owner1.publicKey, 10000000000),
             "confirmed"
     );
    await program.state.rpc.createProduct(
      "test1",
      owner1.publicKey,
      {
        accounts: {
          authority: provider.wallet.publicKey,
          
      },
    });
    const state = await program.state.fetch();
    // #endregion accessor
    assert.equal(state.products.length, 1);
    assert.equal(state.products[0].name, "test1");
    assert.equal(state.products[0].status, 0);
    assert.ok(state.products[0].owner.equals(owner1.publicKey));

  });

  it("Executes delegateProduct method on the program", async () => {
    
    
    await program.state.rpc.delegateProduct(
      0,
      owner1.publicKey,
      delegateTo.publicKey,
      {
        accounts: {
          authority: provider.wallet.publicKey,
          
      },
    });

    const state2 = await program.state.fetch();
    
    assert.equal(state2.products.length, 1);
    assert.equal(state2.products[0].name, "test1");
    assert.equal(state2.products[0].status, 1);
    assert.ok(state2.products[0].owner.equals(owner1.publicKey));
    assert.ok(state2.products[0].delegateTo.equals(delegateTo.publicKey));

  });

  it("Executes acceptProduct method on the program", async () => {
    
    
    await program.state.rpc.acceptProduct(
      0,
      delegateTo.publicKey,
      {
        accounts: {
          authority: provider.wallet.publicKey,
          
      },
    });

    const state2 = await program.state.fetch();
    
    assert.equal(state2.products.length, 1);
    assert.equal(state2.products[0].name, "test1");
    assert.equal(state2.products[0].status, 0);
    assert.ok(state2.products[0].owner.equals(delegateTo.publicKey));
    assert.equal(state2.products[0].delegate_to, undefined);

  });

  it("Executes acceptProduct method on the program and fail", async () => {
    
    try {
      await program.state.rpc.acceptProduct(
        0,
        owner1.publicKey,
        {
          accounts: {
            authority: provider.wallet.publicKey,
            
        },
      });
    } catch (err) {
      assert.equal(err.toString(), "InvalidStatus!")
    }

    const state = await program.state.fetch();
    
    assert.equal(state.products.length, 1);
    assert.equal(state.products[0].name, "test1");
    assert.equal(state.products[0].status, 0);
    assert.ok(state.products[0].owner.equals(delegateTo.publicKey));
    assert.equal(state.products[0].delegate_to, undefined);

  });

  

});
