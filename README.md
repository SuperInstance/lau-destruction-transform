# lau-destruction-transform

**Destruction-transform pipelines in the PLATO ecosystem** — deconstruct artifacts into components, discover hidden wisdom through examination, and rebuild them into new forms.

> *Things must be broken to be understood.*

---

## What This Does

This library models the lifecycle of **destructive transformation**: taking something whole, breaking it apart to understand it, and rebuilding it into something new. It provides:

- A **six-phase state machine** for artifacts (Whole → Examining → Scattered → Understanding → Rebuilding → Transformed)
- **Components** with condition ratings and optional hidden wisdom that can only be discovered through examination
- A **TransformLog** that tracks all artifacts, events, and tick-level timing across the full lifecycle
- **Dissolve rules** that decide when a "room" (context/environment) should be dissolved and re-created
- **Pre-built artifacts** (Old Boat, Failed Bridge, Crashed Agent) that model real transformation scenarios
- Full **serde** serialization for persisting and resuming transformation pipelines

This is the conceptual engine behind the PLATO ecosystem's philosophy that destruction is a path to understanding — you must break something apart to truly know it, and from that knowledge comes the ability to rebuild it into something better.

**Stats:** ~530 lines of source, 27 tests, zero unsafe code.

---

## Key Idea

The destruction-transform process mirrors how deep understanding works:

```
Whole          You see something complete and functional
  ↓
Examining      You start taking it apart — each component reveals something
  ↓
Scattered      It's in pieces — the original form is gone
  ↓
Understanding  From the pieces, patterns emerge — hidden wisdom discovered
  ↓
Rebuilding     You reassemble with new knowledge — into a different form
  ↓
Transformed    The thing is new. It carries the wisdom of what it was.
```

The critical constraint: **you can only rebuild if you've discovered wisdom from more than half the components.** Understanding isn't optional — it's a prerequisite for transformation. If you break something without understanding it, you can't rebuild it.

---

## Install

Add to your `Cargo.toml`:

```toml
[dependencies]
lau-destruction-transform = "0.1"
```

### Dependencies

- **serde** (with `derive`) — serialization of artifacts, logs, events, and rules

For JSON serialization in tests, add:

```toml
[dev-dependencies]
serde_json = "1.0"
```

---

## Quick Start

### Create and transform an artifact

```rust
use lau_destruction_transform::{
    TransformLog, Component, Artifact,
};

let mut log = TransformLog::new();

// Create an artifact with components, some carrying hidden wisdom
log.create(
    "Old Radio",
    vec![
        Component::with_wisdom("Speaker", 0.7, "The cone shaped sound — it can shape silence too"),
        Component::with_wisdom("Dial", 0.4, "Every frequency is a door; tuning is choosing which to open"),
        Component::new("Antenna", 0.2),
    ],
    vec!["A wind chime".into(), "A listening device".into()],
);

// Phase 1: Deconstruct
log.deconstruct("Old Radio");

// Phase 2: Examine components to discover hidden wisdom
let wisdom = log.examine("Old Radio", 0); // Speaker → Some("The cone shaped sound...")
log.examine("Old Radio", 1);               // Dial → Some("Every frequency...")
log.examine("Old Radio", 2);               // Antenna → None (no hidden wisdom)

// Phase 3-4: Scatter and understand
log.scatter("Old Radio");
log.understand("Old Radio");

// Phase 5: Rebuild (requires discoveries > components / 2)
let success = log.rebuild("Old Radio", "A wind chime"); // 2 discoveries > 3/2 → true

// Phase 6: Complete the transformation
log.complete("Old Radio");

// Check results
assert_eq!(log.completed().len(), 1);
assert_eq!(log.transform_rate(), 1.0);
```

### Use pre-built artifacts

