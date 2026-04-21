# Pi-Health Portal — API Reference

## Overview

The platform exposes two HTTP servers:

| Server | Port | Purpose |
|---|---|---|
| **doc_server** | `5000` | Primary RPC server — all clinical/patient data |
| **meta_server** | `8080` | Search, auth, drugs, doctors, users |

---

## Authentication

All protected endpoints require a **Biscuit token** in the `Authorization` header:

```
Authorization: Bearer <biscuit-token>
```

Tokens are obtained from:
- `POST /login` on the meta server (returns a signed Biscuit token)
- `POST /auth` on the doc server (token-based auth)

---

## doc_server (port 5000)

### Special endpoints

| Method | Path | Auth | Description |
|---|---|---|---|
| `GET` | `/health_check` | None | Liveness probe — returns 200 OK |
| `GET` | `/metrics` | None | Returns JSON: `{ requests_total, uptime_seconds }` |
| `POST` | `/auth` | None | Authenticate and obtain a Biscuit token |
| `POST` | `/phone_code` | None | Send OTP to a mobile number |
| `POST` | `/api` | Required | JSON-RPC 2.0 entrypoint for all clinical methods |

### JSON-RPC 2.0 protocol

All clinical operations go through `POST /api` as a JSON-RPC 2.0 request:

```json
{
  "jsonrpc": "2.0",
  "method": "<Namespace>.<Action>",
  "params": [<input_object>, <org_id>],
  "id": 1
}
```

The response:
```json
{
  "jsonrpc": "2.0",
  "result": <data>,
  "error": null,
  "id": 1
}
```

---

### RPC Namespaces

#### Organization

| Method | Params | Description |
|---|---|---|
| `Organization.Add` | `OrganizationAdd` | Create a new organization |
| `Organization.Update` | `OrganizationUpdate, org_id` | Update organization details |
| `Organization.Get` | `OrganizationsByID` | Get organization by ID |
| `Organization.GetAll` | — | List all organizations |
| `Organization.Delete` | `OrganizationDelete, org_id` | Soft-delete an organization |
| `Organization.Select` | `SelectOrganization, org_id` | Select/switch active organization |
| `Organization.Sync` | `LastUpdatedInput, org_id` | Pull all records updated after given timestamps |
| `Organization.Vaccines` | — | Get all available vaccines |
| `Organization.Drugs` | — | Get all drugs from static data |
| `Organization.Location` | `PinCodeInput, org_id` | Look up cities by pin code |
| `Organization.PinCodes` | — | Get all pin codes |

**Sync endpoint** — `Organization.Sync` is the primary data sync mechanism.

```json
// Request
{
  "jsonrpc": "2.0",
  "method": "Organization.Sync",
  "params": [{
    "patients":          "1970-01-01T00:00:00Z",
    "doctors":           "1970-01-01T00:00:00Z",
    "appointments":      "1970-01-01T00:00:00Z",
    "prescription":      "1970-01-01T00:00:00Z",
    "vitals":            "1970-01-01T00:00:00Z",
    "medication":        "1970-01-01T00:00:00Z",
    "allergy":           "1970-01-01T00:00:00Z",
    "problems":          "1970-01-01T00:00:00Z",
    "note":              "1970-01-01T00:00:00Z",
    "staff":             "1970-01-01T00:00:00Z",
    "user":              "1970-01-01T00:00:00Z",
    "organization":      "1970-01-01T00:00:00Z",
    "service_location":  "1970-01-01T00:00:00Z",
    "system_admin":      "1970-01-01T00:00:00Z",
    "add_historical":    "1970-01-01T00:00:00Z",
    "administer":        "1970-01-01T00:00:00Z",
    "not_administer":    "1970-01-01T00:00:00Z",
    "order":             "1970-01-01T00:00:00Z",
    "familyhistory":     "1970-01-01T00:00:00Z",
    "hospitalization":   "1970-01-01T00:00:00Z",
    "implantabledevices":"1970-01-01T00:00:00Z",
    "obandpregnanacy":   "1970-01-01T00:00:00Z",
    "pastmedicalhistory":"1970-01-01T00:00:00Z",
    "pastsurgicalhistory":"1970-01-01T00:00:00Z",
    "socialhistory":     "1970-01-01T00:00:00Z",
    "limit": 500
  }, "<org_id>"],
  "id": 1
}
```

Pagination: use `limit` (1–1000, default 500). If any table returns exactly `limit` records, re-request with the latest `last_updated` timestamp from those records as the cursor for that table.

---

#### Patients

| Method | Description |
|---|---|
| `Patients.Add` | Add a new patient |
| `Patients.Update` | Update patient demographics |
| `Patients.Get` | Get patient by ID |
| `Patients.GetAll` | List patients in org |
| `Patients.Delete` | Soft-delete patient |
| `Patients.Vitals.Add` | Record vital signs |
| `Patients.Vitals.Get` | Get vitals for patient |
| `Patients.Medications.Add` | Add medication |
| `Patients.Medications.Update` | Update medication |
| `Patients.Medications.Delete` | Remove medication |
| `Patients.Allergy.Add` | Add allergy |
| `Patients.Allergy.Update` | Update allergy |
| `Patients.Allergy.Delete` | Remove allergy |
| `Patients.Problem.Add` | Add problem/diagnosis |
| `Patients.Problem.Update` | Update problem |
| `Patients.Problem.Delete` | Remove problem |
| `Patients.Notes.*` | SOAP, H&P, phone, group, amendment, office form notes |
| `Patients.Immunization.Order` | Order immunization |
| `Patients.Immunization.Administer` | Record administered vaccine |
| `Patients.Immunization.NotAdminister` | Record not-administered vaccine |
| `Patients.Immunization.AddHistorical` | Add historical immunization |
| `Patients.History.FamilyHistory.*` | Family history CRUD |
| `Patients.History.Hospitalization.*` | Hospitalization history CRUD |
| `Patients.History.PastMedical.*` | Past medical history CRUD |
| `Patients.History.PastSurgical.*` | Past surgical history CRUD |
| `Patients.History.SocialHistory.*` | Social history CRUD |
| `Patients.History.OBAndPregnancy.*` | OB & pregnancy history CRUD |
| `Patients.History.ImplantableDevices.*` | Implantable devices CRUD |
| `Patients.Prescription.*` | Prescription management |

