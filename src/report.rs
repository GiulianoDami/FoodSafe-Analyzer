use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AnalysisReport {
    pub food_item: String,
    pub cooking_method: String,
    pub risk_level: String,
    pub potential_compounds: Vec<String>,
    pub recommendations: Vec<String>,
}

impl AnalysisReport {
    pub fn new(
        food_item: String,
        cooking_method: String,
        risk_level: String,
        potential_compounds: Vec<String>,
        recommendations: Vec<String>,
    ) -> Self {
        AnalysisReport {
            food_item,
            cooking_method,
            risk_level,
            potential_compounds,
            recommendations,
        }
    }
}

pub fn generate_report(report: &AnalysisReport) -> String {
    format!(
        "FoodSafe Analyzer Report\n\
         =======================\n\
         Food Item: {}\n\
         Cooking Method: {}\n\
         Risk Level: {}\n\
         \n\
         Potential Compounds:\n\
         {}\n\
         \n\
         Recommendations:\n\
         {}\n",
        report.food_item,
        report.cooking_method,
        report.risk_level,
        report.potential_compounds.join("\n  - "),
        report.recommendations.join("\n  - ")
    )
}