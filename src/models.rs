// src/models.rs

use serde::Deserialize;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Deserialize)]
pub enum Category {
    Starter,
    Legendaire,
    Fossile,      // NOUVEAU
    Ethologique,  // NOUVEAU
    Normal,
}

#[derive(Clone, PartialEq, Debug, Deserialize)]
pub enum Type {
    Eau, Poison, Insecte, Plante, Normal, Vol, Electrik, Roche, 
    Sol, Psy, Fee, Combat, Feu, Glace, Spectre, Dragon, Acier, Tenebres
}

#[derive(Clone, PartialEq, Debug, Deserialize)]
pub struct Pokemon {
    pub name: String,
    pub category: Category,
    pub types: Vec<Type>,
    pub locations: Vec<String>,
    pub regions: Vec<String>, // BASCULE EN VECTEUR
}