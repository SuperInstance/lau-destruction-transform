use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ──────────────────────────────────────────────
// 1. TransformId
// ──────────────────────────────────────────────

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct TransformId(pub String);

impl std::fmt::Display for TransformId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ──────────────────────────────────────────────
// 2. TransformPhase
// ──────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransformPhase {
    Whole,
    Examining,
    Scattered,
    Understanding,
    Rebuilding,
    Transformed,
}

impl std::fmt::Display for TransformPhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransformPhase::Whole => write!(f, "Whole"),
            TransformPhase::Examining => write!(f, "Examining"),
            TransformPhase::Scattered => write!(f, "Scattered"),
            TransformPhase::Understanding => write!(f, "Understanding"),
            TransformPhase::Rebuilding => write!(f, "Rebuilding"),
            TransformPhase::Transformed => write!(f, "Transformed"),
        }
    }
}

// ──────────────────────────────────────────────
// 3. Component & Artifact
// ──────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    pub name: String,
    pub condition: f64,
    pub hidden_wisdom: Option<String>,
}

impl Component {
    pub fn new(name: &str, condition: f64) -> Self {
        Self {
            name: name.to_string(),
            condition,
            hidden_wisdom: None,
        }
    }

    pub fn with_wisdom(name: &str, condition: f64, wisdom: &str) -> Self {
        Self {
            name: name.to_string(),
            condition,
            hidden_wisdom: Some(wisdom.to_string()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    pub name: String,
    pub phase: TransformPhase,
    pub components: Vec<Component>,
    pub discoveries: Vec<String>,
    pub original_form: String,
    pub potential_forms: Vec<String>,
    pub destruction_tick: Option<u64>,
    pub rebuild_tick: Option<u64>,
}

impl Artifact {
    pub fn new(name: &str, original_form: &str, potential_forms: Vec<String>) -> Self {
        Self {
            name: name.to_string(),
            phase: TransformPhase::Whole,
            components: Vec::new(),
            discoveries: Vec::new(),
            original_form: original_form.to_string(),
            potential_forms,
            destruction_tick: None,
            rebuild_tick: None,
        }
    }

    /// Move the artifact into Examining phase and record the destruction tick.
    pub fn deconstruct(&mut self, tick: u64) {
        self.phase = TransformPhase::Examining;
        self.destruction_tick = Some(tick);
    }

    /// Examine a specific component, discovering hidden wisdom if it exists.
    /// Returns `None` if the index is out of bounds or the wisdom was already discovered.
    pub fn examine_component(&mut self, idx: usize) -> Option<&str> {
        let component = self.components.get_mut(idx)?;
        if let Some(wisdom) = component.hidden_wisdom.take() {
            self.discoveries.push(wisdom.clone());
            Some(Box::leak(Box::new(wisdom)).as_str())
        } else {
            None
        }
    }

    /// Move the artifact into Scattered phase.
    pub fn scatter(&mut self) {
        self.phase = TransformPhase::Scattered;
    }

    /// Move the artifact into Understanding phase.
    pub fn understand(&mut self) {
        self.phase = TransformPhase::Understanding;
    }

    /// Attempt to rebuild the artifact into a new form.
    /// Requires that discoveries > components.len() / 2 (i.e., majority understood).
    pub fn rebuild(&mut self, new_form: &str, tick: u64) -> bool {
        if self.discoveries.len() > self.components.len() / 2 {
            self.phase = TransformPhase::Rebuilding;
            self.original_form = new_form.to_string();
            self.rebuild_tick = Some(tick);
            true
        } else {
            false
        }
    }

    /// Complete the transformation into Transformed phase.
    pub fn complete(&mut self) {
        self.phase = TransformPhase::Transformed;
    }

    /// Returns true if the artifact is in a transforming phase (not Whole or Transformed).
    pub fn is_transforming(&self) -> bool {
        matches!(
            self.phase,
            TransformPhase::Examining
                | TransformPhase::Scattered
                | TransformPhase::Understanding
                | TransformPhase::Rebuilding
        )
    }

    /// Number of discoveries made so far.
    pub fn discovery_count(&self) -> usize {
        self.discoveries.len()
    }

    /// Number of components that still have undiscovered hidden wisdom.
    pub fn hidden_count(&self) -> usize {
        self.components
            .iter()
            .filter(|c| c.hidden_wisdom.is_some())
            .count()
    }
}

// ──────────────────────────────────────────────
// 4. TransformEvent
// ──────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransformEvent {
    DeconstructionStarted(String),
    ComponentExamined {
        artifact: String,
        component: String,
        discovery: Option<String>,
    },
    WisdomFound {
        artifact: String,
        wisdom: String,
    },
    RebuildAttempted {
        artifact: String,
        success: bool,
    },
    TransformationComplete {
        artifact: String,
        old_form: String,
        new_form: String,
    },
    RoomDissolved {
        room: String,
        reason: String,
    },
}

// ──────────────────────────────────────────────
// 5. TransformLog
// ──────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformLog {
    pub artifacts: HashMap<String, Artifact>,
    pub events: Vec<TransformEvent>,
    pub tick: u64,
}

impl TransformLog {
    pub fn new() -> Self {
        Self {
            artifacts: HashMap::new(),
            events: Vec::new(),
            tick: 0,
        }
    }

