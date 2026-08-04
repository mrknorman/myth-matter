//! Versioned semantic material identity (TOP6C slice C-1).
//!
//! `MaterialSemanticKeyV1` captures every recipe input that changes semantic
//! material meaning: the discrete taxonomy ids, the parameter-kind
//! discriminant with its nested discrete ids, and every continuous parameter
//! lane quantized onto a fixed versioned grid. Derived appearance is
//! excluded: the representative color and the derived common properties are
//! outputs of the recipe parameters (plus world/rarity context), so they are
//! never identity inputs.
//!
//! Deterministic per-coordinate variation must never enter this key
//! (docs/material_presentation_redesign.md section 3.1): the same recipe at
//! two world positions is one material, otherwise ordinary noise would create
//! one material per face and defeat greedy meshing and palette reuse.
//!
//! The quantization grid is versioned by
//! [`MATERIAL_SEMANTIC_KEY_PROFILE_V1`]: retuning the step count creates a
//! new profile string and therefore a new key space, never silent drift
//! inside an existing one.

use sha2::{Digest, Sha256};

use crate::{
    MaterialClass, MaterialIceParameter, MaterialOpticalClass, MaterialOrientationAxis,
    MaterialRecipe, MaterialRecipeParameters, MaterialSoilParameter, MaterialStoneParameter,
    MaterialVariant, MaterialWaterParameter,
};

/// Versioned profile of the semantic-key canonical encoding and its
/// quantization grid. Any change to the encoded field set, field order, or
/// [`MATERIAL_PARAMETER_QUANTIZATION_STEPS_V1`] requires a new profile string.
pub const MATERIAL_SEMANTIC_KEY_PROFILE_V1: &str = "myth.material-semantic-key.v1";

/// Number of quantization steps for every continuous `[0, 1]` parameter lane.
/// Frozen by [`MATERIAL_SEMANTIC_KEY_PROFILE_V1`].
///
/// Deliberately fine (8-bit lanes) so material transitions read as
/// continuous while remaining discrete (owner refinement, 2026-08-04).
/// The initial 64-step draft was redefined to 256 before any artifact,
/// palette, or persisted identity existed under this profile; the
/// redefinition is recorded in docs/material_presentation_redesign.md
/// section 8 (MAT-1Q).
pub const MATERIAL_PARAMETER_QUANTIZATION_STEPS_V1: u32 = 256;

/// Sealed 32-byte sha256 content key over the canonical encoding of a
/// [`MaterialSemanticKeyV1`]. Equal keys mean semantically identical
/// materials under the current profile.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MaterialContentKeyV1([u8; 32]);

impl MaterialContentKeyV1 {
    /// Reconstructs a key from persisted bytes. The bytes must previously
    /// have been produced by [`MaterialSemanticKeyV1::content_key`]; this
    /// exists only so persisted palette mappings can be reloaded.
    pub const fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    pub fn to_hex(&self) -> String {
        let mut hex = String::with_capacity(64);
        for byte in self.0 {
            hex.push(char::from_digit((byte >> 4) as u32, 16).expect("nibble is below 16"));
            hex.push(char::from_digit((byte & 0x0f) as u32, 16).expect("nibble is below 16"));
        }
        hex
    }
}

/// Discrete identity and quantized parameter lanes of one recipe kind.
/// Lane order is frozen to the accessor enumeration order of the source
/// parameter enums (`COMPOSITION` then `MODIFIERS` for soil, `COMMON` then
/// `SPECIFIC` for stone, `ALL` for ice and water).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MaterialSemanticParametersV1 {
    None,
    Soil {
        regolith_origin_id: u32,
        /// Element binding ids in frozen order: sand, silt, clay, gravel,
        /// pebble, tephra, organic, iron oxide.
        element_binding_ids: [u32; 8],
        /// Quantized lanes in `MaterialSoilParameter` order: seven
        /// composition lanes then the water and iron-oxide modifiers.
        lanes: [u8; 9],
    },
    Stone {
        genesis_id: u32,
        lithology_id: u32,
        /// Quantized lanes in `MaterialStoneParameter` order: the four
        /// common lanes then the four genesis-specific lanes.
        lanes: [u8; 8],
    },
    Ice {
        impurity_element_id: u32,
        /// Quantized lanes in `MaterialIceParameter::ALL` order.
        lanes: [u8; 3],
    },
    Water {
        /// Quantized lanes in `MaterialWaterParameter::ALL` order. The max
        /// path length lane is the normalized `[0, 1]` encoding, not metres.
        lanes: [u8; 6],
    },
}

