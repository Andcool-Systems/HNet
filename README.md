# 📡 Hypothetical Network (HNet)

**HNet** is a routing emulator for a hypothetical hierarchical computer network. It models a system with an alternative, human-readable IP addressing format, designed around geographic and administrative hierarchy.

---

## 🌐 HNet Address Format

HNet uses a globally structured address format that is both machine-readable and human-friendly. Each address reflects a fixed geographic and administrative location.

```
[2 bytes]  Continent   — "EU", "AS", "NA", "AF", "SA", "OC", "AN"
[2 bytes]  Country     — ISO-3166 code, e.g., "RU", "US", "FR"
[4 bytes]  Provider    — Alphanumeric identifier, e.g., "MGTS", "NTNT"
[4 bytes]  Subnet      — Hex value, unique within the provider, e.g., 045A
[4 bytes]  Host        — Device identifier within the subnet, e.g., 1C2F
```

### 🔍 Example Address

```
NA:US:NTNT:045A:1C2F
^  ^  ^     ^     ^
|  |  |     |     └── Device 0x1C2F
|  |  |     └──────── Subnet 0x045A
|  |  └────────────── Provider "NTNT"
|  └───────────────── Country "US"
└──────────────────── Continent "NA"
```

---

## 🧭 Routing Model

Each router (`HNetRouter`) follows this internal structure:

### 📘 Routing Table (`routing_table`)

Stores mappings from hierarchical address prefixes to interface identifiers:

```rust
HashMap<String, String> // prefix => interface_id
```

> Prefixes are matched in order of decreasing specificity:
>
> * `EU:RU:MGTS:045A:1C2F`
> * `EU:RU:MGTS:045A:*`
> * `EU:RU:MGTS:*:*`
> * `EU:RU:*:*:*`
> * `EU:*:*:*:*`

### 🔌 Interface Table (`interfaces`)

Maps interface IDs to neighboring routers (connected peers):

```rust
HashMap<String, HNetRouter> // interface_id => connected router
```

---

## 🚚 Routing Algorithm

1. A packet is received by a router.
2. The router generates all prefix variants for the destination (from most to least specific).
3. It checks the `routing_table` for the first matching prefix.
4. It retrieves the associated `interface_id` and forwards the packet through the interface using `interfaces`.
5. The process repeats until the packet reaches its final destination or no route is found.

---

**by AndcoolSystems, 17 May, 2025**