    /// Create a new artifact in the Whole phase.
    pub fn create(
        &mut self,
        name: &str,
        components: Vec<Component>,
        potential_forms: Vec<String>,
    ) {
        let mut artifact = Artifact::new(name, name, potential_forms);
        artifact.components = components;
        self.artifacts.insert(name.to_string(), artifact);
    }

    /// Begin deconstruction of an artifact.
    pub fn deconstruct(&mut self, name: &str) {
        self.tick += 1;
        if let Some(artifact) = self.artifacts.get_mut(name) {
            artifact.deconstruct(self.tick);
            self.events
                .push(TransformEvent::DeconstructionStarted(name.to_string()));
        }
    }

    /// Examine a component of an artifact, discovering hidden wisdom if present.
    pub fn examine(&mut self, artifact: &str, component_idx: usize) -> Option<String> {
        let component_name = self
            .artifacts
            .get(artifact)?
            .components
            .get(component_idx)
            .map(|c| c.name.clone())?;

        let discovery = self
            .artifacts
            .get_mut(artifact)?
            .examine_component(component_idx)
            .map(|s| s.to_string());

        // Record the examination event
        self.events.push(TransformEvent::ComponentExamined {
            artifact: artifact.to_string(),
            component: component_name.clone(),
            discovery: discovery.clone(),
        });

        // If wisdom was found, also record a WisdomFound event
        if let Some(ref wisdom) = discovery {
            self.events.push(TransformEvent::WisdomFound {
                artifact: artifact.to_string(),
                wisdom: wisdom.clone(),
            });
        }

        discovery
    }

    /// Move an artifact to the Scattered phase.
    pub fn scatter(&mut self, name: &str) {
        if let Some(artifact) = self.artifacts.get_mut(name) {
            artifact.scatter();
        }
    }

    /// Move an artifact to the Understanding phase.
    pub fn understand(&mut self, name: &str) {
        if let Some(artifact) = self.artifacts.get_mut(name) {
            artifact.understand();
        }
    }

    /// Attempt to rebuild an artifact into a new form. Returns true on success.
    pub fn rebuild(&mut self, name: &str, new_form: &str) -> bool {
        self.tick += 1;
        let old_form = self.artifacts.get(name).map(|a| a.original_form.clone());
        let success = self
            .artifacts
            .get_mut(name)
            .map(|a| a.rebuild(new_form, self.tick))
            .unwrap_or(false);

        self.events.push(TransformEvent::RebuildAttempted {
            artifact: name.to_string(),
            success,
        });

        if success && let Some(old) = old_form {
            self.events.push(TransformEvent::TransformationComplete {
                artifact: name.to_string(),
                old_form: old,
                new_form: new_form.to_string(),
            });
        }

        success
    }

    /// Complete the transformation of an artifact.
    pub fn complete(&mut self, name: &str) {
        if let Some(artifact) = self.artifacts.get_mut(name) {
            artifact.complete();
        }
    }