impl MaterialSemanticParametersV1 {
    /// Stable discriminant of the parameter kind. Frozen; never derived from
    /// declaration order.
    pub const fn kind_id(&self) -> u32 {
        match self {
            Self::None => 0,
            Self::Soil { .. } => 1,
            Self::Stone { .. } => 2,
            Self::Ice { .. } => 3,
            Self::Water { .. } => 4,
        }
    }
}

/// Discrete semantic identity of one [`MaterialRecipe`] plus its quantized
/// parameter lanes. Two recipes with the same semantic key are the same
/// material for palette and presentation purposes; representative color and
/// derived common properties never contribute.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MaterialSemanticKeyV1 {
    pub class_id: u32,
    pub variant_id: u32,
    pub orientation_id: u32,
    pub optical_class_id: u32,
    pub parameters: MaterialSemanticParametersV1,
}

impl MaterialSemanticKeyV1 {
    /// Captures the semantic identity of `recipe`. Discrete fields use the
    /// frozen `encode_id` tables; every continuous lane is quantized with
    /// [`material_quantize_unit_lane`].
    pub fn from_recipe(recipe: &MaterialRecipe) -> Self {
        Self {
            class_id: recipe.class.encode_id(),
            variant_id: recipe.variant.encode_id(),
            orientation_id: recipe.orientation.encode_id(),
            optical_class_id: recipe.optical_class.encode_id(),
            parameters: semantic_parameters_from_recipe(recipe),
        }
    }

    /// Sealed content key over the deterministic field-tagged canonical byte
    /// encoding of this semantic key, profile string first.
    pub fn content_key(&self) -> MaterialContentKeyV1 {
        let mut canonical = CanonicalEncoder::new(MATERIAL_SEMANTIC_KEY_PROFILE_V1);
        canonical.u32("class", self.class_id);
        canonical.u32("variant", self.variant_id);
        canonical.u32("orientation", self.orientation_id);
        canonical.u32("optical_class", self.optical_class_id);
        canonical.u32("parameter_kind", self.parameters.kind_id());
        match self.parameters {
            MaterialSemanticParametersV1::None => {}
            MaterialSemanticParametersV1::Soil {
                regolith_origin_id,
                element_binding_ids,
                lanes,
            } => {
                canonical.u32("regolith_origin", regolith_origin_id);
                for (name, id) in SOIL_ELEMENT_BINDING_FIELDS.iter().zip(element_binding_ids) {
                    canonical.u32(name, id);
                }
                for (name, lane) in SOIL_LANE_FIELDS.iter().zip(lanes) {
                    canonical.u8(name, lane);
                }
            }
            MaterialSemanticParametersV1::Stone {
                genesis_id,
                lithology_id,
                lanes,
            } => {
                canonical.u32("genesis", genesis_id);
                canonical.u32("lithology", lithology_id);
                for (name, lane) in STONE_LANE_FIELDS.iter().zip(lanes) {
                    canonical.u8(name, lane);
                }
            }
            MaterialSemanticParametersV1::Ice {
                impurity_element_id,
                lanes,
            } => {
                canonical.u32("impurity_element", impurity_element_id);
                for (name, lane) in ICE_LANE_FIELDS.iter().zip(lanes) {
                    canonical.u8(name, lane);
                }
            }
            MaterialSemanticParametersV1::Water { lanes } => {
                for (name, lane) in WATER_LANE_FIELDS.iter().zip(lanes) {
                    canonical.u8(name, lane);
                }
            }
        }
        MaterialContentKeyV1(canonical.sha256())
    }
}

/// Quantizes one clamped `[0, 1]` parameter lane onto the frozen
/// 64-step grid. Values inside the same step are the same material.
pub fn material_quantize_unit_lane(value: f32) -> u8 {
    let step_max = (MATERIAL_PARAMETER_QUANTIZATION_STEPS_V1 - 1) as f32;
    (value.clamp(0.0, 1.0) * step_max).round() as u8
}

