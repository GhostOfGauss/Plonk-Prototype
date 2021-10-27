# manta-circuits

Plonk based circuit implementation for manta network

## Modules

### `zk`

implements gadgets and circuits for manta network

### `signer`

### Command

Use this command to run plonk official example:
* `cargo run --bin plonk_example`

Use this command to run variable length pedersen commitment written in plonk
* `cargo run --bin pedersen_example`

To test the constraint size and performance of Arkworks BH hashing, please first checkout branch `arkwork_bh_hash`.
Then, use the following command:
* TODO