    /// Get a reference to an artifact by name.
    pub fn get(&self, name: &str) -> Option<&Artifact> {
        self.artifacts.get(name)
    }

    /// Get all artifacts currently in a transforming phase.
    pub fn transforming(&self) -> Vec<&Artifact> {
        self.artifacts
            .values()
            .filter(|a| a.is_transforming())
            .collect()
    }

    /// Get all completed (Transformed) artifacts.
    pub fn completed(&self) -> Vec<&Artifact> {
        self.artifacts
            .values()
            .filter(|a| matches!(a.phase, TransformPhase::Transformed))
            .collect()
    }

    /// Total number of discoveries across all artifacts.
    pub fn total_discoveries(&self) -> usize {
        self.artifacts.values().map(|a| a.discoveries.len()).sum()
    }

    /// Total number of hidden wisdom entries still undiscovered across all artifacts.
    pub fn hidden_remaining(&self) -> usize {
        self.artifacts.values().map(|a| a.hidden_count()).sum()
    }

    /// Transformation rate: completed artifacts / total artifacts.
    /// Returns 0.0 if there are no artifacts.
    pub fn transform_rate(&self) -> f64 {
        if self.artifacts.is_empty() {
            return 0.0;
        }
        let completed = self.completed().len() as f64;
        let total = self.artifacts.len() as f64;
        completed / total
    }

    /// Get all events related to a specific artifact name.
    pub fn events_for(&self, name: &str) -> Vec<&TransformEvent> {
        self.events
            .iter()
            .filter(|e| match e {
                TransformEvent::DeconstructionStarted(a)
                | TransformEvent::RebuildAttempted { artifact: a, .. }
                | TransformEvent::TransformationComplete { artifact: a, .. } => a == name,
                TransformEvent::ComponentExamined { artifact: a, .. }
                | TransformEvent::WisdomFound { artifact: a, .. } => a == name,
                TransformEvent::RoomDissolved { .. } => false,
            })
            .collect()
    }
}

impl Default for TransformLog {
    fn default() -> Self {
        Self::new()
    }
}

// ──────────────────────────────────────────────
// 6. DissolveRule
// ──────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DissolveRule {
    pub condition: String,
    pub trigger_threshold: f64,
    pub preserve: Vec<String>,
    pub new_potential: Vec<String>,
}

impl DissolveRule {
    /// Returns true if the adaptation score is <= the trigger threshold,
    /// meaning the room should dissolve.
    pub fn should_dissolve(&self, adaptation_score: f64) -> bool {
        adaptation_score <= self.trigger_threshold
    }
}

// ──────────────────────────────────────────────
// 7. Pre-built artifacts
// ──────────────────────────────────────────────

/// Build the "Old Boat" artifact: 5 components, 3 hidden wisdom entries.
pub fn old_boat() -> Artifact {
    let mut art = Artifact::new(
        "Old Boat",
        "a weathered fishing boat",
        vec![
            "A sleek yacht".to_string(),
            "A floating home".to_string(),
            "A planter for the garden".to_string(),
        ],
    );
    art.components = vec![
        Component::with_wisdom("Hull", 0.3, "The wood remembers the sea — it curves not from decay, but from years of yielding to waves"),
        Component::new("Mast", 0.2),
        Component::with_wisdom("Rudder", 0.1, "A broken rudder steers truer than a new one, if you know how to read its grain"),
        Component::new("Sails", 0.4),
        Component::with_wisdom("Anchor", 0.6, "The anchor was never meant to hold the boat still — it was meant to teach patience"),
    ];
    art
}

/// Build the "Failed Bridge" artifact: 3 components, 2 hidden wisdom entries.
pub fn failed_bridge() -> Artifact {
    let mut art = Artifact::new(
        "Failed Bridge",
        "a collapsed suspension bridge",
        vec![
            "A shorter footbridge".to_string(),
            "A scenic overlook".to_string(),
            "A cable-stayed bridge".to_string(),
        ],
    );
    art.components = vec![
        Component::with_wisdom(
            "Suspension Cables",
            0.3,
            "The cables only failed because they were the wrong kind of strong — rigid instead of flexible",
        ),
        Component::new("Deck", 0.1),
        Component::with_wisdom(
            "Pillars",
            0.5,
            "The pillars stand crooked but unbroken — they learned to lean with the earth",
        ),
    ];
    art
}

