use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub id: String,
    pub title: String,
    pub description: String,
    pub risk_level: String,
}

pub fn get_safety_recommendations() -> Vec<Recommendation> {
    vec![
        Recommendation {
            id: "1".to_string(),
            title: "Use Lower Cooking Temperatures".to_string(),
            description: "Reduce cooking temperatures to minimize the formation of harmful compounds like PAHs.".to_string(),
            risk_level: "high".to_string(),
        },
        Recommendation {
            id: "2".to_string(),
            title: "Choose Alternative Cooking Methods".to_string(),
            description: "Opt for steaming, boiling, or microwaving instead of grilling or frying.".to_string(),
            risk_level: "medium".to_string(),
        },
        Recommendation {
            id: "3".to_string(),
            title: "Marinate Foods Before Cooking".to_string(),
            description: "Marinating can reduce the formation of harmful compounds by up to 90%.".to_string(),
            risk_level: "low".to_string(),
        },
        Recommendation {
            id: "4".to_string(),
            title: "Avoid Charring".to_string(),
            description: "Prevent burning or charring of foods which increases PAH formation.".to_string(),
            risk_level: "high".to_string(),
        },
        Recommendation {
            id: "5".to_string(),
            title: "Use Smokeless Cooking Techniques".to_string(),
            description: "Choose methods that don't produce smoke to avoid PAH formation from smoke.".to_string(),
            risk_level: "medium".to_string(),
        },
    ]
}