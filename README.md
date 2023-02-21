## Test vectors for Orchard Tx Building

Currently, there is only one test.

A tx with 2 orchard inputs and one output is built
using a deterministic RNG.

We vary the seed of the RNG and record the raw transaction 
bytes. 

The test vectors are created from a desktop computer
and bundled in the mobile app.

## Test App

There are two Android versions: 32 and 64 bits.

The 64 bit version recalculates the expected raw transactions.

But the 32 bit version produces different results.
