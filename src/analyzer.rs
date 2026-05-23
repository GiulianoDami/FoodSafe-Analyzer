use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    pub food_type: String,
    pub cooking_method: String,
    pub risk_level: String,
    pub potential_compounds: Vec<String>,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CookingScenario {
    pub food_type: String,
    pub cooking_method: String,
    pub temperature: f64,
    pub duration: u32,
}

impl CookingScenario {
    pub fn new(food_type: String, cooking_method: String, temperature: f64, duration: u32) -> Self {
        Self {
            food_type,
            cooking_method,
            temperature,
            duration,
        }
    }
}

pub fn analyze_cooking_scenario(scenario: &CookingScenario) -> RiskAssessment {
    let mut potential_compounds = Vec::new();
    let mut recommendations = Vec::new();
    let mut risk_level = "low".to_string();

    // Check for high-temperature cooking methods
    if scenario.temperature > 200.0 {
        match scenario.cooking_method.to_lowercase().as_str() {
            "grill" | "roast" | "smoke" | "fry" => {
                potential_compounds.push("Polycyclic Aromatic Hydrocarbons (PAHs)".to_string());
                risk_level = "high".to_string();
            }
            _ => {}
        }
    }

    // Check for prolonged cooking times
    if scenario.duration > 30 {
        potential_compounds.push("Advanced Glycation End-products (AGEs)".to_string());
        risk_level = "medium".to_string();
    }

    // Specific food type considerations
    if scenario.food_type.to_lowercase().contains("meat") {
        if scenario.cooking_method == "grill" || scenario.cooking_method == "fry" {
            recommendations.push("Use marinades with antioxidants like lemon juice or herbs".to_string());
            recommendations.push("Avoid charring or burning the meat".to_string());
        }
    }

    // General recommendations
    if !recommendations.is_empty() {
        recommendations.push("Consider using lower cooking temperatures when possible".to_string());
    } else {
        recommendations.push("Continue with current cooking practices".to_string());
    }

    RiskAssessment {
        food_type: scenario.food_type.clone(),
        cooking_method: scenario.cooking_method.clone(),
        risk_level,
        potential_compounds,
        recommendations,
    }
}