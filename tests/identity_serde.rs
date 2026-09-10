use myth_matter::{MaterialSemanticKeyV1, MaterialSemanticParametersV1};
use serde_json::{Value, json};

fn keys() -> Vec<MaterialSemanticKeyV1> {
    [
        MaterialSemanticParametersV1::None,
        MaterialSemanticParametersV1::Soil {
            regolith_origin_id: 0,
            element_binding_ids: [0; 8],
            lanes: [17; 9],
        },
        MaterialSemanticParametersV1::Stone {
            genesis_id: 0,
            lithology_id: 0,
            lanes: [23; 8],
        },
        MaterialSemanticParametersV1::Ice {
            impurity_element_id: 0,
            lanes: [31; 3],
        },
        MaterialSemanticParametersV1::Water { lanes: [47; 6] },
    ]
    .into_iter()
    .map(|parameters| MaterialSemanticKeyV1 {
        class_id: 1,
        variant_id: 2,
        orientation_id: 0,
        optical_class_id: 0,
        parameters,
    })
    .collect()
}

#[test]
fn semantic_json_round_trip_preserves_every_kind_and_content_key() {
    for key in keys() {
        let bytes = serde_json::to_vec(&key).unwrap();
        let decoded: MaterialSemanticKeyV1 = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(decoded, key);
        assert_eq!(decoded.content_key(), key.content_key());
        assert_eq!(serde_json::to_vec(&decoded).unwrap(), bytes);
        let mut missing = serde_json::to_value(key).unwrap();
        missing.as_object_mut().unwrap().remove("orientation_id");
        assert!(serde_json::from_value::<MaterialSemanticKeyV1>(missing).is_err());
        let text = String::from_utf8(bytes).unwrap();
        let duplicate = text.replacen("\"class_id\":", "\"class_id\":1,\"class_id\":", 1);
        assert!(serde_json::from_str::<MaterialSemanticKeyV1>(&duplicate).is_err());
    }
}

#[test]
fn semantic_json_refuses_unknown_fields_at_every_recipe_boundary() {
    for key in keys() {
        let mut outer = serde_json::to_value(key).unwrap();
        outer["unknown_recipe_member"] = json!(0);
        assert!(
            serde_json::from_value::<MaterialSemanticKeyV1>(outer).is_err(),
            "semantic identity must refuse unrecognized members"
        );
        if key.parameters != MaterialSemanticParametersV1::None {
            let mut inner = serde_json::to_value(key).unwrap();
            let parameters = inner["parameters"].as_object_mut().unwrap();
            let payload = parameters.values_mut().next().unwrap();
            payload["unknown_parameter_member"] = Value::Bool(true);
            assert!(
                serde_json::from_value::<MaterialSemanticKeyV1>(inner).is_err(),
                "every semantic parameter variant must refuse unrecognized members"
            );
        }
    }
}
