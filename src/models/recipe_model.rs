use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ingredient {
    pub name: String,
    pub quantity: f32,
    pub unit: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Recipes {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub ingredients: Vec<Ingredient>,
    pub instructions: Vec<String>,
}

impl Default for Recipes {
    fn default() -> Self {
        Self {
            id: None,
            name: String::from("Pancakes"),
            ingredients: vec![
                Ingredient {
                    name: String::from("all-purpose flour"),
                    quantity: 2.0,
                    unit: Some(String::from("cup")),
                },
                Ingredient {
                    name: String::from("granulated sugar"),
                    quantity: 2.0,
                    unit: Some(String::from("tablespoons")),
                },
                Ingredient {
                    name: String::from("baking powder"),
                    quantity: 2.0,
                    unit: Some(String::from("teaspoons")),
                },
                Ingredient {
                    name: String::from("salt"),
                    quantity: 0.5,
                    unit: Some(String::from("teaspoon")),
                },
                Ingredient {
                    name: String::from("large eggs"),
                    quantity: 2.0,
                    unit: None,
                },
                Ingredient {
                    name: String::from("milk"),
                    quantity: 1.5,
                    unit: Some(String::from("cups")),
                },
                Ingredient {
                    name: String::from("unsalted butter, melted"),
                    quantity: 0.25,
                    unit: Some(String::from("cup")),
                },
            ],
            instructions: vec![
                String::from("In a large bowl, whisk together the flour, sugar, baking powder, and salt."),
                String::from("In another bowl, whisk together the eggs, milk, and melted butter."),
                String::from("Pour the wet ingredients into the dry ingredients and stir until just combined. Do not overmix."),
                String::from("Heat a non-stick skillet over medium heat. Pour 1/4 cup of batter for each pancake."),
                String::from("Cook until bubbles form on the surface, then flip and cook until golden brown."),
            ],
        }
    }
}
