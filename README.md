# Level 0: Hello Solana

This is a guided warm-up, not a security challenge. Your goal is to call the challenge program once and drain its 1 SOL prize PDA.

You receive:

- `challenge/src/lib.rs`: the program you must call
- `solve/src/lib.rs`: the program you edit
- `client/src/main.rs`: the uploader that connects to the server
- `Dockerfile`: builds `solve.so` and runs the uploader

## What the server does

1. Loads the challenge program.
2. Reads and deploys your uploaded `solve.so`.
3. Creates a transaction whose top-level instruction calls your solve program.
4. Passes the player, prize, and challenge program accounts to your solve program.
5. Returns the flag if the prize balance becomes zero.

## What to edit

Edit only `solve/src/lib.rs`. Build an `Instruction` for the challenge program and call it with `invoke`.

Challenge instruction:

- data: `[0]`
- account 0: player, signer and writable
- account 1: prize PDA, writable

## Program API

- `[0] ClaimWelcomeGrant`: transfers the welcome vault balance to the signing player
- `[1] QuoteWelcomeGrant`: returns the current grant amount as a little-endian `u64`
- `[2] GetServiceMetadata`: returns the service identifier and version

Accounts received by your solve program:

1. player, signer and writable
2. prize PDA, writable
3. challenge program, executable

## Build

```bash
docker build --platform linux/amd64 -t hello-solana-solve .
```

## Submit

```bash
docker run --rm --platform linux/amd64 hello-solana-solve HOST PORT
```

For the local organizer instance on Docker Desktop:

```bash
docker run --rm --platform linux/amd64 hello-solana-solve host.docker.internal 5000
```

## Expected call shape

```rust
invoke(
    &Instruction {
        program_id: *challenge_program.key,
        accounts: vec![
            AccountMeta::new(*player.key, true),
            AccountMeta::new(*prize.key, false),
        ],
        data: vec![0],
    },
    &[player.clone(), prize.clone(), challenge_program.clone()],
)
```
