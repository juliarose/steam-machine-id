# steam-machine-id

Used for generating Steam machine IDs. Based on [node-steam-user](https://github.com/DoctorMcKay/node-steam-user).

## Usage

Generating random machine IDs.
```rs
use steam_machine_id::MachineId;

// Creates a random machine ID.
let machine_id = MachineId::random();
```

Consuming a generated machine ID for a login request.
```rs
use steam_machine_id::MachineId;

struct LoginRequest {
    machine_id: Vec<u8>,
}

// Creates a machine ID from the given account name.
let machine_id = MachineId::from_account_name("accountname");
let login = LoginRequest {
    // Converts the machine ID into a binary message object.
    machine_id: machine_id.into(),
};
```

## License

[MIT](https://github.com/juliarose/steam-machine-id/tree/main/LICENSE)
