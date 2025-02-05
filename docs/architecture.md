locci-vault/
│
├── Cargo.toml                  # Workspace configuration
│
├── crates/
│   ├── core/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── vault.rs         # Main Vault struct implementation
│   │       ├── auth/
│   │       │   ├── mod.rs
│   │       │   ├── token.rs
│   │       │   └── policy.rs
│   │       ├── storage/
│   │       │   ├── mod.rs
│   │       │   ├── backend.rs
│   │       │   └── physical.rs
│   │       ├── secrets/
│   │       │   ├── mod.rs
│   │       │   ├── engine.rs
│   │       │   └── transit.rs
│   │       └── crypto/
│   │           ├── mod.rs
│   │           └── encryption.rs
│   │
│   ├── http/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── routes/
│   │       │   ├── secrets.rs
│   │       │   └── auth.rs
│   │       └── middleware/
│   │           └── authentication.rs
│   │
│   ├── grpc/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       └── services/
│   │           ├── secrets.rs
│   │           └── auth.rs
│   │
│   └── cli/
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs
│           ├── commands/
│           │   ├── create.rs
│           │   ├── read.rs
│           │   └── delete.rs
│           └── config.rs
│
├── deployments/
│   ├── helm/
│   │   ├── Chart.yaml
│   │   ├── values.yaml
│   │   └── templates/
│   │       ├── deployment.yaml
│   │       ├── service.yaml
│   │       └── crds.yaml
│   │
│   └── kustomize/
│       ├── base/
│       └── overlays/
│
├── examples/
│   ├── custom-storage-backend/
│   └── custom-auth-provider/
│
├── tests/
│   ├── integration/
│   └── e2e/
│
└── docs/
    ├── architecture.md
    └── usage.md