```rust
use lau_destruction_transform::{old_boat, TransformLog};

let mut log = TransformLog::new();
let boat = old_boat();
// Old Boat: 5 components (Hull, Mast, Rudder, Sails, Anchor)
//           3 with hidden wisdom

log.create("Old Boat", boat.components.clone(), boat.potential_forms.clone());
log.deconstruct("Old Boat");

// Examine all components
for i in 0..boat.components.len() {
    log.examine("Old Boat", i);
}

log.scatter("Old Boat");
log.understand("Old Radio");
log.rebuild("Old Boat", "A sleek yacht");
log.complete("Old Boat");
```

### Serialize and deserialize state

```rust
let log: TransformLog = /* ... */;
let json = serde_json::to_string(&log).unwrap();

// Later: resume where you left off
let restored: TransformLog = serde_json::from_str(&json).unwrap();
assert_eq!(log.tick, restored.tick);
```

---

## API Reference

### `TransformId`

A newtype wrapper around `String` for identifying transforms. Implements `Display`, `Hash`, `Eq`, `Clone`, `Serialize`, `Deserialize`.

```rust
let id = TransformId("plato-room".to_string());
```

### `TransformPhase`

The six-phase state machine:

| Phase | Meaning |
|---|---|
| `Whole` | Initial state — artifact is intact |
| `Examining` | Deconstruction started — components are being examined |
| `Scattered` | Artifact is fully disassembled |
| `Understanding` | Making sense of the pieces |
| `Rebuilding` | Reassembling into a new form |
| `Transformed` | Transformation complete |

### `Component`

```rust
Component::new("Part", 0.8)                        // No hidden wisdom
Component::with_wisdom("Part", 0.5, "secret wisdom") // Has discoverable wisdom
```

Fields:
- `name: String` — component identifier
- `condition: f64` — condition rating (0.0–1.0, higher = better)
- `hidden_wisdom: Option<String>` — wisdom revealed on first examination, then consumed

### `Artifact`

```rust
let mut art = Artifact::new("Thing", "a thing", vec!["New Form".into()]);
```

| Method | Description |
|---|---|
| `.deconstruct(tick)` | Move to `Examining` phase, record destruction tick |
| `.examine_component(idx)` | Examine a component, returns hidden wisdom (first time only) |
| `.scatter()` | Move to `Scattered` phase |
| `.understand()` | Move to `Understanding` phase |
| `.rebuild(new_form, tick)` | Rebuild if discoveries > components/2, returns success |
| `.complete()` | Move to `Transformed` phase |
| `.is_transforming()` | True if in any mid-transformation phase |
| `.discovery_count()` | Number of wisdom entries discovered so far |
| `.hidden_count()` | Number of components with undiscovered wisdom |

### `TransformEvent`

Events emitted during the lifecycle:

| Variant | When |
|---|---|
| `DeconstructionStarted(artifact)` | Artifact enters `Examining` |
| `ComponentExamined { artifact, component, discovery }` | A component is examined |
| `WisdomFound { artifact, wisdom }` | Hidden wisdom is discovered |
| `RebuildAttempted { artifact, success }` | A rebuild is attempted |
| `TransformationComplete { artifact, old_form, new_form }` | Rebuild succeeds |
| `RoomDissolved { room, reason }` | A room/context is dissolved |

### `TransformLog`

The central orchestrator. Tracks all artifacts and events with tick-level timing.

| Method | Description |
|---|---|
| `::new()` | Create an empty log |
| `.create(name, components, potential_forms)` | Register a new artifact |
| `.deconstruct(name)` | Start deconstruction (increments tick) |
| `.examine(artifact, component_idx)` | Examine a component, returns discovered wisdom |
| `.scatter(name)` / `.understand(name)` | Phase transitions |
| `.rebuild(name, new_form)` | Attempt rebuild (increments tick) |
| `.complete(name)` | Finalize transformation |
| `.get(name)` | Look up an artifact |
| `.transforming()` | Artifacts currently mid-transformation |
| `.completed()` | Fully transformed artifacts |
| `.total_discoveries()` | Wisdom found across all artifacts |
| `.hidden_remaining()` | Undiscovered wisdom remaining |
| `.transform_rate()` | completed / total ratio |
| `.events_for(name)` | All events for a specific artifact |

