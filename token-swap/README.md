# Token-swap program

A Uniswap-like exchange for the Token program on the Solana blockchain, with an identity gate.

Full documentation is available at https://spl.solana.com/token-swap

For details on the Identity gate, see [the identity program](../identity/README.md) and https://github.com/civicteam/identity-swap

JavaScript binding are available in the `./js` directory.

## Using the Token-Swap program with an identity gate.

### Creating a swap

When creating a swap, add the public key of the IdV that validates the identity accounts of the user.

For example, with the JS client:
```
TokenSwap.createTokenSwap(
    connection,
    swapPayer,
    tokenSwapAccount,
    authority,
    tokenAccountA,
    tokenAccountB,
    tokenPool.publicKey,
    mintA.publicKey,
    mintB.publicKey,
    feeAccount,
    tokenAccountPool,
    idv.publicKey,          <--- The IdV account key
    tokenSwapProgramId,
    tokenProgramId,
    nonce,
    CURVE_TYPE,
    TRADING_FEE_NUMERATOR,
    TRADING_FEE_DENOMINATOR,
    OWNER_TRADING_FEE_NUMERATOR,
    OWNER_TRADING_FEE_DENOMINATOR,
    OWNER_WITHDRAW_FEE_NUMERATOR,
    OWNER_WITHDRAW_FEE_DENOMINATOR,
    HOST_FEE_NUMERATOR,
    HOST_FEE_DENOMINATOR,
  );
```

### Making a swap

When swapping tokens, pass an identity account public key that has an attestation
registered by the same IdV. See [the identity program JS client](../identity/js/README.md)
for details on how to create an identity account.

e.g, with the JS client:

```
tokenSwap.swap(
    userAccountA,
    tokenAccountA,
    tokenAccountB,
    userAccountB,
    identityAccount,    <-- The identity account public key
    poolAccount,
    SWAP_AMOUNT_IN,
    SWAP_AMOUNT_OUT,
  );
```

The transaction signer must be the owner of the identity account.

Note: at present, only the swap method, not deposit and withdraw, is gated by the identity program.
No identity is required when depositing or withdrawing funds.
