mod secret;

use secret::recover_secret_from_input;
use std::collections::HashMap;

fn main() {
    let input: HashMap<String, HashMap<String, String>> = serde_json::from_str(
        r#"
        {
            "keys": {
                "n": "10",
                "k": "7"
            },
            "1": {
                "base": "6",
                "value": "13444211440455345511"
            },
            "2": {
                "base": "15",
                "value": "aed7015a346d63"
            },
            "3": {
                "base": "15",
                "value": "6aeeb69631c227c"
            },
            "4": {
                "base": "16",
                "value": "e1b5e05623d881f"
            },
            "5": {
                "base": "8",
                "value": "316034514573652620673"
            },
            "6": {
                "base": "3",
                "value": "2122212201122002221120200210011020220200"
            },
            "7": {
                "base": "3",
                "value": "20120221122211000100210021102001201112121"
            },
            "8": {
                "base": "6",
                "value": "20220554335330240002224253"
            },
            "9": {
                "base": "12",
                "value": "45153788322a1255483"
            },
            "10": {
                "base": "7",
                "value": "1101613130313526312514143"
            }
        }
        "#,
    )
    .expect("Invalid JSON input");

    match recover_secret_from_input(&input) {
        Ok(secret) => println!("Recovered Secret: {}", secret),
        Err(e) => println!("Error: {}", e),
    }
}