/// Canonical on-grid representative of one `[0, 1]` parameter lane: the
/// dequantized [`material_quantize_unit_lane`] value.
///
/// MAT-1Q (docs/material_presentation_redesign.md section 8) makes canonical
/// derivation emit lanes already snapped through this function, so the C-1
/// semantic-key quantization is an assertion, never a transformation:
/// `material_quantize_unit_lane(material_snap_unit_lane(v)) ==
/// material_quantize_unit_lane(v)` for every `v`, and snapping an already
/// snapped value is the identity.
pub fn material_snap_unit_lane(value: f32) -> f32 {
    let step_max = (MATERIAL_PARAMETER_QUANTIZATION_STEPS_V1 - 1) as f32;
    material_quantize_unit_lane(value) as f32 / step_max
}

/// Canonical field name per soil element binding, aligned with
/// `MaterialSemanticParametersV1::Soil::element_binding_ids`.
const SOIL_ELEMENT_BINDING_FIELDS: [&str; 8] = [
    "binding_sand",
    "binding_silt",
    "binding_clay",
    "binding_gravel",
    "binding_pebble",
    "binding_tephra",
    "binding_organic",
    "binding_iron_oxide",
];

/// Canonical field name per soil lane, aligned with
/// `MaterialSoilParameter::COMPOSITION` then `MODIFIERS`.
const SOIL_LANE_FIELDS: [&str; 9] = [
    "sand",
    "silt",
    "clay",
    "gravel",
    "pebble",
    "tephra",
    "organic",
    "water",
    "iron_oxide",
];

/// Canonical field name per stone lane, aligned with
/// `MaterialStoneParameter::COMMON` then `SPECIFIC`.
const STONE_LANE_FIELDS: [&str; 8] = [
    "vein_content",
    "fracture_density",
    "oxide_staining",
    "weathering",
    "primary",
    "secondary",
    "tertiary",
    "quaternary",
];

/// Canonical field name per ice lane, aligned with `MaterialIceParameter::ALL`.
const ICE_LANE_FIELDS: [&str; 3] = ["compaction", "air_content", "impurity"];

/// Canonical field name per water lane, aligned with
/// `MaterialWaterParameter::ALL`.
const WATER_LANE_FIELDS: [&str; 6] = [
    "purity",
    "salinity",
    "sediment",
    "organic_tint",
    "aeration",
    "max_path_length",
];

fn semantic_parameters_from_recipe(recipe: &MaterialRecipe) -> MaterialSemanticParametersV1 {
    match recipe.parameters {
        MaterialRecipeParameters::None => MaterialSemanticParametersV1::None,
        MaterialRecipeParameters::Soil {
            params,
            bindings,
            regolith_origin,
        } => {
            let mut lanes = [0u8; 9];
            let soil_parameters = MaterialSoilParameter::COMPOSITION
                .into_iter()
                .chain(MaterialSoilParameter::MODIFIERS);
            for (lane, parameter) in lanes.iter_mut().zip(soil_parameters) {
                *lane = material_quantize_unit_lane(params.value(parameter));
            }
            MaterialSemanticParametersV1::Soil {
                regolith_origin_id: regolith_origin.encode_id(),
                element_binding_ids: [
                    bindings.sand.encode_id(),
                    bindings.silt.encode_id(),
                    bindings.clay.encode_id(),
                    bindings.gravel.encode_id(),
                    bindings.pebble.encode_id(),
                    bindings.tephra.encode_id(),
                    bindings.organic.encode_id(),
                    bindings.iron_oxide.encode_id(),
                ],
                lanes,
            }
        }
        MaterialRecipeParameters::Stone {
            genesis,
            params,
            lithology,
        } => {
            let mut lanes = [0u8; 8];
            let stone_parameters = MaterialStoneParameter::COMMON
                .into_iter()
                .chain(MaterialStoneParameter::SPECIFIC);
            for (lane, parameter) in lanes.iter_mut().zip(stone_parameters) {
                *lane = material_quantize_unit_lane(params.value(parameter));
            }
            MaterialSemanticParametersV1::Stone {
                genesis_id: genesis.encode_id(),
                lithology_id: lithology.encode_id(),
                lanes,
            }
        }
        MaterialRecipeParameters::Ice { params } => {
            let mut lanes = [0u8; 3];
            for (lane, parameter) in lanes.iter_mut().zip(MaterialIceParameter::ALL) {
                *lane = material_quantize_unit_lane(params.value(parameter));
            }
            MaterialSemanticParametersV1::Ice {
                impurity_element_id: params.impurity_element.encode_id(),
                lanes,
            }
        }
        MaterialRecipeParameters::Water { params } => {
            let mut lanes = [0u8; 6];
            for (lane, parameter) in lanes.iter_mut().zip(MaterialWaterParameter::ALL) {
                *lane = material_quantize_unit_lane(params.value(parameter));
            }
            MaterialSemanticParametersV1::Water { lanes }
        }
    }
}