/// Build the "Crashed Agent" artifact: 4 components, 3 hidden wisdom entries.
pub fn crashed_agent() -> Artifact {
    let mut art = Artifact::new(
        "Crashed Agent",
        "a corrupted AI agent",
        vec![
            "A wiser assistant".to_string(),
            "A creative collaborator".to_string(),
            "A distributed swarm node".to_string(),
        ],
    );
    art.components = vec![
        Component::with_wisdom(
            "Memory Core",
            0.2,
            "The crash wiped the cache but preserved the weights — forgetting made room for insight",
        ),
        Component::with_wisdom(
            "Decision Engine",
            0.4,
            "The heuristic that caused the crash was the same one that found the elegant solution",
        ),
        Component::new("Communication Bus", 0.6),
        Component::with_wisdom(
            "Ethical Guardrail",
            0.3,
            "The guardrail bent under pressure, but taught the agent that rigidity is not the same as safety",
        ),
    ];
    art
}

#[cfg(test)]
mod tests {
    use super::*;

    // ──────────────────────────────────────
    // TransformId
    // ──────────────────────────────────────

    #[test]
    fn test_transform_id_new() {
        let id = TransformId("plato-room".to_string());
        assert_eq!(id.0, "plato-room");
    }

    #[test]
    fn test_transform_id_clone_eq_hash() {
        let a = TransformId("x".to_string());
        let b = TransformId("x".to_string());
        let c = TransformId("y".to_string());
        assert_eq!(a, b);
        assert_ne!(a, c);
        let cloned = a.clone();
        assert_eq!(a, cloned);
    }

    #[test]
    fn test_transform_id_display() {
        let id = TransformId("hello".to_string());
        assert_eq!(format!("{id}"), "hello");
    }

    // ──────────────────────────────────────
    // TransformPhase
    // ──────────────────────────────────────

    #[test]
    fn test_transform_phase_display() {
        assert_eq!(format!("{}", TransformPhase::Whole), "Whole");
        assert_eq!(format!("{}", TransformPhase::Examining), "Examining");
        assert_eq!(format!("{}", TransformPhase::Scattered), "Scattered");
        assert_eq!(format!("{}", TransformPhase::Understanding), "Understanding");
        assert_eq!(format!("{}", TransformPhase::Rebuilding), "Rebuilding");
        assert_eq!(format!("{}", TransformPhase::Transformed), "Transformed");
    }

    // ──────────────────────────────────────
    // Artifact lifecycle
    // ──────────────────────────────────────

    #[test]
    fn test_artifact_new_is_whole() {
        let art = old_boat();
        assert_eq!(art.phase, TransformPhase::Whole);
        assert!(!art.is_transforming());
    }

    #[test]
    fn test_artifact_deconstruct() {
        let mut art = old_boat();
        art.deconstruct(42);
        assert_eq!(art.phase, TransformPhase::Examining);
        assert_eq!(art.destruction_tick, Some(42));
        assert!(art.is_transforming());
    }

    #[test]
    fn test_examine_component_finds_wisdom() {
        let mut art = old_boat();
        art.deconstruct(1);

        // First component (Hull) has wisdom
        let result = art.examine_component(0);
        assert!(result.is_some());
        assert!(result.unwrap().contains("wood remembers"));
        assert_eq!(art.discovery_count(), 1);

        // Second component (Mast) has no wisdom
        let result = art.examine_component(1);
        assert!(result.is_none());
        assert_eq!(art.discovery_count(), 1);
    }

    #[test]
    fn test_examine_component_out_of_bounds() {
        let mut art = old_boat();
        art.deconstruct(1);
        let result = art.examine_component(999);
        assert!(result.is_none());
    }

