<h1 align="center">🐻 Tanukeys</h1>

Service for storing public keys of users for various purposes such as message signature verification, encrypting information for a specific user, storing cryptocurrency wallet addresses, and simplifying key exchanges between devices. The service also supports post-quantum cryptographic algorithms for future-proofing.

Tanukeys is designed as a **federated system**, allowing instances to interoperate by retrieving and verifying public keys from other Tanukeys instances. This enables seamless key sharing, automatic updates through instance subscriptions, and a decentralized trust model.

> The project is being rebuilt from scratch on a DDD + Hexagonal Architecture + CQRS foundation (based on [rust-ddd-skeleton](https://github.com/otsobide)). The previous implementation is available in the git history.

## Architecture

The project is a Cargo workspace organized following Domain-Driven Design with Hexagonal Architecture (Ports and Adapters). Business logic lives in bounded contexts under `libs/`, has zero knowledge of databases or HTTP frameworks, and all coupling flows inward through repository traits and bus abstractions.

```
tanukeys/
├── libs/
│   ├── kernel/                  # Core bounded context (temporarily hosts every module)
│   │   └── src/users/           # User identity module
│   │       ├── domain/          # User aggregate, value objects, events, repository trait
│   │       ├── application/     # One folder per use case (create, find, update, delete)
│   │       └── infrastructure/  # In-memory persistence
│   └── shared/
│       ├── cqrs/                # CommandBus + QueryBus (TypeId-based dispatch)
│       ├── domain-events/       # EventBus + DomainEventSubscriber
│       └── valueobject/         # Value object primitives + validation error
│
├── tests/
│   └── libs/kernel/             # Unit tests (mocks, object mothers, domain services)
│
├── Makefile                     # Root Makefile (delegates to per-app Makefiles)
└── Cargo.toml                   # Workspace root
```

### Domain model

The `users` module stores basic user information:

- **`UserId`** — a UUID v4, the true identity of a user across the platform.
- **`UserName`** — the public handle: lowercase letters, digits and `-`, `_`, `.` (max 50 chars).
- **`UserDescription`** — optional free-form text (max 600 chars).

Every write publishes a domain event (`tanukeys.kernel.user.{created,updated,deleted}`) on the event bus.

### Dependency rule

Domain → Application → Infrastructure. Domain never imports infrastructure; coupling is via repository traits only.

## Quick start

```bash
make test        # run the whole test suite
make dev/build   # debug build
make build       # release build
make format      # cargo fmt
```

## Roadmap

- [x] `kernel/users` — basic user identity
- [ ] `kernel/crypto_keys` — public key storage
- [ ] HTTP API (Actix-Web) over the kernel context
- [ ] PostgreSQL persistence adapters
- [ ] Federated key retrieval and instance subscriptions
- [ ] Signed subkeys referencing a master key

## License

Licensed under the GNU Affero General Public License v3.0 — see [LICENSE](./LICENSE).