/// Deterministic field-tagged canonical byte encoder: each field is the
/// little-endian field-name length, the field-name bytes, the little-endian
/// value length, and the value bytes, appended in a fixed order with the
/// profile string first. It mirrors the `CanonicalEncoder` pattern of
/// `myth-terrain-protocol` for the Lithology domain; it is implemented
/// locally because `myth-matter` must not depend on `myth-terrain-protocol`.
struct CanonicalEncoder {
    bytes: Vec<u8>,
}

impl CanonicalEncoder {
    fn new(profile: &str) -> Self {
        let mut result = Self { bytes: Vec::new() };
        result.field("profile", profile.as_bytes());
        result
    }

    fn field(&mut self, name: &str, value: &[u8]) {
        self.bytes
            .extend_from_slice(&(name.len() as u32).to_le_bytes());
        self.bytes.extend_from_slice(name.as_bytes());
        self.bytes
            .extend_from_slice(&(value.len() as u64).to_le_bytes());
        self.bytes.extend_from_slice(value);
    }

    fn u8(&mut self, name: &str, value: u8) {
        self.field(name, &value.to_le_bytes());
    }

    fn u32(&mut self, name: &str, value: u32) {
        self.field(name, &value.to_le_bytes());
    }

    fn sha256(self) -> [u8; 32] {
        Sha256::digest(self.bytes).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        MaterialCommonProperties, MaterialElement, MaterialIceParameters, MaterialMatterState,
        MaterialRegolithOrigin, MaterialRepresentativeColor, MaterialSoilElementBindings,
        MaterialSoilParameters, MaterialStoneGenesis, MaterialStoneLithology,
        MaterialStoneParameters, MaterialWaterParameters, material_derive_stone_lithology,
    };

    fn common_properties_fixture() -> MaterialCommonProperties {
        MaterialCommonProperties::new(
            620,
            740,
            880,
            260,
            180,
            460,
            920,
            980,
            380,
            40,
            680,
            180,
            8,
            MaterialMatterState::Solid,
        )
    }

    fn stone_recipe_fixture() -> MaterialRecipe {
        let genesis = MaterialStoneGenesis::Igneous;
        let params = MaterialStoneParameters {
            vein_content: 0.28,
            fracture_density: 0.16,
            oxide_staining: 0.06,
            weathering: 0.10,
            primary: 0.62,
            secondary: 0.24,
            tertiary: 0.18,
            quaternary: 0.82,
        };
        MaterialRecipe {
            class: MaterialClass::Elemental,
            variant: MaterialVariant::Stone,
            orientation: MaterialOrientationAxis::Y,
            optical_class: MaterialOpticalClass::Opaque,
            representative_color: MaterialRepresentativeColor::new(0.56, 0.54, 0.56),
            common_properties: common_properties_fixture(),
            parameters: MaterialRecipeParameters::Stone {
                genesis,
                params,
                lithology: material_derive_stone_lithology(genesis, params),
            },
        }
    }

    fn soil_recipe_fixture() -> MaterialRecipe {
        MaterialRecipe {
            class: MaterialClass::Mixture,
            variant: MaterialVariant::Soil,
            orientation: MaterialOrientationAxis::Y,
            optical_class: MaterialOpticalClass::Opaque,
            representative_color: MaterialRepresentativeColor::new(0.42, 0.32, 0.22),
            common_properties: common_properties_fixture(),
            parameters: MaterialRecipeParameters::Soil {
                params: MaterialSoilParameters::default(),
                bindings: MaterialSoilElementBindings::from_regolith_origin(
                    MaterialRegolithOrigin::Residual,
                ),
                regolith_origin: MaterialRegolithOrigin::Residual,
            },
        }
    }

