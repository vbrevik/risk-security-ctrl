Now I have all the context needed. Here is the section content.

# Section 2: Permission Matrix

## Overview

This section defines the authorization permission model for the application. It introduces three enums (`Feature`, `Action`, `Role`), a static `has_permission()` lookup function, and a `HasPermission` trait implemented on the `AuthUser` struct. All code lives in a single new file: `backend/src/features/auth/permissions.rs`.

The permission matrix is entirely static -- no database queries, no runtime allocation. It is encoded as a `match` expression. This section is a prerequisite for Section 03 (RBAC Middleware), which consumes these types and the `has_permission()` function to enforce authorization on routes.

## Dependencies

- **Section 01 (deps-and-config)** must be completed first so that the cargo dependencies are in place.
- **Split 01 (01-backend-core-auth)** must be completed so that the `AuthUser` struct exists in `backend/src/features/auth/models.rs`. The `AuthUser` struct has fields: `id: String`, `email: String`, `name: String`, `role: String`, `session_id: String`.

## Files to Create/Modify

| Action | File Path |
|--------|-----------|
| Create | `backend/src/features/auth/permissions.rs` |
| Modify | `backend/src/features/auth/mod.rs` |

## Tests First

All tests live in `backend/src/features/auth/permissions.rs` inside a `#[cfg(test)] mod tests` block. They validate the permission matrix logic and the `HasPermission` trait.

### Test: admin has all permissions on all features

```rust
/// Iterate over every combination of Feature and Action.
/// Assert has_permission returns true for Role::Admin on all of them.
#[test]
fn admin_has_all_permissions() { todo!() }
```

### Test: risk_manager can read/create/update/delete on compliance

```rust
/// Assert has_permission returns true for Role::RiskManager on:
///   (Feature::Compliance, Action::Read)
///   (Feature::Compliance, Action::Create)
///   (Feature::Compliance, Action::Update)
///   (Feature::Compliance, Action::Delete)
#[test]
fn risk_manager_full_compliance_access() { todo!() }
```

### Test: risk_manager cannot manage users

```rust
/// Assert has_permission returns false for Role::RiskManager on
/// (Feature::Auth, Action::ManageUsers).
#[test]
fn risk_manager_cannot_manage_users() { todo!() }
```

### Test: specialist can create compliance assessments

```rust
/// Assert has_permission returns true for Role::Specialist on
/// (Feature::Compliance, Action::Create).
#[test]
fn specialist_can_create_compliance() { todo!() }
```

### Test: specialist cannot delete compliance assessments

```rust
/// Assert has_permission returns false for Role::Specialist on
/// (Feature::Compliance, Action::Delete).
#[test]
fn specialist_cannot_delete_compliance() { todo!() }
```

### Test: specialist cannot write ontology

```rust
/// Assert has_permission returns false for Role::Specialist on
/// (Feature::Ontology, Action::Create), (Feature::Ontology, Action::Update),
/// and (Feature::Ontology, Action::Delete).
#[test]
fn specialist_cannot_write_ontology() { todo!() }
```

### Test: viewer can read all features

```rust
/// For every Feature variant, assert has_permission returns true for
/// Role::Viewer on Action::Read.
#[test]
fn viewer_can_read_all_features() { todo!() }
```

### Test: viewer cannot create/update/delete anything

```rust
/// For every Feature variant, assert has_permission returns false for
/// Role::Viewer on Action::Create, Action::Update, and Action::Delete.
#[test]
fn viewer_cannot_create_update_delete() { todo!() }
```

### Test: viewer cannot export reports

```rust
/// Assert has_permission returns false for Role::Viewer on
/// (Feature::Reports, Action::Export).
#[test]
fn viewer_cannot_export_reports() { todo!() }
```

### Test: unknown role string defaults to viewer permissions

```rust
/// Construct an AuthUser with role set to "unknown_role".
/// Call has_permission_trait method (via HasPermission).
/// Assert it behaves identically to a Viewer -- reads allowed, writes denied.
#[test]
fn unknown_role_defaults_to_viewer() { todo!() }
```

### Test: HasPermission trait works on AuthUser struct

```rust
/// Construct an AuthUser with role "admin".
/// Call .has_permission(Feature::Auth, Action::ManageUsers) via the trait.
/// Assert it returns true.
/// Construct an AuthUser with role "viewer".
/// Call .has_permission(Feature::Compliance, Action::Delete).
/// Assert it returns false.
#[test]
fn has_permission_trait_on_auth_user() { todo!() }
```

## Implementation Details

### 1. Create `backend/src/features/auth/permissions.rs`