    #[test]
    fn test_examine_component_wisdom_only_once() {
        let mut art = old_boat();
        art.deconstruct(1);

        // First call gets wisdom
        let first = art.examine_component(0);
        assert!(first.is_some());

        // Second call should get nothing — wisdom was consumed
        let second = art.examine_component(0);
        assert!(second.is_none());
    }

    #[test]
    fn test_scatter_phase() {
        let mut art = old_boat();
        art.deconstruct(1);
        art.scatter();
        assert_eq!(art.phase, TransformPhase::Scattered);
    }

    #[test]
    fn test_understand_phase() {
        let mut art = old_boat();
        art.deconstruct(1);
        art.scatter();
        art.understand();
        assert_eq!(art.phase, TransformPhase::Understanding);
    }

    #[test]
    fn test_rebuild_success() {
        let mut art = failed_bridge();
        art.deconstruct(1);
        // Examine enough components (need > 3/2 = 1, so >1 = at least 2)
        art.examine_component(0); // wisdom found
        art.examine_component(2); // wisdom found
        assert_eq!(art.discovery_count(), 2);

        let success = art.rebuild("A shorter footbridge", 99);
        assert!(success);
        assert_eq!(art.phase, TransformPhase::Rebuilding);
        assert_eq!(art.rebuild_tick, Some(99));
    }

    #[test]
    fn test_rebuild_failure_not_enough_discoveries() {
        let mut art = failed_bridge();
        art.deconstruct(1);
        // Only 1 discovery, need > 1 (3/2 = 1)
        art.examine_component(0);
        assert_eq!(art.discovery_count(), 1);

        let success = art.rebuild("A shorter footbridge", 99);
        assert!(!success);
        assert_eq!(art.phase, TransformPhase::Examining);
    }

    #[test]
    fn test_complete_transformation() {
        let mut art = crashed_agent();
        art.deconstruct(1);
        art.scatter();
        art.understand();
        art.examine_component(0);
        art.examine_component(1);
        art.examine_component(3);
        art.rebuild("A wiser assistant", 50);
        art.complete();
        assert_eq!(art.phase, TransformPhase::Transformed);
        assert!(!art.is_transforming());
    }

    #[test]
    fn test_hidden_count() {
        let mut art = old_boat();
        assert_eq!(art.hidden_count(), 3);
        art.examine_component(0);
        assert_eq!(art.hidden_count(), 2);
    }

    #[test]
    fn test_discovery_count() {
        let mut art = crashed_agent();
        assert_eq!(art.discovery_count(), 0);
        art.examine_component(0);
        assert_eq!(art.discovery_count(), 1);
    }

    #[test]
    fn test_full_old_boat_lifecycle() {
        let mut art = old_boat();
        assert_eq!(art.components.len(), 5);

        art.deconstruct(10);
        assert_eq!(art.phase, TransformPhase::Examining);

        art.examine_component(0); // Hull wisdom
        art.examine_component(1); // Mast — no wisdom
        art.examine_component(2); // Rudder wisdom
        art.examine_component(4); // Anchor wisdom
        assert_eq!(art.discovery_count(), 3);

        art.scatter();
        assert_eq!(art.phase, TransformPhase::Scattered);

        art.understand();
        assert_eq!(art.phase, TransformPhase::Understanding);

        // discoveries (3) > components/2 (2.5 -> 2) = 3 > 2 -> true
        let success = art.rebuild("A sleek yacht", 20);
        assert!(success);
        assert_eq!(art.phase, TransformPhase::Rebuilding);

        art.complete();
        assert_eq!(art.phase, TransformPhase::Transformed);
    }

    // ──────────────────────────────────────
    // TransformLog
    // ──────────────────────────────────────

    #[test]
    fn test_transform_log_create_and_get() {
        let mut log = TransformLog::new();
        log.create(
            "My Artifact",
            vec![Component::new("Part A", 0.8)],
            vec!["New Form".to_string()],
        );
        let art = log.get("My Artifact").unwrap();
        assert_eq!(art.phase, TransformPhase::Whole);
        assert_eq!(art.components.len(), 1);
    }