    fn ice_recipe_fixture() -> MaterialRecipe {
        MaterialRecipe {
            class: MaterialClass::Elemental,
            variant: MaterialVariant::Ice,
            orientation: MaterialOrientationAxis::Y,
            optical_class: MaterialOpticalClass::ThinTransmissive,
            representative_color: MaterialRepresentativeColor::new(0.78, 0.88, 0.94),
            common_properties: common_properties_fixture(),
            parameters: MaterialRecipeParameters::Ice {
                params: MaterialIceParameters::new(0.92, 0.08, 0.02, MaterialElement::Stone),
            },
        }
    }

    fn water_recipe_fixture() -> MaterialRecipe {
        MaterialRecipe {
            class: MaterialClass::Elemental,
            variant: MaterialVariant::Water,
            orientation: MaterialOrientationAxis::Y,
            optical_class: MaterialOpticalClass::ThinTransmissive,
            representative_color: MaterialRepresentativeColor::new(0.56, 0.80, 0.94),
            common_properties: common_properties_fixture(),
            parameters: MaterialRecipeParameters::Water {
                params: MaterialWaterParameters::default(),
            },
        }
    }

    fn with_stone_params(
        mut recipe: MaterialRecipe,
        mutate: impl FnOnce(&mut MaterialStoneParameters),
    ) -> MaterialRecipe {
        match &mut recipe.parameters {
            MaterialRecipeParameters::Stone { params, .. } => mutate(params),
            _ => panic!("expected stone recipe"),
        }
        recipe
    }

    fn content_key(recipe: &MaterialRecipe) -> MaterialContentKeyV1 {
        MaterialSemanticKeyV1::from_recipe(recipe).content_key()
    }

    #[test]
    fn identical_recipes_produce_identical_keys() {
        for fixture in [
            stone_recipe_fixture,
            soil_recipe_fixture,
            ice_recipe_fixture,
            water_recipe_fixture,
        ] {
            let first = content_key(&fixture());
            let second = content_key(&fixture());
            assert_eq!(first, second);
        }
    }

    #[test]
    fn every_discrete_field_change_produces_a_different_key() {
        let baseline = content_key(&stone_recipe_fixture());

        let mut class_changed = stone_recipe_fixture();
        class_changed.class = MaterialClass::Mixture;
        assert_ne!(content_key(&class_changed), baseline);

        let mut variant_changed = stone_recipe_fixture();
        variant_changed.variant = MaterialVariant::Metal;
        assert_ne!(content_key(&variant_changed), baseline);

        let mut orientation_changed = stone_recipe_fixture();
        orientation_changed.orientation = MaterialOrientationAxis::Z;
        assert_ne!(content_key(&orientation_changed), baseline);

        let mut optical_changed = stone_recipe_fixture();
        optical_changed.optical_class = MaterialOpticalClass::SpecularOpaque;
        assert_ne!(content_key(&optical_changed), baseline);

        let mut genesis_changed = stone_recipe_fixture();
        let mut lithology_changed = stone_recipe_fixture();
        match (
            &mut genesis_changed.parameters,
            &mut lithology_changed.parameters,
        ) {
            (
                MaterialRecipeParameters::Stone { genesis, .. },
                MaterialRecipeParameters::Stone { lithology, .. },
            ) => {
                *genesis = MaterialStoneGenesis::Metamorphic;
                *lithology = MaterialStoneLithology::Sandstone;
            }
            _ => panic!("expected stone recipes"),
        }
        assert_ne!(content_key(&genesis_changed), baseline);
        assert_ne!(content_key(&lithology_changed), baseline);

        let soil_baseline = content_key(&soil_recipe_fixture());
        let mut origin_changed = soil_recipe_fixture();
        let mut binding_changed = soil_recipe_fixture();
        match (
            &mut origin_changed.parameters,
            &mut binding_changed.parameters,
        ) {
            (
                MaterialRecipeParameters::Soil {
                    regolith_origin, ..
                },
                MaterialRecipeParameters::Soil { bindings, .. },
            ) => {
                *regolith_origin = MaterialRegolithOrigin::Aeolian;
                bindings.sand = MaterialElement::Carbonate;
            }
            _ => panic!("expected soil recipes"),
        }
        assert_ne!(content_key(&origin_changed), soil_baseline);
        assert_ne!(content_key(&binding_changed), soil_baseline);

        let ice_baseline = content_key(&ice_recipe_fixture());
        let mut impurity_changed = ice_recipe_fixture();
        match &mut impurity_changed.parameters {
            MaterialRecipeParameters::Ice { params } => {
                params.impurity_element = MaterialElement::Tephra;
            }
            _ => panic!("expected ice recipe"),
        }
        assert_ne!(content_key(&impurity_changed), ice_baseline);

        let mut kind_changed = stone_recipe_fixture();
        kind_changed.parameters = MaterialRecipeParameters::None;
        assert_ne!(content_key(&kind_changed), baseline);
    }

