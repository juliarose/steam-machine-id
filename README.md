# steam-machine-id

Used for generating Steam machine IDs. Based on [node-steam-user](https://github.com/DoctorMcKay/node-steam-user).

## Usage

Generating random machine IDs.
```rust
use steam_machine_id::MachineID;

// Creates a random machine ID.
let machine_id = MachineID::random();
```

Consuming a generated machine ID for a login request.
```rust
use steam_machine_id::MachineID;

// Creates a machine ID from the given account name.
let machine_id = MachineID::from_account_name("accountname");
// Converts the machine ID into a binary message object
// that can be sent to Steam.
let machine_id_bytes: Vec<u8> = machine_id.into();
```

## License

[MIT](https://github.com/juliarose/steam-machine-id/tree/main/LICENSE)