#### Enums

Define three enums, all deriving `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`:

**`Feature`** -- represents a domain area of the application:
- `Ontology` -- ontology concept management
- `Compliance` -- compliance assessments and items
- `Analysis` -- document analysis engine
- `Reports` -- report generation and export
- `Auth` -- user management operations

**`Action`** -- represents an operation type:
- `Read` -- viewing/listing resources
- `Create` -- creating new resources
- `Update` -- modifying existing resources
- `Delete` -- removing resources
- `Export` -- exporting data (reports)
- `ManageUsers` -- user administration

**`Role`** -- represents a user authorization level:
- `Admin`
- `RiskManager`
- `Specialist`
- `Viewer`

#### Role::from_str

Implement `std::str::FromStr` for `Role`. The parsing should be case-insensitive and handle both snake_case and lowercase forms:

- `"admin"` -> `Role::Admin`
- `"risk_manager"` or `"riskmanager"` -> `Role::RiskManager`
- `"specialist"` -> `Role::Specialist`
- `"viewer"` -> `Role::Viewer`
- Any unrecognized string -> return `Err(...)`. The caller (the `HasPermission` trait impl) will use `unwrap_or(Role::Viewer)` to default to the most restrictive role, and should log a warning when this happens.

#### has_permission() Function

```rust
/// Static permission check. No allocations, no I/O.
/// Returns true if the given role is allowed to perform the action on the feature.
pub fn has_permission(role: &Role, feature: &Feature, action: &Action) -> bool
```

The function body is a match expression. The rules are:

1. **Admin:** Always returns `true` for any feature/action combination.

2. **All roles:** `Action::Read` returns `true` for any feature. All authenticated users can read all data.

3. **RiskManager:**
   - `Compliance`: Create, Update, Delete -> true
   - `Analysis`: Create, Update, Delete -> true
   - `Reports`: Export -> true
   - `Ontology`: Create, Update -> true (risk managers can modify ontology)
   - `Ontology`: Delete -> false (only admins delete ontology concepts)
   - `Auth`: ManageUsers -> false

4. **Specialist:**
   - `Compliance`: Create, Update -> true
   - `Analysis`: Create, Update -> true
   - Everything else (Delete, Export, ManageUsers, Ontology writes) -> false

5. **Viewer:**
   - Only `Read` on any feature -> true
   - Everything else -> false

6. **Default catch-all:** `false` (deny by default).

#### HasPermission Trait

```rust
/// Trait for checking permissions on authenticated entities.
pub trait HasPermission {
    fn has_permission(&self, feature: Feature, action: Action) -> bool;
}
```

#### HasPermission impl for AuthUser

```rust
use super::models::AuthUser;

impl HasPermission for AuthUser {
    fn has_permission(&self, feature: Feature, action: Action) -> bool {
        let role = self.role.parse::<Role>().unwrap_or_else(|_| {
            tracing::warn!(
                user_id = %self.id,
                role = %self.role,
                "Unrecognized role, defaulting to Viewer"
            );
            Role::Viewer
        });
        has_permission(&role, &feature, &action)
    }
}
```

This implementation:
- Parses the string `role` field from `AuthUser` into the `Role` enum
- Falls back to `Role::Viewer` (most restrictive) if parsing fails
- Logs a warning via `tracing` when an unrecognized role is encountered
- Delegates to the static `has_permission()` function

### 2. Update `backend/src/features/auth/mod.rs`

Add the permissions module declaration:

```rust
pub mod models;
pub mod permissions;
pub mod routes;
```

This makes `Feature`, `Action`, `Role`, `HasPermission`, and `has_permission()` accessible as `crate::features::auth::permissions::*` throughout the codebase. Section 03 (RBAC Middleware) will import from this module.

## Key Design Decisions

- **Static match, not a data structure:** A match expression is zero-cost at runtime, requires no initialization, and the compiler can verify exhaustiveness. No need for `phf` or `HashMap` for five roles and six actions.
- **Deny by default:** The match catch-all returns `false`. Any new Feature/Action variant added in the future will be denied until explicitly granted. This is the secure default.
- **Role parsing fallback:** Unrecognized role strings default to `Viewer` rather than returning an error to the user. This prevents privilege escalation through role string manipulation while keeping the system functional.
- **Trait-based design:** The `HasPermission` trait allows the permission check to be called ergonomically on `AuthUser` instances and can be extended to other types in the future if needed.
- **Read is universal:** The spec explicitly states all authenticated users can read all data. This is enforced in the matrix so the RBAC middleware (Section 03) does not need to guard GET endpoints.