### `DissolveRule`

```rust
let rule = DissolveRule {
    condition: "over-adapted".into(),
    trigger_threshold: 0.3,
    preserve: vec!["layout".into()],
    new_potential: vec!["garden".into()],
};
rule.should_dissolve(0.2); // true — score ≤ threshold
```

Determines when a context (room) should be dissolved and recreated. Fields:
- `condition` — why the room should dissolve
- `trigger_threshold` — dissolve when adaptation score ≤ this value
- `preserve` — aspects to carry over to the new room
- `new_potential` — possible forms the new room could take

### Pre-built Artifacts

| Function | Components | Hidden Wisdom | Description |
|---|---|---|---|
| `old_boat()` | 5 (Hull, Mast, Rudder, Sails, Anchor) | 3 | A weathered fishing boat with maritime wisdom |
| `failed_bridge()` | 3 (Cables, Deck, Pillars) | 2 | A collapsed suspension bridge with structural wisdom |
| `crashed_agent()` | 4 (Memory Core, Decision Engine, Communication Bus, Ethical Guardrail) | 3 | A corrupted AI agent with computational wisdom |

---

## How It Works

### The Wisdom Mechanic

Each component may carry a piece of hidden wisdom — a string that's only revealed the first time the component is examined. The wisdom is then "consumed" (set to `None`), preventing double-discovery. This models the insight that understanding something once is enough; the knowledge persists in the artifact's `discoveries` list.

### The Rebuild Gate

An artifact can only be rebuilt if `discoveries.len() > components.len() / 2`. This enforces the philosophy that **majority understanding** is required before transformation. If you haven't examined enough components to discover their hidden wisdom, you can't rebuild — you'd just be reassembling without understanding.

The division is integer division: for 3 components, the threshold is > 1 (i.e., ≥ 2 discoveries). For 5 components, > 2 (i.e., ≥ 3).

### The Tick System

The `TransformLog` maintains a monotonically increasing tick counter. It increments on deconstruction and rebuild attempts, providing a temporal ordering of events. Each artifact records its `destruction_tick` and `rebuild_tick`.

### Event Sourcing

Every meaningful action produces a `TransformEvent`. The log is an append-only event stream that can be queried by artifact name. Combined with serde serialization, this enables:
- **Audit trails**: see exactly what happened to each artifact
- **Undo/replay**: the event log is the source of truth
- **Analytics**: count discoveries, measure transformation rates, track timing

### Dissolve Rules

Rooms (contexts/environments) can be dissolved when their adaptation score falls below a threshold. Dissolve rules specify what to preserve and what new potentials emerge — modeling creative destruction at the environmental level.

---

## The Philosophy

This library embodies the PLATO ecosystem's core insight:

> *Destruction is not loss — it is the prerequisite for understanding.*

The six-phase lifecycle mirrors how genuine understanding works:

1. **Whole**: You see something complete. It works. You don't question it.
2. **Examining**: You start taking it apart. Each piece reveals something unexpected.
3. **Scattered**: The original is gone. This is uncomfortable — but necessary.
4. **Understanding**: From the scattered pieces, patterns emerge. Hidden wisdom surfaces.
5. **Rebuilding**: With new understanding, you create something different. Not better or worse — *new*.
6. **Transformed**: The result carries the wisdom of its previous form. It is more than it was.

The wisdom embedded in the pre-built artifacts reflects this:

- *"The wood remembers the sea"* — experience shapes form
- *"The anchor was never meant to hold the boat still — it was meant to teach patience"* — purpose is often hidden
- *"Forgetting made room for insight"* — loss creates space for understanding
- *"Rigidity is not the same as safety"* — strength has many forms

---

## License

MIT