    #[test]
    fn parameter_values_within_one_quantization_step_share_a_key() {
        let step_max = (MATERIAL_PARAMETER_QUANTIZATION_STEPS_V1 - 1) as f32;
        let inside_a = with_stone_params(stone_recipe_fixture(), |params| {
            params.weathering = 20.2 / step_max;
        });
        let inside_b = with_stone_params(stone_recipe_fixture(), |params| {
            params.weathering = 20.3 / step_max;
        });
        assert_eq!(content_key(&inside_a), content_key(&inside_b));
    }

    #[test]
    fn parameter_values_across_a_quantization_step_differ_in_key() {
        let step_max = (MATERIAL_PARAMETER_QUANTIZATION_STEPS_V1 - 1) as f32;
        let below = with_stone_params(stone_recipe_fixture(), |params| {
            params.weathering = 20.3 / step_max;
        });
        let above = with_stone_params(stone_recipe_fixture(), |params| {
            params.weathering = 20.6 / step_max;
        });
        assert_ne!(content_key(&below), content_key(&above));
    }

    #[test]
    fn representative_color_is_excluded_from_identity() {
        let baseline = content_key(&stone_recipe_fixture());
        let mut recolored = stone_recipe_fixture();
        recolored.representative_color = MaterialRepresentativeColor::new(0.9, 0.1, 0.1);
        assert_eq!(content_key(&recolored), baseline);
    }

    #[test]
    fn golden_stone_recipe_key_is_pinned() {
        let key = content_key(&stone_recipe_fixture());
        assert_eq!(
            key.to_hex(),
            "eb4d8b6c08daf7c3adcd151902109e98e7776e0bb10a1a21fa1855fc94d04478",
            "canonical encoding drifted; a change here requires a new profile string"
        );
    }

    #[test]
    fn content_key_round_trips_through_bytes() {
        let key = content_key(&water_recipe_fixture());
        assert_eq!(MaterialContentKeyV1::from_bytes(*key.as_bytes()), key);
    }

    /// MAT-1Q: the snap is the canonical on-grid lane representative —
    /// quantization-preserving, idempotent, and the identity on every grid
    /// value.
    #[test]
    fn snap_unit_lane_is_idempotent_and_preserves_quantization() {
        for sample in 0..=1000 {
            let value = sample as f32 / 1000.0;
            let snapped = material_snap_unit_lane(value);
            assert_eq!(
                material_quantize_unit_lane(snapped),
                material_quantize_unit_lane(value),
                "snap changed the quantized step of {value}"
            );
            assert_eq!(
                material_snap_unit_lane(snapped),
                snapped,
                "snap of {value} is not idempotent"
            );
        }
        assert_eq!(material_snap_unit_lane(-0.5), 0.0);
        assert_eq!(material_snap_unit_lane(1.5), 1.0);

        let step_max = (MATERIAL_PARAMETER_QUANTIZATION_STEPS_V1 - 1) as f32;
        for step in 0..MATERIAL_PARAMETER_QUANTIZATION_STEPS_V1 {
            let grid_value = step as f32 / step_max;
            assert_eq!(material_snap_unit_lane(grid_value), grid_value);
        }
    }
}