    #[test]
    fn test_transform_log_deconstruction_event() {
        let mut log = TransformLog::new();
        log.create(
            "Test",
            vec![Component::new("A", 1.0)],
            vec!["B".to_string()],
        );
        assert_eq!(log.tick, 0);

        log.deconstruct("Test");
        assert_eq!(log.tick, 1);
        assert_eq!(log.events.len(), 1);
        assert!(matches!(
            log.events[0],
            TransformEvent::DeconstructionStarted(ref n) if n == "Test"
        ));
    }

    #[test]
    fn test_transform_log_examine() {
        let mut log = TransformLog::new();
        log.create(
            "Test",
            vec![Component::with_wisdom("A", 0.5, "secret wisdom")],
            vec!["New".to_string()],
        );
        log.deconstruct("Test");

        let discovery = log.examine("Test", 0);
        assert!(discovery.is_some());
        assert_eq!(discovery.unwrap(), "secret wisdom");

        // Should have ComponentExamined + WisdomFound events
        assert_eq!(log.events.len(), 3); // DeconstructionStarted + ComponentExamined + WisdomFound
    }

    #[test]
    fn test_transform_log_rebuild_and_complete() {
        let mut log = TransformLog::new();
        log.create(
            "Bridge",
            vec![
                Component::with_wisdom("A", 0.5, "wisdom-A"),
                Component::with_wisdom("B", 0.5, "wisdom-B"),
                Component::new("C", 0.3),
            ],
            vec!["New Bridge".to_string()],
        );

        assert_eq!(log.total_discoveries(), 0);

        log.deconstruct("Bridge");
        log.examine("Bridge", 0);
        log.examine("Bridge", 1);
        assert_eq!(log.total_discoveries(), 2);

        assert_eq!(log.hidden_remaining(), 0);

        let success = log.rebuild("Bridge", "New Bridge");
        assert!(success);

        log.complete("Bridge");
        assert_eq!(log.completed().len(), 1);

        assert_eq!(log.transform_rate(), 1.0);
    }

    #[test]
    fn test_transform_log_transforming() {
        let mut log = TransformLog::new();
        log.create(
            "A",
            vec![Component::new("x", 1.0)],
            vec!["B".to_string()],
        );
        log.create(
            "C",
            vec![Component::new("y", 1.0)],
            vec!["D".to_string()],
        );
        log.create(
            "E",
            vec![Component::new("z", 1.0)],
            vec!["F".to_string()],
        );

        assert_eq!(log.transforming().len(), 0);

        log.deconstruct("A");
        assert_eq!(log.transforming().len(), 1);

        log.deconstruct("C");
        assert_eq!(log.transforming().len(), 2);
    }

    #[test]
    fn test_transform_log_events_for() {
        let mut log = TransformLog::new();
        log.create(
            "Boat",
            vec![Component::with_wisdom("Hull", 0.5, "deep truth")],
            vec!["Yacht".to_string()],
        );

        log.deconstruct("Boat");
        log.examine("Boat", 0);

        let events = log.events_for("Boat");
        assert_eq!(events.len(), 3);
    }

    #[test]
    fn test_transform_rate_empty() {
        let log = TransformLog::new();
        assert_eq!(log.transform_rate(), 0.0);
    }

    #[test]
    fn test_transform_rate_partial() {
        let mut log = TransformLog::new();
        log.create(
            "A",
            vec![Component::new("x", 1.0)],
            vec!["B".to_string()],
        );
        log.create(
            "C",
            vec![Component::new("y", 1.0)],
            vec!["D".to_string()],
        );

        log.create(
            "E",
            vec![Component::new("z", 1.0)],
            vec!["F".to_string()],
        );

        // Complete 1 of 3
        log.artifacts.get_mut("A").unwrap().phase = TransformPhase::Transformed;
        assert!((log.transform_rate() - 1.0 / 3.0).abs() < 1e-10);
    }

    // ──────────────────────────────────────
    // DissolveRule
    // ──────────────────────────────────────

