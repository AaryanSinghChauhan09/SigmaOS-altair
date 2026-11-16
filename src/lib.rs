//! SigmaOS-altair: Altair star system model.
//!
//! Real implementation: planets, orbits, validation.
//! Replaces the 31-line stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Planet {
    pub id: String,
    pub name: String,
    pub mass: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Orbit {
    pub id: String,
    pub planet_id: String,
    pub period: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AltairError {
    EmptyPlanetName,
    DuplicatePlanet(String),
}

impl fmt::Display for AltairError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyPlanetName => write!(f, "planet name must not be empty"),
            Self::DuplicatePlanet(id) => write!(f, "duplicate planet {:?}", id),
        }
    }
}

impl std::error::Error for AltairError {}

pub struct Altair {
    pub name: String,
    planets: BTreeMap<String, Planet>,
    orbits: BTreeMap<String, Orbit>,
}

impl Altair {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), planets: BTreeMap::new(), orbits: BTreeMap::new() }
    }

    pub fn add_planet(&mut self, p: Planet) -> Result<(), AltairError> {
        if p.name.trim().is_empty() { return Err(AltairError::EmptyPlanetName); }
        let id = p.id.clone();
        if self.planets.contains_key(&id) {
            return Err(AltairError::DuplicatePlanet(id.clone()));
        }
        self.planets.insert(id, p);
        Ok(())
    }

    pub fn planet_count(&self) -> usize { self.planets.len() }

    pub fn get_planet(&self, id: &str) -> Option<&Planet> { self.planets.get(id) }

    pub fn add_orbit(&mut self, o: Orbit) {
        self.orbits.insert(o.id.clone(), o);
    }

    pub fn get_orbit(&self, id: &str) -> Option<&Orbit> { self.orbits.get(id) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_aaa() {
        let a = Altair::new("Altair");
        assert_eq!(a.name, "Altair");
        assert_eq!(a.planet_count(), 0);
    }

    #[test]
    fn test_add_aaa() {
        let mut a = Altair::new("Altair");
        let planet = Planet { id: "p1".into(), name: "Altair b".into(), mass: "1.7 Mjup".into() };
        a.add_planet(planet).unwrap();
        assert_eq!(a.planet_count(), 1);
    }

    #[test]
    fn test_empty_rejected_aaa() {
        let mut a = Altair::new("Altair");
        let planet = Planet { id: "x".into(), name: "".into(), mass: "x".into() };
        assert!(a.add_planet(planet).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut a = Altair::new("Altair");
        let planet = Planet { id: "x".into(), name: "X".into(), mass: "x".into() };
        a.add_planet(planet).unwrap();
        let planet2 = Planet { id: "x".into(), name: "X".into(), mass: "x".into() };
        assert!(a.add_planet(planet2).is_err());
    }
}