#### Doctor

| Method | Description |
|---|---|
| `Doctor.Add` | Register a doctor |
| `Doctor.Update` | Update doctor profile |
| `Doctor.Delete` | Soft-delete doctor |
| `Doctor.Get` | Get doctor by ID |

#### Appointment

| Method | Description |
|---|---|
| `Appointment.Add` | Create appointment |
| `Appointment.Update` | Update appointment |
| `Appointment.Delete` | Cancel appointment |
| `Appointment.Get` | Get appointment by ID |

#### ServiceLocation

| Method | Description |
|---|---|
| `ServiceLocation.Add` | Create service location |
| `ServiceLocation.Update` | Update service location |
| `ServiceLocation.Delete` | Remove service location |
| `ServiceLocation.Get` | Get by ID |

#### Staff

| Method | Description |
|---|---|
| `Staff.Add` | Add staff member |
| `Staff.Update` | Update staff |
| `Staff.Delete` | Remove staff |

#### User

| Method | Description |
|---|---|
| `User.Add` | Create user account |
| `User.Update` | Update user details |
| `User.UpdatePassword` | Change password |
| `User.Delete` | Deactivate user |

#### SystemAdmin

| Method | Description |
|---|---|
| `SystemAdmin.Add` | Create system admin + org |
| `SystemAdmin.Update` | Update system admin |
| `SystemAdmin.Delete` | Remove system admin |

---

## meta_server (port 8080)

### Auth

| Method | Path | Auth | Body | Description |
|---|---|---|---|---|
| `POST` | `/login` | None | `{ mobile_number, password }` | Returns Biscuit token |

### Search (no auth)

| Method | Path | Body | Description |
|---|---|---|---|
| `POST` | `/drugs` | `{ "query": "amox" }` | Full-text drug search via Meilisearch |
| `POST` | `/doctors` | `{ "query": "cardiologist" }` | Doctor search |
| `POST` | `/icd10` | `{ "query": "diabetes" }` | ICD-10 code search |
| `POST` | `/location` | `{ "query": "Mumbai" }` | Location/city search |

### REST API (auth required — `Authorization: Bearer <token>`)

#### Drugs `/api/drugs`

| Method | Path | Description |
|---|---|---|
| `GET` | `/api/drugs` | List drugs |
| `POST` | `/api/drugs` | Add drug |
| `PUT` | `/api/drugs/{id}` | Update drug |
| `DELETE` | `/api/drugs/{id}` | Remove drug |

#### Doctors `/api/doctors`

| Method | Path | Description |
|---|---|---|
| `GET` | `/api/doctors` | List doctors |
| `POST` | `/api/doctors` | Add doctor |
| `PUT` | `/api/doctors/{id}` | Update doctor |
| `DELETE` | `/api/doctors/{id}` | Remove doctor |

#### Clients `/api/clients`

| Method | Path | Description |
|---|---|---|
| `GET` | `/api/clients` | List clients |
| `POST` | `/api/clients` | Add client |
| `PUT` | `/api/clients/{id}` | Update client |
| `DELETE` | `/api/clients/{id}` | Remove client |

#### Users `/api/users`

| Method | Path | Description |
|---|---|---|
| `GET` | `/api/users` | List users |
| `POST` | `/api/users` | Create user |
| `PUT` | `/api/users/{id}` | Update user |
| `DELETE` | `/api/users/{id}` | Remove user |

### Utility

| Method | Path | Auth | Description |
|---|---|---|---|
| `GET` | `/health_check` | None | Liveness probe |

---

## Error codes (JSON-RPC)

| Code | Meaning |
|---|---|
| `-32700` | Parse error — invalid JSON |
| `-32600` | Invalid request object |
| `-32601` | Method not found |
| `-32602` | Invalid params |
| `-32603` | Internal error |

---

## Rate limits

Both servers enforce **120 requests per minute per IP** with a burst allowance of 30. Exceeding this returns `HTTP 429 Too Many Requests`.

---

## Environment variables

| Variable | Required | Description |
|---|---|---|
| `APP_ENVIRONMENT` | Yes | `development` / `staging` / `production` |
| `PRIVATE_KEY` | Yes | Biscuit private key (hex) |
| `PUBLIC_KEY` | Yes | Biscuit public key (hex) |
| `READ_DB_FILE` | Yes (doc_server) | Path to SQLite read DB |
| `WRITE_DB_FILE` | Yes (doc_server) | Path to SQLite write DB |
| `META_DB_FILE` | Yes (meta_server) | Path to SQLite meta DB |
| `MEILI_MASTER_KEY` | Yes (meta_server) | Meilisearch master key |
| `FIRST_USER_ID` | Yes | UUID for the bootstrap system admin |
| `FIRST_USER_MOBILE` | Yes | Mobile number of bootstrap admin |
| `FIRST_USER_FIRST_NAME` | Yes | First name of bootstrap admin |
| `FIRST_USER_PASSWORD` | Yes | Password of bootstrap admin |
| `TEXT_LOCAL_API_KEY` | No | SMS gateway API key |
| `RUST_LOG` | No | Log level — e.g. `info`, `debug` |