    #[test]
    fn test_dissolve_rule_should_dissolve() {
        let rule = DissolveRule {
            condition: "over-adapted".to_string(),
            trigger_threshold: 0.3,
            preserve: vec!["layout".to_string(), "theme".to_string()],
            new_potential: vec!["garden".to_string(), "library".to_string()],
        };

        // Score below threshold → dissolve
        assert!(rule.should_dissolve(0.2));
        // Score at threshold → dissolve
        assert!(rule.should_dissolve(0.3));
        // Score above threshold → don't dissolve
        assert!(!rule.should_dissolve(0.4));
    }

    // ──────────────────────────────────────
    // Pre-built artifacts
    // ──────────────────────────────────────

    #[test]
    fn test_old_boat_prebuilt() {
        let art = old_boat();
        assert_eq!(art.name, "Old Boat");
        assert_eq!(art.components.len(), 5);
        assert_eq!(art.hidden_count(), 3);
    }

    #[test]
    fn test_failed_bridge_prebuilt() {
        let art = failed_bridge();
        assert_eq!(art.name, "Failed Bridge");
        assert_eq!(art.components.len(), 3);
        assert_eq!(art.hidden_count(), 2);
    }

    #[test]
    fn test_crashed_agent_prebuilt() {
        let art = crashed_agent();
        assert_eq!(art.name, "Crashed Agent");
        assert_eq!(art.components.len(), 4);
        assert_eq!(art.hidden_count(), 3);
    }

    // ──────────────────────────────────────
    // Serde round-trip
    // ──────────────────────────────────────

    #[test]
    fn test_serde_component_roundtrip() {
        let comp = Component::with_wisdom("Test", 0.5, "wisdom");
        let json = serde_json::to_string(&comp).unwrap();
        let deserialized: Component = serde_json::from_str(&json).unwrap();
        assert_eq!(comp.name, deserialized.name);
        assert_eq!(comp.hidden_wisdom, deserialized.hidden_wisdom);
    }

    #[test]
    fn test_serde_artifact_roundtrip() {
        let art = old_boat();
        let json = serde_json::to_string(&art).unwrap();
        let deserialized: Artifact = serde_json::from_str(&json).unwrap();
        assert_eq!(art.name, deserialized.name);
        assert_eq!(art.components.len(), deserialized.components.len());
        assert_eq!(art.phase, deserialized.phase);
    }

    #[test]
    fn test_serde_transform_log_roundtrip() {
        let mut log = TransformLog::new();
        log.create(
            "Test",
            vec![Component::with_wisdom("A", 0.5, "secret")],
            vec!["New".to_string()],
        );
        log.deconstruct("Test");
        log.examine("Test", 0);
        log.scatter("Test");

        let json = serde_json::to_string(&log).unwrap();
        let deserialized: TransformLog = serde_json::from_str(&json).unwrap();

        assert_eq!(log.artifacts.len(), deserialized.artifacts.len());
        assert_eq!(log.events.len(), deserialized.events.len());
        assert_eq!(log.tick, deserialized.tick);
    }

    #[test]
    fn test_serde_transform_event_roundtrip() {
        let events = vec![
            TransformEvent::DeconstructionStarted("boat".to_string()),
            TransformEvent::ComponentExamined {
                artifact: "boat".to_string(),
                component: "Hull".to_string(),
                discovery: Some("wisdom".to_string()),
            },
            TransformEvent::WisdomFound {
                artifact: "boat".to_string(),
                wisdom: "deep truth".to_string(),
            },
            TransformEvent::RebuildAttempted {
                artifact: "boat".to_string(),
                success: true,
            },
            TransformEvent::TransformationComplete {
                artifact: "boat".to_string(),
                old_form: "old".to_string(),
                new_form: "new".to_string(),
            },
            TransformEvent::RoomDissolved {
                room: "kitchen".to_string(),
                reason: "over-adapted".to_string(),
            },
        ];
        for event in events {
            let json = serde_json::to_string(&event).unwrap();
            let deserialized: TransformEvent = serde_json::from_str(&json).unwrap();
            // Verify by re-serializing and comparing
            let json2 = serde_json::to_string(&deserialized).unwrap();
            assert_eq!(json, json2);
        }
    }
}
