#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

#[doc = r" Error types."]
pub mod error {
    #[doc = r" Error from a `TryFrom` or `FromStr` implementation."]
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
#[doc = "`CalculatedWaterProfile`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"combined\","]
#[doc = "    \"mash\","]
#[doc = "    \"sparge\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"combined\": {"]
#[doc = "      \"$ref\": \"#/components/schemas/WaterProfile\""]
#[doc = "    },"]
#[doc = "    \"mash\": {"]
#[doc = "      \"$ref\": \"#/components/schemas/WaterProfile\""]
#[doc = "    },"]
#[doc = "    \"sparge\": {"]
#[doc = "      \"$ref\": \"#/components/schemas/WaterProfile\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CalculatedWaterProfile {
    pub combined: WaterProfile,
    pub mash: WaterProfile,
    pub sparge: WaterProfile,
}
impl CalculatedWaterProfile {
    pub fn builder() -> builder::CalculatedWaterProfile {
        Default::default()
    }
}
#[doc = "`CreateEquipmentProfileInput`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"batch_size_l\","]
#[doc = "    \"boil_size_l\","]
#[doc = "    \"efficiency_pct\","]
#[doc = "    \"name\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"batch_size_l\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"boil_size_l\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"boil_time_min\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"efficiency_pct\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"evap_rate_pct_hr\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"fermenter_loss_l\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"notes\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"trub_chiller_loss_l\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CreateEquipmentProfileInput {
    pub batch_size_l: f64,
    pub boil_size_l: f64,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub boil_time_min: ::std::option::Option<f64>,
    pub efficiency_pct: f64,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub evap_rate_pct_hr: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub fermenter_loss_l: ::std::option::Option<f64>,
    pub name: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub notes: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub trub_chiller_loss_l: ::std::option::Option<f64>,
}
impl CreateEquipmentProfileInput {
    pub fn builder() -> builder::CreateEquipmentProfileInput {
        Default::default()
    }
}
#[doc = "`CreateFermentableAdditionInput`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"amount_kg\","]
#[doc = "    \"color_lovibond\","]
#[doc = "    \"name\","]
#[doc = "    \"type_\","]
#[doc = "    \"yield_pct\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"add_after_boil\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"amount_kg\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"color_lovibond\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"fermentable_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"type_\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"yield_pct\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CreateFermentableAdditionInput {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub add_after_boil: ::std::option::Option<bool>,
    pub amount_kg: f64,
    pub color_lovibond: f64,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub fermentable_id: ::std::option::Option<::std::string::String>,
    pub name: ::std::string::String,
    pub type_: ::std::string::String,
    pub yield_pct: f64,
}
impl CreateFermentableAdditionInput {
    pub fn builder() -> builder::CreateFermentableAdditionInput {
        Default::default()
    }
}
#[doc = "`CreateHopAdditionInput`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"alpha_pct\","]
#[doc = "    \"amount_kg\","]
#[doc = "    \"name\","]
#[doc = "    \"time_min\","]
#[doc = "    \"use_\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"alpha_pct\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"amount_kg\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"form\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"hop_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"time_min\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"use_\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"hopstand_temp_c\": {"]
#[doc = "      \"description\": \"Whirlpool temperature override for this addition in °C\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CreateHopAdditionInput {
    pub alpha_pct: f64,
    pub amount_kg: f64,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub form: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub hop_id: ::std::option::Option<::std::string::String>,
    pub name: ::std::string::String,
    pub time_min: f64,
    pub use_: ::std::string::String,
    #[doc = "Whirlpool temperature override for this addition in °C"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub hopstand_temp_c: ::std::option::Option<f64>,
}
impl CreateHopAdditionInput {
    pub fn builder() -> builder::CreateHopAdditionInput {
        Default::default()
    }
}
#[doc = "`CreateMashStepInput`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"step_temp_c\","]
#[doc = "    \"step_time_min\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"end_temp_c\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"infuse_amount_l\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"ramp_time_min\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"step_temp_c\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"step_time_min\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"type_\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CreateMashStepInput {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub end_temp_c: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub infuse_amount_l: ::std::option::Option<f64>,
    pub name: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ramp_time_min: ::std::option::Option<i64>,
    pub step_temp_c: f64,
    pub step_time_min: i64,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<::std::string::String>,
}
impl CreateMashStepInput {
    pub fn builder() -> builder::CreateMashStepInput {
        Default::default()
    }
}
#[doc = "`CreateMiscAdditionInput`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"amount\","]
#[doc = "    \"name\","]
#[doc = "    \"time_min\","]
#[doc = "    \"type_\","]
#[doc = "    \"use_\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"amount\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"amount_is_weight\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"misc_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"time_min\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"type_\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"use_\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CreateMiscAdditionInput {
    pub amount: f64,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub amount_is_weight: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub misc_id: ::std::option::Option<::std::string::String>,
    pub name: ::std::string::String,
    pub time_min: f64,
    pub type_: ::std::string::String,
    pub use_: ::std::string::String,
}
impl CreateMiscAdditionInput {
    pub fn builder() -> builder::CreateMiscAdditionInput {
        Default::default()
    }
}
#[doc = "`CreateRecipeInput`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"batch_size_l\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"boil_size_l\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"boil_time_min\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"equipment_profile_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"source_id\": {"]
#[doc = "      \"description\": \"ID of a recipe to copy ingredients from\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"style_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"type_\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"hopstand_temp_c\": {"]
#[doc = "      \"description\": \"Default whirlpool/hopstand temperature in °C\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CreateRecipeInput {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub batch_size_l: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub boil_size_l: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub boil_time_min: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub equipment_profile_id: ::std::option::Option<::std::string::String>,
    pub name: ::std::string::String,
    #[doc = "ID of a recipe to copy ingredients from"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub source_id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub style_id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<::std::string::String>,
    #[doc = "Default whirlpool/hopstand temperature in °C"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub hopstand_temp_c: ::std::option::Option<f64>,
}
impl CreateRecipeInput {
    pub fn builder() -> builder::CreateRecipeInput {
        Default::default()
    }
}
#[doc = "`CreateWaterAdditionInput`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"amount_l\","]
#[doc = "    \"name\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"amount_l\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"water_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CreateWaterAdditionInput {
    pub amount_l: f64,
    pub name: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub water_id: ::std::option::Option<::std::string::String>,
}
impl CreateWaterAdditionInput {
    pub fn builder() -> builder::CreateWaterAdditionInput {
        Default::default()
    }
}
#[doc = "`CreateWaterAdjustmentInput`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"addition\","]
#[doc = "    \"amount\","]
#[doc = "    \"target\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"addition\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"gypsum\","]
#[doc = "        \"calcium_chloride\","]
#[doc = "        \"epsom_salt\","]
#[doc = "        \"table_salt\","]
#[doc = "        \"baking_soda\","]
#[doc = "        \"chalk\","]
#[doc = "        \"lactic_acid\","]
#[doc = "        \"phosphoric_acid\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"amount\": {"]
#[doc = "      \"description\": \"Amount in grams for salts, ml for acids\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"target\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"mash\","]
#[doc = "        \"sparge\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CreateWaterAdjustmentInput {
    pub addition: CreateWaterAdjustmentInputAddition,
    #[doc = "Amount in grams for salts, ml for acids"]
    pub amount: f64,
    pub target: CreateWaterAdjustmentInputTarget,
}
impl CreateWaterAdjustmentInput {
    pub fn builder() -> builder::CreateWaterAdjustmentInput {
        Default::default()
    }
}
#[doc = "`CreateWaterAdjustmentInputAddition`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"gypsum\","]
#[doc = "    \"calcium_chloride\","]
#[doc = "    \"epsom_salt\","]
#[doc = "    \"table_salt\","]
#[doc = "    \"baking_soda\","]
#[doc = "    \"chalk\","]
#[doc = "    \"lactic_acid\","]
#[doc = "    \"phosphoric_acid\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CreateWaterAdjustmentInputAddition {
    #[serde(rename = "gypsum")]
    Gypsum,
    #[serde(rename = "calcium_chloride")]
    CalciumChloride,
    #[serde(rename = "epsom_salt")]
    EpsomSalt,
    #[serde(rename = "table_salt")]
    TableSalt,
    #[serde(rename = "baking_soda")]
    BakingSoda,
    #[serde(rename = "chalk")]
    Chalk,
    #[serde(rename = "lactic_acid")]
    LacticAcid,
    #[serde(rename = "phosphoric_acid")]
    PhosphoricAcid,
}
impl ::std::fmt::Display for CreateWaterAdjustmentInputAddition {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Gypsum => f.write_str("gypsum"),
            Self::CalciumChloride => f.write_str("calcium_chloride"),
            Self::EpsomSalt => f.write_str("epsom_salt"),
            Self::TableSalt => f.write_str("table_salt"),
            Self::BakingSoda => f.write_str("baking_soda"),
            Self::Chalk => f.write_str("chalk"),
            Self::LacticAcid => f.write_str("lactic_acid"),
            Self::PhosphoricAcid => f.write_str("phosphoric_acid"),
        }
    }
}
impl ::std::str::FromStr for CreateWaterAdjustmentInputAddition {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "gypsum" => Ok(Self::Gypsum),
            "calcium_chloride" => Ok(Self::CalciumChloride),
            "epsom_salt" => Ok(Self::EpsomSalt),
            "table_salt" => Ok(Self::TableSalt),
            "baking_soda" => Ok(Self::BakingSoda),
            "chalk" => Ok(Self::Chalk),
            "lactic_acid" => Ok(Self::LacticAcid),
            "phosphoric_acid" => Ok(Self::PhosphoricAcid),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateWaterAdjustmentInputAddition {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CreateWaterAdjustmentInputAddition {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateWaterAdjustmentInputAddition {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`CreateWaterAdjustmentInputTarget`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"mash\","]
#[doc = "    \"sparge\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CreateWaterAdjustmentInputTarget {
    #[serde(rename = "mash")]
    Mash,
    #[serde(rename = "sparge")]
    Sparge,
}
impl ::std::fmt::Display for CreateWaterAdjustmentInputTarget {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Mash => f.write_str("mash"),
            Self::Sparge => f.write_str("sparge"),
        }
    }
}
impl ::std::str::FromStr for CreateWaterAdjustmentInputTarget {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "mash" => Ok(Self::Mash),
            "sparge" => Ok(Self::Sparge),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateWaterAdjustmentInputTarget {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CreateWaterAdjustmentInputTarget {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateWaterAdjustmentInputTarget {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`CreateYeastAdditionInput`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"form\","]
#[doc = "    \"name\","]
#[doc = "    \"type_\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"add_to_secondary\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"amount\": {"]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"amount_is_weight\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"attenuation_pct\": {"]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"form\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"laboratory\": {"]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"product_id\": {"]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"times_cultured\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"type_\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"yeast_id\": {"]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CreateYeastAdditionInput {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub add_to_secondary: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub amount: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub amount_is_weight: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub attenuation_pct: ::std::option::Option<f64>,
    pub form: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub laboratory: ::std::option::Option<::std::string::String>,
    pub name: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub product_id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub times_cultured: ::std::option::Option<i64>,
    pub type_: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub yeast_id: ::std::option::Option<::std::string::String>,
}
impl CreateYeastAdditionInput {
    pub fn builder() -> builder::CreateYeastAdditionInput {
        Default::default()
    }
}
#[doc = "`EquipmentProfile`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"batch_size_l\","]
#[doc = "    \"boil_size_l\","]
#[doc = "    \"boil_time_min\","]
#[doc = "    \"calc_boil_volume\","]
#[doc = "    \"created_at\","]
#[doc = "    \"efficiency_pct\","]
#[doc = "    \"evap_rate_pct_hr\","]
#[doc = "    \"fermenter_loss_l\","]
#[doc = "    \"hop_utilization_pct\","]
#[doc = "    \"id\","]
#[doc = "    \"lauter_deadspace_l\","]
#[doc = "    \"name\","]
#[doc = "    \"top_up_kettle_l\","]
#[doc = "    \"top_up_water_l\","]
#[doc = "    \"trub_chiller_loss_l\","]
#[doc = "    \"updated_at\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"batch_size_l\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"boil_size_l\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"boil_time_min\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"calc_boil_volume\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"created_at\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"efficiency_pct\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"evap_rate_pct_hr\": {"]
#[doc = "      \"description\": \"Boil evaporation rate as a percentage per hour\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"fermenter_loss_l\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"hop_utilization_pct\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"lauter_deadspace_l\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"notes\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"top_up_kettle_l\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"top_up_water_l\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"trub_chiller_loss_l\": {"]
#[doc = "      \"description\": \"Volume lost to trub and chiller in litres\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"tun_specific_heat\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"tun_volume_l\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"tun_weight_kg\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"updated_at\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct EquipmentProfile {
    pub batch_size_l: f64,
    pub boil_size_l: f64,
    pub boil_time_min: f64,
    pub calc_boil_volume: bool,
    pub created_at: i64,
    pub efficiency_pct: f64,
    #[doc = "Boil evaporation rate as a percentage per hour"]
    pub evap_rate_pct_hr: f64,
    pub fermenter_loss_l: f64,
    pub hop_utilization_pct: f64,
    pub id: ::std::string::String,
    pub lauter_deadspace_l: f64,
    pub name: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub notes: ::std::option::Option<::std::string::String>,
    pub top_up_kettle_l: f64,
    pub top_up_water_l: f64,
    #[doc = "Volume lost to trub and chiller in litres"]
    pub trub_chiller_loss_l: f64,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tun_specific_heat: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tun_volume_l: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tun_weight_kg: ::std::option::Option<f64>,
    pub updated_at: i64,
}
impl EquipmentProfile {
    pub fn builder() -> builder::EquipmentProfile {
        Default::default()
    }
}
#[doc = "`Fermentable`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"add_after_boil\","]
#[doc = "    \"color_lovibond\","]
#[doc = "    \"id\","]
#[doc = "    \"name\","]
#[doc = "    \"type_\","]
#[doc = "    \"yield_pct\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"add_after_boil\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"coarse_fine_diff_pct\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"color_lovibond\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"diastatic_power_lintner\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"ibu_gal_per_lb\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"max_in_batch_pct\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"moisture_pct\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"notes\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"origin\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"protein_pct\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"recommend_mash\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"boolean\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"supplier\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"type_\": {"]
#[doc = "      \"description\": \"Grain, Sugar, Extract, Dry Extract, Adjunct\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"yield_pct\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Fermentable {
    pub add_after_boil: bool,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub coarse_fine_diff_pct: ::std::option::Option<f64>,
    pub color_lovibond: f64,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub diastatic_power_lintner: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ibu_gal_per_lb: ::std::option::Option<f64>,
    pub id: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max_in_batch_pct: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub moisture_pct: ::std::option::Option<f64>,
    pub name: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub notes: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub origin: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub protein_pct: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub recommend_mash: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub supplier: ::std::option::Option<::std::string::String>,
    #[doc = "Grain, Sugar, Extract, Dry Extract, Adjunct"]
    pub type_: ::std::string::String,
    pub yield_pct: f64,
}
impl Fermentable {
    pub fn builder() -> builder::Fermentable {
        Default::default()
    }
}
#[doc = "`Hop`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"alpha_pct\","]
#[doc = "    \"form\","]
#[doc = "    \"id\","]
#[doc = "    \"name\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"alpha_pct\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"beta_pct\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"caryophyllene_pct\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"cohumulone_pct\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"form\": {"]
#[doc = "      \"description\": \"Pellet, Plug, Leaf\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"hsi_pct\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"humulene_pct\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"myrcene_pct\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"notes\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"origin\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"substitutes\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"type_\": {"]
#[doc = "      \"description\": \"Bittering, Aroma, Both\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"year\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Hop {
    pub alpha_pct: f64,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub beta_pct: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub caryophyllene_pct: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cohumulone_pct: ::std::option::Option<f64>,
    #[doc = "Pellet, Plug, Leaf"]
    pub form: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub hsi_pct: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub humulene_pct: ::std::option::Option<f64>,
    pub id: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub myrcene_pct: ::std::option::Option<f64>,
    pub name: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub notes: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub origin: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub substitutes: ::std::option::Option<::std::string::String>,
    #[doc = "Bittering, Aroma, Both"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub year: ::std::option::Option<::std::string::String>,
}
impl Hop {
    pub fn builder() -> builder::Hop {
        Default::default()
    }
}
#[doc = "`Mash`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"equip_adjust\","]
#[doc = "    \"grain_temp_c\","]
#[doc = "    \"id\","]
#[doc = "    \"name\","]
#[doc = "    \"recipe_id\","]
#[doc = "    \"steps\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"equip_adjust\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"grain_temp_c\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"notes\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"ph\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"ratio_l_per_kg\": {"]
#[doc = "      \"description\": \"Water-to-grain ratio in litres per kilogram\","]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"recipe_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"sparge_temp_c\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"steps\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/components/schemas/MashStep\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"tun_specific_heat\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"tun_temp_c\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"tun_weight_kg\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Mash {
    pub equip_adjust: bool,
    pub grain_temp_c: f64,
    pub id: ::std::string::String,
    pub name: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub notes: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ph: ::std::option::Option<f64>,
    #[doc = "Water-to-grain ratio in litres per kilogram"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ratio_l_per_kg: ::std::option::Option<f64>,
    pub recipe_id: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sparge_temp_c: ::std::option::Option<f64>,
    pub steps: ::std::vec::Vec<MashStep>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tun_specific_heat: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tun_temp_c: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tun_weight_kg: ::std::option::Option<f64>,
}
impl Mash {
    pub fn builder() -> builder::Mash {
        Default::default()
    }
}
#[doc = "`MashStep`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"mash_id\","]
#[doc = "    \"name\","]
#[doc = "    \"step_order\","]
#[doc = "    \"step_temp_c\","]
#[doc = "    \"step_time_min\","]
#[doc = "    \"type_\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"end_temp_c\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"infuse_amount_l\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"mash_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"ramp_time_min\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"step_order\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"step_temp_c\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"step_time_min\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"type_\": {"]
#[doc = "      \"description\": \"Infusion, Temperature, Decoction\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct MashStep {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub end_temp_c: ::std::option::Option<f64>,
    pub id: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub infuse_amount_l: ::std::option::Option<f64>,
    pub mash_id: ::std::string::String,
    pub name: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ramp_time_min: ::std::option::Option<i64>,
    pub step_order: i64,
    pub step_temp_c: f64,
    pub step_time_min: i64,
    #[doc = "Infusion, Temperature, Decoction"]
    pub type_: ::std::string::String,
}
impl MashStep {
    pub fn builder() -> builder::MashStep {
        Default::default()
    }
}
#[doc = "`Misc`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"amount_is_weight\","]
#[doc = "    \"id\","]
#[doc = "    \"name\","]
#[doc = "    \"time_min\","]
#[doc = "    \"type_\","]
#[doc = "    \"use_\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"amount_is_weight\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"notes\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"time_min\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"type_\": {"]
#[doc = "      \"description\": \"Spice, Fining, Water Agent, Herb, Flavor, Other\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"use_\": {"]
#[doc = "      \"description\": \"Boil, Mash, Primary, Secondary, Bottling\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"use_for\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Misc {
    pub amount_is_weight: bool,
    pub id: ::std::string::String,
    pub name: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub notes: ::std::option::Option<::std::string::String>,
    pub time_min: f64,
    #[doc = "Spice, Fining, Water Agent, Herb, Flavor, Other"]
    pub type_: ::std::string::String,
    #[doc = "Boil, Mash, Primary, Secondary, Bottling"]
    pub use_: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub use_for: ::std::option::Option<::std::string::String>,
}
impl Misc {
    pub fn builder() -> builder::Misc {
        Default::default()
    }
}
#[doc = "`Recipe`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"batch_size_l\","]
#[doc = "    \"boil_size_l\","]
#[doc = "    \"boil_time_min\","]
#[doc = "    \"created_at\","]
#[doc = "    \"fermentables\","]
#[doc = "    \"fermentation_stages\","]
#[doc = "    \"forced_carbonation\","]
#[doc = "    \"hops\","]
#[doc = "    \"id\","]
#[doc = "    \"miscs\","]
#[doc = "    \"name\","]
#[doc = "    \"type_\","]
#[doc = "    \"updated_at\","]
#[doc = "    \"waters\","]
#[doc = "    \"yeasts\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"age_days\": {"]
#[doc = "      \"description\": \"Conditioning/bottle age in days\","]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"age_temp_c\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"asst_brewer\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"batch_size_l\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"boil_size_l\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"boil_time_min\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"brewer\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"carbonation_temp_c\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"carbonation_vols\": {"]
#[doc = "      \"description\": \"Target carbonation in volumes of CO2\","]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"created_at\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"date\": {"]
#[doc = "      \"description\": \"Brew date (free-form string)\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"efficiency_pct\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"equipment_profile\": {"]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/components/schemas/EquipmentProfile\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"equipment_profile_id\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"fermentables\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/components/schemas/RecipeAdditionFermentable\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"fermentation_stages\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"fg\": {"]
#[doc = "      \"description\": \"Measured final gravity\","]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"forced_carbonation\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"hops\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/components/schemas/RecipeAdditionHop\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"keg_priming_factor\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"mash\": {"]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/components/schemas/Mash\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"mash_water_id\": {"]
#[doc = "      \"description\": \"ID of the mash water profile\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"miscs\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/components/schemas/RecipeAdditionMisc\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"notes\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"og\": {"]
#[doc = "      \"description\": \"Measured original gravity\","]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"primary_age_days\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"primary_temp_c\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"priming_sugar_equiv\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"priming_sugar_name\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"secondary_age_days\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"secondary_temp_c\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"sparge_water_id\": {"]
#[doc = "      \"description\": \"ID of the sparge water profile (null means use mash water)\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"style\": {"]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/components/schemas/Style\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"style_id\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"taste_notes\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"taste_rating\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"tertiary_age_days\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"tertiary_temp_c\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"type_\": {"]
#[doc = "      \"description\": \"Recipe type (e.g. All Grain, Extract, Partial Mash)\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"updated_at\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"water_adjustments\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/components/schemas/RecipeWaterAdjustment\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"waters\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/components/schemas/RecipeAdditionWater\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"hopstand_temp_c\": {"]
#[doc = "      \"description\": \"Default whirlpool/hopstand temperature in °C for this recipe\","]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"yeasts\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/components/schemas/RecipeAdditionYeast\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Recipe {
    #[doc = "Conditioning/bottle age in days"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub age_days: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub age_temp_c: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub asst_brewer: ::std::option::Option<::std::string::String>,
    pub batch_size_l: f64,
    pub boil_size_l: f64,
    pub boil_time_min: f64,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub brewer: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub carbonation_temp_c: ::std::option::Option<f64>,
    #[doc = "Target carbonation in volumes of CO2"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub carbonation_vols: ::std::option::Option<f64>,
    pub created_at: i64,
    #[doc = "Brew date (free-form string)"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub date: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub efficiency_pct: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub equipment_profile: ::std::option::Option<EquipmentProfile>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub equipment_profile_id: ::std::option::Option<::std::string::String>,
    pub fermentables: ::std::vec::Vec<RecipeAdditionFermentable>,
    pub fermentation_stages: i64,
    #[doc = "Measured final gravity"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub fg: ::std::option::Option<f64>,
    pub forced_carbonation: bool,
    pub hops: ::std::vec::Vec<RecipeAdditionHop>,
    pub id: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub keg_priming_factor: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub mash: ::std::option::Option<Mash>,
    #[doc = "ID of the mash water profile"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub mash_water_id: ::std::option::Option<::std::string::String>,
    pub miscs: ::std::vec::Vec<RecipeAdditionMisc>,
    pub name: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub notes: ::std::option::Option<::std::string::String>,
    #[doc = "Measured original gravity"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub og: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub primary_age_days: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub primary_temp_c: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub priming_sugar_equiv: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub priming_sugar_name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub secondary_age_days: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub secondary_temp_c: ::std::option::Option<f64>,
    #[doc = "ID of the sparge water profile (null means use mash water)"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sparge_water_id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub style: ::std::option::Option<Style>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub style_id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub taste_notes: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub taste_rating: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tertiary_age_days: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tertiary_temp_c: ::std::option::Option<f64>,
    #[doc = "Recipe type (e.g. All Grain, Extract, Partial Mash)"]
    pub type_: ::std::string::String,
    pub updated_at: i64,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub water_adjustments: ::std::vec::Vec<RecipeWaterAdjustment>,
    pub waters: ::std::vec::Vec<RecipeAdditionWater>,
    #[doc = "Default whirlpool/hopstand temperature in °C for this recipe"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub hopstand_temp_c: ::std::option::Option<f64>,
    pub yeasts: ::std::vec::Vec<RecipeAdditionYeast>,
}
impl Recipe {
    pub fn builder() -> builder::Recipe {
        Default::default()
    }
}
#[doc = "`RecipeAdditionFermentable`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"add_after_boil\","]
#[doc = "    \"addition_order\","]
#[doc = "    \"amount_kg\","]
#[doc = "    \"color_lovibond\","]
#[doc = "    \"id\","]
#[doc = "    \"name\","]
#[doc = "    \"recipe_id\","]
#[doc = "    \"type_\","]
#[doc = "    \"yield_pct\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"add_after_boil\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"addition_order\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"amount_kg\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"color_lovibond\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"fermentable_id\": {"]
#[doc = "      \"description\": \"Source library ingredient ID (null if manually entered)\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"recipe_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"type_\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"yield_pct\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct RecipeAdditionFermentable {
    pub add_after_boil: bool,
    pub addition_order: i64,
    pub amount_kg: f64,
    pub color_lovibond: f64,
    #[doc = "Source library ingredient ID (null if manually entered)"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub fermentable_id: ::std::option::Option<::std::string::String>,
    pub id: ::std::string::String,
    pub name: ::std::string::String,
    pub recipe_id: ::std::string::String,
    pub type_: ::std::string::String,
    pub yield_pct: f64,
}
impl RecipeAdditionFermentable {
    pub fn builder() -> builder::RecipeAdditionFermentable {
        Default::default()
    }
}
#[doc = "`RecipeAdditionHop`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"addition_order\","]
#[doc = "    \"alpha_pct\","]
#[doc = "    \"amount_kg\","]
#[doc = "    \"form\","]
#[doc = "    \"id\","]
#[doc = "    \"name\","]
#[doc = "    \"recipe_id\","]
#[doc = "    \"time_min\","]
#[doc = "    \"use_\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"addition_order\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"alpha_pct\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"amount_kg\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"form\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"hop_id\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"recipe_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"time_min\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"use_\": {"]
#[doc = "      \"description\": \"Boil, Dry Hop, Mash, First Wort, Aroma, Hopstand\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"hopstand_temp_c\": {"]
#[doc = "      \"description\": \"Whirlpool/hopstand temperature in °C for this addition (overrides recipe-level setting)\","]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct RecipeAdditionHop {
    pub addition_order: i64,
    pub alpha_pct: f64,
    pub amount_kg: f64,
    pub form: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub hop_id: ::std::option::Option<::std::string::String>,
    pub id: ::std::string::String,
    pub name: ::std::string::String,
    pub recipe_id: ::std::string::String,
    pub time_min: f64,
    #[doc = "Boil, Dry Hop, Mash, First Wort, Aroma, Hopstand"]
    pub use_: ::std::string::String,
    #[doc = "Whirlpool/hopstand temperature in °C for this addition (overrides recipe-level setting)"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub hopstand_temp_c: ::std::option::Option<f64>,
}
impl RecipeAdditionHop {
    pub fn builder() -> builder::RecipeAdditionHop {
        Default::default()
    }
}
#[doc = "`RecipeAdditionMisc`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"addition_order\","]
#[doc = "    \"amount\","]
#[doc = "    \"amount_is_weight\","]
#[doc = "    \"id\","]
#[doc = "    \"name\","]
#[doc = "    \"recipe_id\","]
#[doc = "    \"time_min\","]
#[doc = "    \"type_\","]
#[doc = "    \"use_\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"addition_order\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"amount\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"amount_is_weight\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"misc_id\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"recipe_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"time_min\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"type_\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"use_\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct RecipeAdditionMisc {
    pub addition_order: i64,
    pub amount: f64,
    pub amount_is_weight: bool,
    pub id: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub misc_id: ::std::option::Option<::std::string::String>,
    pub name: ::std::string::String,
    pub recipe_id: ::std::string::String,
    pub time_min: f64,
    pub type_: ::std::string::String,
    pub use_: ::std::string::String,
}
impl RecipeAdditionMisc {
    pub fn builder() -> builder::RecipeAdditionMisc {
        Default::default()
    }
}
#[doc = "`RecipeAdditionWater`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"amount_l\","]
#[doc = "    \"id\","]
#[doc = "    \"name\","]
#[doc = "    \"recipe_id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"amount_l\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"recipe_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"water_id\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct RecipeAdditionWater {
    pub amount_l: f64,
    pub id: ::std::string::String,
    pub name: ::std::string::String,
    pub recipe_id: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub water_id: ::std::option::Option<::std::string::String>,
}
impl RecipeAdditionWater {
    pub fn builder() -> builder::RecipeAdditionWater {
        Default::default()
    }
}
#[doc = "`RecipeAdditionYeast`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"add_to_secondary\","]
#[doc = "    \"amount_is_weight\","]
#[doc = "    \"form\","]
#[doc = "    \"id\","]
#[doc = "    \"name\","]
#[doc = "    \"recipe_id\","]
#[doc = "    \"times_cultured\","]
#[doc = "    \"type_\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"add_to_secondary\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"amount\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"amount_is_weight\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"attenuation_pct\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"form\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"laboratory\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"product_id\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"recipe_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"times_cultured\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"type_\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"yeast_id\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct RecipeAdditionYeast {
    pub add_to_secondary: bool,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub amount: ::std::option::Option<f64>,
    pub amount_is_weight: bool,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub attenuation_pct: ::std::option::Option<f64>,
    pub form: ::std::string::String,
    pub id: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub laboratory: ::std::option::Option<::std::string::String>,
    pub name: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub product_id: ::std::option::Option<::std::string::String>,
    pub recipe_id: ::std::string::String,
    pub times_cultured: i64,
    pub type_: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub yeast_id: ::std::option::Option<::std::string::String>,
}
impl RecipeAdditionYeast {
    pub fn builder() -> builder::RecipeAdditionYeast {
        Default::default()
    }
}
#[doc = "`RecipeStats`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"abv_pct\","]
#[doc = "    \"bu_gu_ratio\","]
#[doc = "    \"calories_per_355ml\","]
#[doc = "    \"fg\","]
#[doc = "    \"ibu\","]
#[doc = "    \"og\","]
#[doc = "    \"post_boil_volume_l\","]
#[doc = "    \"pre_boil_gravity\","]
#[doc = "    \"pre_boil_volume_l\","]
#[doc = "    \"srm\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"abv_pct\": {"]
#[doc = "      \"description\": \"Alcohol by volume percentage\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"bu_gu_ratio\": {"]
#[doc = "      \"description\": \"Bitterness/gravity ratio\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"calories_per_355ml\": {"]
#[doc = "      \"description\": \"Estimated calories per 355 ml (12 oz) serving\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"fg\": {"]
#[doc = "      \"description\": \"Calculated final gravity (specific gravity)\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"ibu\": {"]
#[doc = "      \"description\": \"International Bitterness Units\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"og\": {"]
#[doc = "      \"description\": \"Calculated original gravity (specific gravity)\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"post_boil_volume_l\": {"]
#[doc = "      \"description\": \"Estimated post-boil volume in litres\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"pre_boil_gravity\": {"]
#[doc = "      \"description\": \"Estimated pre-boil specific gravity\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"pre_boil_volume_l\": {"]
#[doc = "      \"description\": \"Estimated pre-boil volume in litres\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"srm\": {"]
#[doc = "      \"description\": \"Standard Reference Method color value\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"strike_temp_c\": {"]
#[doc = "      \"description\": \"Calculated strike water temperature in degrees Celsius\","]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct RecipeStats {
    #[doc = "Alcohol by volume percentage"]
    pub abv_pct: f64,
    #[doc = "Bitterness/gravity ratio"]
    pub bu_gu_ratio: f64,
    #[doc = "Estimated calories per 355 ml (12 oz) serving"]
    pub calories_per_355ml: f64,
    #[doc = "Calculated final gravity (specific gravity)"]
    pub fg: f64,
    #[doc = "International Bitterness Units"]
    pub ibu: f64,
    #[doc = "Calculated original gravity (specific gravity)"]
    pub og: f64,
    #[doc = "Estimated post-boil volume in litres"]
    pub post_boil_volume_l: f64,
    #[doc = "Estimated pre-boil specific gravity"]
    pub pre_boil_gravity: f64,
    #[doc = "Estimated pre-boil volume in litres"]
    pub pre_boil_volume_l: f64,
    #[doc = "Standard Reference Method color value"]
    pub srm: f64,
    #[doc = "Calculated strike water temperature in degrees Celsius"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub strike_temp_c: ::std::option::Option<f64>,
}
impl RecipeStats {
    pub fn builder() -> builder::RecipeStats {
        Default::default()
    }
}
#[doc = "`RecipeSummary`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"batch_size_l\","]
#[doc = "    \"created_at\","]
#[doc = "    \"id\","]
#[doc = "    \"name\","]
#[doc = "    \"type_\","]
#[doc = "    \"updated_at\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"batch_size_l\": {"]
#[doc = "      \"description\": \"Target batch volume in litres\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"created_at\": {"]
#[doc = "      \"description\": \"Unix timestamp (milliseconds)\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"style_name\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"type_\": {"]
#[doc = "      \"description\": \"Recipe type (e.g. All Grain, Extract, Partial Mash)\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"updated_at\": {"]
#[doc = "      \"description\": \"Unix timestamp (milliseconds)\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct RecipeSummary {
    #[doc = "Target batch volume in litres"]
    pub batch_size_l: f64,
    #[doc = "Unix timestamp (milliseconds)"]
    pub created_at: i64,
    pub id: ::std::string::String,
    pub name: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub style_name: ::std::option::Option<::std::string::String>,
    #[doc = "Recipe type (e.g. All Grain, Extract, Partial Mash)"]
    pub type_: ::std::string::String,
    #[doc = "Unix timestamp (milliseconds)"]
    pub updated_at: i64,
}
impl RecipeSummary {
    pub fn builder() -> builder::RecipeSummary {
        Default::default()
    }
}
#[doc = "`RecipeWaterAdjustment`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"addition\","]
#[doc = "    \"amount\","]
#[doc = "    \"id\","]
#[doc = "    \"recipe_id\","]
#[doc = "    \"target\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"addition\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"gypsum\","]
#[doc = "        \"calcium_chloride\","]
#[doc = "        \"epsom_salt\","]
#[doc = "        \"table_salt\","]
#[doc = "        \"baking_soda\","]
#[doc = "        \"chalk\","]
#[doc = "        \"lactic_acid\","]
#[doc = "        \"phosphoric_acid\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"amount\": {"]
#[doc = "      \"description\": \"Amount in grams for salts, ml for acids\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"recipe_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"target\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"mash\","]
#[doc = "        \"sparge\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct RecipeWaterAdjustment {
    pub addition: RecipeWaterAdjustmentAddition,
    #[doc = "Amount in grams for salts, ml for acids"]
    pub amount: f64,
    pub id: ::std::string::String,
    pub recipe_id: ::std::string::String,
    pub target: RecipeWaterAdjustmentTarget,
}
impl RecipeWaterAdjustment {
    pub fn builder() -> builder::RecipeWaterAdjustment {
        Default::default()
    }
}
#[doc = "`RecipeWaterAdjustmentAddition`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"gypsum\","]
#[doc = "    \"calcium_chloride\","]
#[doc = "    \"epsom_salt\","]
#[doc = "    \"table_salt\","]
#[doc = "    \"baking_soda\","]
#[doc = "    \"chalk\","]
#[doc = "    \"lactic_acid\","]
#[doc = "    \"phosphoric_acid\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum RecipeWaterAdjustmentAddition {
    #[serde(rename = "gypsum")]
    Gypsum,
    #[serde(rename = "calcium_chloride")]
    CalciumChloride,
    #[serde(rename = "epsom_salt")]
    EpsomSalt,
    #[serde(rename = "table_salt")]
    TableSalt,
    #[serde(rename = "baking_soda")]
    BakingSoda,
    #[serde(rename = "chalk")]
    Chalk,
    #[serde(rename = "lactic_acid")]
    LacticAcid,
    #[serde(rename = "phosphoric_acid")]
    PhosphoricAcid,
}
impl ::std::fmt::Display for RecipeWaterAdjustmentAddition {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Gypsum => f.write_str("gypsum"),
            Self::CalciumChloride => f.write_str("calcium_chloride"),
            Self::EpsomSalt => f.write_str("epsom_salt"),
            Self::TableSalt => f.write_str("table_salt"),
            Self::BakingSoda => f.write_str("baking_soda"),
            Self::Chalk => f.write_str("chalk"),
            Self::LacticAcid => f.write_str("lactic_acid"),
            Self::PhosphoricAcid => f.write_str("phosphoric_acid"),
        }
    }
}
impl ::std::str::FromStr for RecipeWaterAdjustmentAddition {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "gypsum" => Ok(Self::Gypsum),
            "calcium_chloride" => Ok(Self::CalciumChloride),
            "epsom_salt" => Ok(Self::EpsomSalt),
            "table_salt" => Ok(Self::TableSalt),
            "baking_soda" => Ok(Self::BakingSoda),
            "chalk" => Ok(Self::Chalk),
            "lactic_acid" => Ok(Self::LacticAcid),
            "phosphoric_acid" => Ok(Self::PhosphoricAcid),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RecipeWaterAdjustmentAddition {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RecipeWaterAdjustmentAddition {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RecipeWaterAdjustmentAddition {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`RecipeWaterAdjustmentTarget`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"mash\","]
#[doc = "    \"sparge\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum RecipeWaterAdjustmentTarget {
    #[serde(rename = "mash")]
    Mash,
    #[serde(rename = "sparge")]
    Sparge,
}
impl ::std::fmt::Display for RecipeWaterAdjustmentTarget {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Mash => f.write_str("mash"),
            Self::Sparge => f.write_str("sparge"),
        }
    }
}
impl ::std::str::FromStr for RecipeWaterAdjustmentTarget {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "mash" => Ok(Self::Mash),
            "sparge" => Ok(Self::Sparge),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RecipeWaterAdjustmentTarget {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RecipeWaterAdjustmentTarget {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RecipeWaterAdjustmentTarget {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`Style`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"category\","]
#[doc = "    \"category_number\","]
#[doc = "    \"color_max_srm\","]
#[doc = "    \"color_min_srm\","]
#[doc = "    \"fg_max\","]
#[doc = "    \"fg_min\","]
#[doc = "    \"ibu_max\","]
#[doc = "    \"ibu_min\","]
#[doc = "    \"id\","]
#[doc = "    \"name\","]
#[doc = "    \"og_max\","]
#[doc = "    \"og_min\","]
#[doc = "    \"style_guide\","]
#[doc = "    \"style_letter\","]
#[doc = "    \"type_\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"abv_max_pct\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"abv_min_pct\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"carb_max_vols\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"carb_min_vols\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"category\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"category_number\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"color_max_srm\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"color_min_srm\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"examples\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"fg_max\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"fg_min\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"ibu_max\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"ibu_min\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"ingredients\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"notes\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"og_max\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"og_min\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"profile\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"style_guide\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"style_letter\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"type_\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Style {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub abv_max_pct: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub abv_min_pct: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub carb_max_vols: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub carb_min_vols: ::std::option::Option<f64>,
    pub category: ::std::string::String,
    pub category_number: ::std::string::String,
    pub color_max_srm: f64,
    pub color_min_srm: f64,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub examples: ::std::option::Option<::std::string::String>,
    pub fg_max: f64,
    pub fg_min: f64,
    pub ibu_max: f64,
    pub ibu_min: f64,
    pub id: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ingredients: ::std::option::Option<::std::string::String>,
    pub name: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub notes: ::std::option::Option<::std::string::String>,
    pub og_max: f64,
    pub og_min: f64,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub profile: ::std::option::Option<::std::string::String>,
    pub style_guide: ::std::string::String,
    pub style_letter: ::std::string::String,
    pub type_: ::std::string::String,
}
impl Style {
    pub fn builder() -> builder::Style {
        Default::default()
    }
}
#[doc = "`UpdateEquipmentProfileInput`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"batch_size_l\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"boil_size_l\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"boil_time_min\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"efficiency_pct\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"evap_rate_pct_hr\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"fermenter_loss_l\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"notes\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"trub_chiller_loss_l\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct UpdateEquipmentProfileInput {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub batch_size_l: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub boil_size_l: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub boil_time_min: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub efficiency_pct: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub evap_rate_pct_hr: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub fermenter_loss_l: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub notes: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub trub_chiller_loss_l: ::std::option::Option<f64>,
}
impl ::std::default::Default for UpdateEquipmentProfileInput {
    fn default() -> Self {
        Self {
            batch_size_l: Default::default(),
            boil_size_l: Default::default(),
            boil_time_min: Default::default(),
            efficiency_pct: Default::default(),
            evap_rate_pct_hr: Default::default(),
            fermenter_loss_l: Default::default(),
            name: Default::default(),
            notes: Default::default(),
            trub_chiller_loss_l: Default::default(),
        }
    }
}
impl UpdateEquipmentProfileInput {
    pub fn builder() -> builder::UpdateEquipmentProfileInput {
        Default::default()
    }
}
#[doc = "`UpdateFermentableAdditionInput`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"add_after_boil\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"addition_order\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"amount_kg\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct UpdateFermentableAdditionInput {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub add_after_boil: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub addition_order: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub amount_kg: ::std::option::Option<f64>,
}
impl ::std::default::Default for UpdateFermentableAdditionInput {
    fn default() -> Self {
        Self {
            add_after_boil: Default::default(),
            addition_order: Default::default(),
            amount_kg: Default::default(),
        }
    }
}
impl UpdateFermentableAdditionInput {
    pub fn builder() -> builder::UpdateFermentableAdditionInput {
        Default::default()
    }
}
#[doc = "`UpdateHopAdditionInput`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"addition_order\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"amount_kg\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"time_min\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"use_\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"hopstand_temp_c\": {"]
#[doc = "      \"description\": \"Whirlpool temperature override for this addition in °C\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct UpdateHopAdditionInput {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub addition_order: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub amount_kg: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub time_min: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub use_: ::std::option::Option<::std::string::String>,
    #[doc = "Whirlpool temperature override for this addition in °C"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub hopstand_temp_c: ::std::option::Option<f64>,
}
impl ::std::default::Default for UpdateHopAdditionInput {
    fn default() -> Self {
        Self {
            addition_order: Default::default(),
            amount_kg: Default::default(),
            time_min: Default::default(),
            use_: Default::default(),
            hopstand_temp_c: Default::default(),
        }
    }
}
impl UpdateHopAdditionInput {
    pub fn builder() -> builder::UpdateHopAdditionInput {
        Default::default()
    }
}
#[doc = "`UpdateMashInput`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"grain_temp_c\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"notes\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"ph\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"ratio_l_per_kg\": {"]
#[doc = "      \"description\": \"Water-to-grain ratio in litres per kilogram\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"sparge_temp_c\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"tun_temp_c\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct UpdateMashInput {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub grain_temp_c: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub notes: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ph: ::std::option::Option<f64>,
    #[doc = "Water-to-grain ratio in litres per kilogram"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ratio_l_per_kg: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sparge_temp_c: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tun_temp_c: ::std::option::Option<f64>,
}
impl ::std::default::Default for UpdateMashInput {
    fn default() -> Self {
        Self {
            grain_temp_c: Default::default(),
            name: Default::default(),
            notes: Default::default(),
            ph: Default::default(),
            ratio_l_per_kg: Default::default(),
            sparge_temp_c: Default::default(),
            tun_temp_c: Default::default(),
        }
    }
}
impl UpdateMashInput {
    pub fn builder() -> builder::UpdateMashInput {
        Default::default()
    }
}
#[doc = "`UpdateMashStepInput`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"end_temp_c\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"infuse_amount_l\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"ramp_time_min\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"step_temp_c\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"step_time_min\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"type_\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct UpdateMashStepInput {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub end_temp_c: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub infuse_amount_l: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ramp_time_min: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub step_temp_c: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub step_time_min: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for UpdateMashStepInput {
    fn default() -> Self {
        Self {
            end_temp_c: Default::default(),
            infuse_amount_l: Default::default(),
            name: Default::default(),
            ramp_time_min: Default::default(),
            step_temp_c: Default::default(),
            step_time_min: Default::default(),
            type_: Default::default(),
        }
    }
}
impl UpdateMashStepInput {
    pub fn builder() -> builder::UpdateMashStepInput {
        Default::default()
    }
}
#[doc = "`UpdateMiscAdditionInput`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"addition_order\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"amount\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"amount_is_weight\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"time_min\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"use_\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct UpdateMiscAdditionInput {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub addition_order: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub amount: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub amount_is_weight: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub time_min: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub use_: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for UpdateMiscAdditionInput {
    fn default() -> Self {
        Self {
            addition_order: Default::default(),
            amount: Default::default(),
            amount_is_weight: Default::default(),
            time_min: Default::default(),
            use_: Default::default(),
        }
    }
}
impl UpdateMiscAdditionInput {
    pub fn builder() -> builder::UpdateMiscAdditionInput {
        Default::default()
    }
}
#[doc = "`UpdateRecipeInput`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"age_days\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"age_temp_c\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"asst_brewer\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"batch_size_l\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"boil_size_l\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"boil_time_min\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"brewer\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"carbonation_temp_c\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"carbonation_vols\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"date\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"efficiency_pct\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"equipment_profile_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"fermentation_stages\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"forced_carbonation\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"keg_priming_factor\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"notes\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"primary_age_days\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"primary_temp_c\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"priming_sugar_equiv\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"priming_sugar_name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"secondary_age_days\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"secondary_temp_c\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"style_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"taste_notes\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"taste_rating\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"tertiary_age_days\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"tertiary_temp_c\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"type_\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"hopstand_temp_c\": {"]
#[doc = "      \"description\": \"Default whirlpool/hopstand temperature in °C\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct UpdateRecipeInput {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub age_days: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub age_temp_c: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub asst_brewer: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub batch_size_l: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub boil_size_l: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub boil_time_min: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub brewer: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub carbonation_temp_c: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub carbonation_vols: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub date: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub efficiency_pct: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub equipment_profile_id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub fermentation_stages: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub forced_carbonation: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub keg_priming_factor: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub notes: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub primary_age_days: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub primary_temp_c: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub priming_sugar_equiv: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub priming_sugar_name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub secondary_age_days: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub secondary_temp_c: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub style_id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub taste_notes: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub taste_rating: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tertiary_age_days: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tertiary_temp_c: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<::std::string::String>,
    #[doc = "Default whirlpool/hopstand temperature in °C"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub hopstand_temp_c: ::std::option::Option<f64>,
}
impl ::std::default::Default for UpdateRecipeInput {
    fn default() -> Self {
        Self {
            age_days: Default::default(),
            age_temp_c: Default::default(),
            asst_brewer: Default::default(),
            batch_size_l: Default::default(),
            boil_size_l: Default::default(),
            boil_time_min: Default::default(),
            brewer: Default::default(),
            carbonation_temp_c: Default::default(),
            carbonation_vols: Default::default(),
            date: Default::default(),
            efficiency_pct: Default::default(),
            equipment_profile_id: Default::default(),
            fermentation_stages: Default::default(),
            forced_carbonation: Default::default(),
            keg_priming_factor: Default::default(),
            name: Default::default(),
            notes: Default::default(),
            primary_age_days: Default::default(),
            primary_temp_c: Default::default(),
            priming_sugar_equiv: Default::default(),
            priming_sugar_name: Default::default(),
            secondary_age_days: Default::default(),
            secondary_temp_c: Default::default(),
            style_id: Default::default(),
            taste_notes: Default::default(),
            taste_rating: Default::default(),
            tertiary_age_days: Default::default(),
            tertiary_temp_c: Default::default(),
            type_: Default::default(),
            hopstand_temp_c: Default::default(),
        }
    }
}
impl UpdateRecipeInput {
    pub fn builder() -> builder::UpdateRecipeInput {
        Default::default()
    }
}
#[doc = "`UpdateWaterAdditionInput`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"amount_l\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct UpdateWaterAdditionInput {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub amount_l: ::std::option::Option<f64>,
}
impl ::std::default::Default for UpdateWaterAdditionInput {
    fn default() -> Self {
        Self {
            amount_l: Default::default(),
        }
    }
}
impl UpdateWaterAdditionInput {
    pub fn builder() -> builder::UpdateWaterAdditionInput {
        Default::default()
    }
}
#[doc = "`UpdateWaterAdjustmentInput`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"addition\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"gypsum\","]
#[doc = "        \"calcium_chloride\","]
#[doc = "        \"epsom_salt\","]
#[doc = "        \"table_salt\","]
#[doc = "        \"baking_soda\","]
#[doc = "        \"chalk\","]
#[doc = "        \"lactic_acid\","]
#[doc = "        \"phosphoric_acid\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"amount\": {"]
#[doc = "      \"description\": \"Amount in grams for salts, ml for acids\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"target\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"mash\","]
#[doc = "        \"sparge\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct UpdateWaterAdjustmentInput {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub addition: ::std::option::Option<UpdateWaterAdjustmentInputAddition>,
    #[doc = "Amount in grams for salts, ml for acids"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub amount: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub target: ::std::option::Option<UpdateWaterAdjustmentInputTarget>,
}
impl ::std::default::Default for UpdateWaterAdjustmentInput {
    fn default() -> Self {
        Self {
            addition: Default::default(),
            amount: Default::default(),
            target: Default::default(),
        }
    }
}
impl UpdateWaterAdjustmentInput {
    pub fn builder() -> builder::UpdateWaterAdjustmentInput {
        Default::default()
    }
}
#[doc = "`UpdateWaterAdjustmentInputAddition`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"gypsum\","]
#[doc = "    \"calcium_chloride\","]
#[doc = "    \"epsom_salt\","]
#[doc = "    \"table_salt\","]
#[doc = "    \"baking_soda\","]
#[doc = "    \"chalk\","]
#[doc = "    \"lactic_acid\","]
#[doc = "    \"phosphoric_acid\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum UpdateWaterAdjustmentInputAddition {
    #[serde(rename = "gypsum")]
    Gypsum,
    #[serde(rename = "calcium_chloride")]
    CalciumChloride,
    #[serde(rename = "epsom_salt")]
    EpsomSalt,
    #[serde(rename = "table_salt")]
    TableSalt,
    #[serde(rename = "baking_soda")]
    BakingSoda,
    #[serde(rename = "chalk")]
    Chalk,
    #[serde(rename = "lactic_acid")]
    LacticAcid,
    #[serde(rename = "phosphoric_acid")]
    PhosphoricAcid,
}
impl ::std::fmt::Display for UpdateWaterAdjustmentInputAddition {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Gypsum => f.write_str("gypsum"),
            Self::CalciumChloride => f.write_str("calcium_chloride"),
            Self::EpsomSalt => f.write_str("epsom_salt"),
            Self::TableSalt => f.write_str("table_salt"),
            Self::BakingSoda => f.write_str("baking_soda"),
            Self::Chalk => f.write_str("chalk"),
            Self::LacticAcid => f.write_str("lactic_acid"),
            Self::PhosphoricAcid => f.write_str("phosphoric_acid"),
        }
    }
}
impl ::std::str::FromStr for UpdateWaterAdjustmentInputAddition {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "gypsum" => Ok(Self::Gypsum),
            "calcium_chloride" => Ok(Self::CalciumChloride),
            "epsom_salt" => Ok(Self::EpsomSalt),
            "table_salt" => Ok(Self::TableSalt),
            "baking_soda" => Ok(Self::BakingSoda),
            "chalk" => Ok(Self::Chalk),
            "lactic_acid" => Ok(Self::LacticAcid),
            "phosphoric_acid" => Ok(Self::PhosphoricAcid),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UpdateWaterAdjustmentInputAddition {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for UpdateWaterAdjustmentInputAddition {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UpdateWaterAdjustmentInputAddition {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`UpdateWaterAdjustmentInputTarget`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"mash\","]
#[doc = "    \"sparge\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum UpdateWaterAdjustmentInputTarget {
    #[serde(rename = "mash")]
    Mash,
    #[serde(rename = "sparge")]
    Sparge,
}
impl ::std::fmt::Display for UpdateWaterAdjustmentInputTarget {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Mash => f.write_str("mash"),
            Self::Sparge => f.write_str("sparge"),
        }
    }
}
impl ::std::str::FromStr for UpdateWaterAdjustmentInputTarget {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "mash" => Ok(Self::Mash),
            "sparge" => Ok(Self::Sparge),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UpdateWaterAdjustmentInputTarget {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for UpdateWaterAdjustmentInputTarget {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UpdateWaterAdjustmentInputTarget {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`UpdateYeastAdditionInput`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"add_to_secondary\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"amount\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"amount_is_weight\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"attenuation_pct\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"times_cultured\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct UpdateYeastAdditionInput {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub add_to_secondary: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub amount: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub amount_is_weight: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub attenuation_pct: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub times_cultured: ::std::option::Option<i64>,
}
impl ::std::default::Default for UpdateYeastAdditionInput {
    fn default() -> Self {
        Self {
            add_to_secondary: Default::default(),
            amount: Default::default(),
            amount_is_weight: Default::default(),
            attenuation_pct: Default::default(),
            times_cultured: Default::default(),
        }
    }
}
impl UpdateYeastAdditionInput {
    pub fn builder() -> builder::UpdateYeastAdditionInput {
        Default::default()
    }
}
#[doc = "`Water`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"bicarbonate_ppm\","]
#[doc = "    \"calcium_ppm\","]
#[doc = "    \"chloride_ppm\","]
#[doc = "    \"id\","]
#[doc = "    \"magnesium_ppm\","]
#[doc = "    \"name\","]
#[doc = "    \"sodium_ppm\","]
#[doc = "    \"sulfate_ppm\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"bicarbonate_ppm\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"calcium_ppm\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"chloride_ppm\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"magnesium_ppm\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"notes\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"ph\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"sodium_ppm\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"sulfate_ppm\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Water {
    pub bicarbonate_ppm: f64,
    pub calcium_ppm: f64,
    pub chloride_ppm: f64,
    pub id: ::std::string::String,
    pub magnesium_ppm: f64,
    pub name: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub notes: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ph: ::std::option::Option<f64>,
    pub sodium_ppm: f64,
    pub sulfate_ppm: f64,
}
impl Water {
    pub fn builder() -> builder::Water {
        Default::default()
    }
}
#[doc = "`WaterProfile`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"bicarbonate_ppm\","]
#[doc = "    \"calcium_ppm\","]
#[doc = "    \"chloride_ppm\","]
#[doc = "    \"cl_so4_ratio\","]
#[doc = "    \"magnesium_ppm\","]
#[doc = "    \"sodium_ppm\","]
#[doc = "    \"sulfate_ppm\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"bicarbonate_ppm\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"calcium_ppm\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"chloride_ppm\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"cl_so4_ratio\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"magnesium_ppm\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"sodium_ppm\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"sulfate_ppm\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct WaterProfile {
    pub bicarbonate_ppm: f64,
    pub calcium_ppm: f64,
    pub chloride_ppm: f64,
    pub cl_so4_ratio: f64,
    pub magnesium_ppm: f64,
    pub sodium_ppm: f64,
    pub sulfate_ppm: f64,
}
impl WaterProfile {
    pub fn builder() -> builder::WaterProfile {
        Default::default()
    }
}
#[doc = "`Yeast`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"add_to_secondary\","]
#[doc = "    \"form\","]
#[doc = "    \"id\","]
#[doc = "    \"name\","]
#[doc = "    \"type_\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"add_to_secondary\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"alcohol_tolerance\": {"]
#[doc = "      \"description\": \"low, medium, high, very_high\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"attenuation_pct\": {"]
#[doc = "      \"description\": \"BeerXML single attenuation value; see min/max fields for range\","]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"best_for\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"flavor_profile\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"flocculation\": {"]
#[doc = "      \"description\": \"Low, Medium, High, Very High\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"form\": {"]
#[doc = "      \"description\": \"Liquid, Dry, Slant, Culture\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"laboratory\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"max_attenuation_pct\": {"]
#[doc = "      \"description\": \"BeerMaverick attenuation range upper bound\","]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"max_reuse\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"max_temperature_c\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"min_attenuation_pct\": {"]
#[doc = "      \"description\": \"BeerMaverick attenuation range lower bound\","]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"min_temperature_c\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"notes\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"pof_positive\": {"]
#[doc = "      \"description\": \"Phenolic Off-Flavor gene present\","]
#[doc = "      \"type\": ["]
#[doc = "        \"boolean\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"product_id\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"species\": {"]
#[doc = "      \"description\": \"e.g. Saccharomyces cerevisiae\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"sta1_positive\": {"]
#[doc = "      \"description\": \"STA-1 dextrin-fermenting gene present\","]
#[doc = "      \"type\": ["]
#[doc = "        \"boolean\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"styles\": {"]
#[doc = "      \"description\": \"Suitable beer styles, comma-separated\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"substitutes\": {"]
#[doc = "      \"description\": \"Substitute yeast strains, comma-separated\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"type_\": {"]
#[doc = "      \"description\": \"Ale, Lager, Wheat, Wine, Champagne\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Yeast {
    pub add_to_secondary: bool,
    #[doc = "low, medium, high, very_high"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub alcohol_tolerance: ::std::option::Option<::std::string::String>,
    #[doc = "BeerXML single attenuation value; see min/max fields for range"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub attenuation_pct: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub best_for: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub flavor_profile: ::std::option::Option<::std::string::String>,
    #[doc = "Low, Medium, High, Very High"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub flocculation: ::std::option::Option<::std::string::String>,
    #[doc = "Liquid, Dry, Slant, Culture"]
    pub form: ::std::string::String,
    pub id: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub laboratory: ::std::option::Option<::std::string::String>,
    #[doc = "BeerMaverick attenuation range upper bound"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max_attenuation_pct: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max_reuse: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max_temperature_c: ::std::option::Option<f64>,
    #[doc = "BeerMaverick attenuation range lower bound"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub min_attenuation_pct: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub min_temperature_c: ::std::option::Option<f64>,
    pub name: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub notes: ::std::option::Option<::std::string::String>,
    #[doc = "Phenolic Off-Flavor gene present"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub pof_positive: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub product_id: ::std::option::Option<::std::string::String>,
    #[doc = "e.g. Saccharomyces cerevisiae"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub species: ::std::option::Option<::std::string::String>,
    #[doc = "STA-1 dextrin-fermenting gene present"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sta1_positive: ::std::option::Option<bool>,
    #[doc = "Suitable beer styles, comma-separated"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub styles: ::std::option::Option<::std::string::String>,
    #[doc = "Substitute yeast strains, comma-separated"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub substitutes: ::std::option::Option<::std::string::String>,
    #[doc = "Ale, Lager, Wheat, Wine, Champagne"]
    pub type_: ::std::string::String,
}
impl Yeast {
    pub fn builder() -> builder::Yeast {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct CalculatedWaterProfile {
        combined: ::std::result::Result<super::WaterProfile, ::std::string::String>,
        mash: ::std::result::Result<super::WaterProfile, ::std::string::String>,
        sparge: ::std::result::Result<super::WaterProfile, ::std::string::String>,
    }
    impl ::std::default::Default for CalculatedWaterProfile {
        fn default() -> Self {
            Self {
                combined: Err("no value supplied for combined".to_string()),
                mash: Err("no value supplied for mash".to_string()),
                sparge: Err("no value supplied for sparge".to_string()),
            }
        }
    }
    impl CalculatedWaterProfile {
        pub fn combined<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::WaterProfile>,
            T::Error: ::std::fmt::Display,
        {
            self.combined = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for combined: {e}"));
            self
        }
        pub fn mash<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::WaterProfile>,
            T::Error: ::std::fmt::Display,
        {
            self.mash = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mash: {e}"));
            self
        }
        pub fn sparge<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::WaterProfile>,
            T::Error: ::std::fmt::Display,
        {
            self.sparge = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sparge: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CalculatedWaterProfile> for super::CalculatedWaterProfile {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CalculatedWaterProfile,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                combined: value.combined?,
                mash: value.mash?,
                sparge: value.sparge?,
            })
        }
    }
    impl ::std::convert::From<super::CalculatedWaterProfile> for CalculatedWaterProfile {
        fn from(value: super::CalculatedWaterProfile) -> Self {
            Self {
                combined: Ok(value.combined),
                mash: Ok(value.mash),
                sparge: Ok(value.sparge),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateEquipmentProfileInput {
        batch_size_l: ::std::result::Result<f64, ::std::string::String>,
        boil_size_l: ::std::result::Result<f64, ::std::string::String>,
        boil_time_min: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        efficiency_pct: ::std::result::Result<f64, ::std::string::String>,
        evap_rate_pct_hr: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        fermenter_loss_l: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        notes: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        trub_chiller_loss_l:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for CreateEquipmentProfileInput {
        fn default() -> Self {
            Self {
                batch_size_l: Err("no value supplied for batch_size_l".to_string()),
                boil_size_l: Err("no value supplied for boil_size_l".to_string()),
                boil_time_min: Ok(Default::default()),
                efficiency_pct: Err("no value supplied for efficiency_pct".to_string()),
                evap_rate_pct_hr: Ok(Default::default()),
                fermenter_loss_l: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                notes: Ok(Default::default()),
                trub_chiller_loss_l: Ok(Default::default()),
            }
        }
    }
    impl CreateEquipmentProfileInput {
        pub fn batch_size_l<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.batch_size_l = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for batch_size_l: {e}"));
            self
        }
        pub fn boil_size_l<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.boil_size_l = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for boil_size_l: {e}"));
            self
        }
        pub fn boil_time_min<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.boil_time_min = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for boil_time_min: {e}"));
            self
        }
        pub fn efficiency_pct<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.efficiency_pct = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for efficiency_pct: {e}"));
            self
        }
        pub fn evap_rate_pct_hr<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.evap_rate_pct_hr = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for evap_rate_pct_hr: {e}"));
            self
        }
        pub fn fermenter_loss_l<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.fermenter_loss_l = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for fermenter_loss_l: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn notes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.notes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for notes: {e}"));
            self
        }
        pub fn trub_chiller_loss_l<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.trub_chiller_loss_l = value.try_into().map_err(|e| {
                format!("error converting supplied value for trub_chiller_loss_l: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<CreateEquipmentProfileInput> for super::CreateEquipmentProfileInput {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateEquipmentProfileInput,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                batch_size_l: value.batch_size_l?,
                boil_size_l: value.boil_size_l?,
                boil_time_min: value.boil_time_min?,
                efficiency_pct: value.efficiency_pct?,
                evap_rate_pct_hr: value.evap_rate_pct_hr?,
                fermenter_loss_l: value.fermenter_loss_l?,
                name: value.name?,
                notes: value.notes?,
                trub_chiller_loss_l: value.trub_chiller_loss_l?,
            })
        }
    }
    impl ::std::convert::From<super::CreateEquipmentProfileInput> for CreateEquipmentProfileInput {
        fn from(value: super::CreateEquipmentProfileInput) -> Self {
            Self {
                batch_size_l: Ok(value.batch_size_l),
                boil_size_l: Ok(value.boil_size_l),
                boil_time_min: Ok(value.boil_time_min),
                efficiency_pct: Ok(value.efficiency_pct),
                evap_rate_pct_hr: Ok(value.evap_rate_pct_hr),
                fermenter_loss_l: Ok(value.fermenter_loss_l),
                name: Ok(value.name),
                notes: Ok(value.notes),
                trub_chiller_loss_l: Ok(value.trub_chiller_loss_l),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateFermentableAdditionInput {
        add_after_boil: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        amount_kg: ::std::result::Result<f64, ::std::string::String>,
        color_lovibond: ::std::result::Result<f64, ::std::string::String>,
        fermentable_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
        yield_pct: ::std::result::Result<f64, ::std::string::String>,
    }
    impl ::std::default::Default for CreateFermentableAdditionInput {
        fn default() -> Self {
            Self {
                add_after_boil: Ok(Default::default()),
                amount_kg: Err("no value supplied for amount_kg".to_string()),
                color_lovibond: Err("no value supplied for color_lovibond".to_string()),
                fermentable_id: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                type_: Err("no value supplied for type_".to_string()),
                yield_pct: Err("no value supplied for yield_pct".to_string()),
            }
        }
    }
    impl CreateFermentableAdditionInput {
        pub fn add_after_boil<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.add_after_boil = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for add_after_boil: {e}"));
            self
        }
        pub fn amount_kg<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.amount_kg = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for amount_kg: {e}"));
            self
        }
        pub fn color_lovibond<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.color_lovibond = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for color_lovibond: {e}"));
            self
        }
        pub fn fermentable_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.fermentable_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for fermentable_id: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
        pub fn yield_pct<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.yield_pct = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for yield_pct: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateFermentableAdditionInput>
        for super::CreateFermentableAdditionInput
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateFermentableAdditionInput,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                add_after_boil: value.add_after_boil?,
                amount_kg: value.amount_kg?,
                color_lovibond: value.color_lovibond?,
                fermentable_id: value.fermentable_id?,
                name: value.name?,
                type_: value.type_?,
                yield_pct: value.yield_pct?,
            })
        }
    }
    impl ::std::convert::From<super::CreateFermentableAdditionInput>
        for CreateFermentableAdditionInput
    {
        fn from(value: super::CreateFermentableAdditionInput) -> Self {
            Self {
                add_after_boil: Ok(value.add_after_boil),
                amount_kg: Ok(value.amount_kg),
                color_lovibond: Ok(value.color_lovibond),
                fermentable_id: Ok(value.fermentable_id),
                name: Ok(value.name),
                type_: Ok(value.type_),
                yield_pct: Ok(value.yield_pct),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateHopAdditionInput {
        alpha_pct: ::std::result::Result<f64, ::std::string::String>,
        amount_kg: ::std::result::Result<f64, ::std::string::String>,
        form: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        hop_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        time_min: ::std::result::Result<f64, ::std::string::String>,
        use_: ::std::result::Result<::std::string::String, ::std::string::String>,
        hopstand_temp_c: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for CreateHopAdditionInput {
        fn default() -> Self {
            Self {
                alpha_pct: Err("no value supplied for alpha_pct".to_string()),
                amount_kg: Err("no value supplied for amount_kg".to_string()),
                form: Ok(Default::default()),
                hop_id: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                time_min: Err("no value supplied for time_min".to_string()),
                use_: Err("no value supplied for use_".to_string()),
                hopstand_temp_c: Ok(Default::default()),
            }
        }
    }
    impl CreateHopAdditionInput {
        pub fn alpha_pct<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.alpha_pct = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for alpha_pct: {e}"));
            self
        }
        pub fn amount_kg<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.amount_kg = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for amount_kg: {e}"));
            self
        }
        pub fn form<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.form = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for form: {e}"));
            self
        }
        pub fn hop_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.hop_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for hop_id: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn time_min<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.time_min = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for time_min: {e}"));
            self
        }
        pub fn use_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.use_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for use_: {e}"));
            self
        }
        pub fn hopstand_temp_c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.hopstand_temp_c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for hopstand_temp_c: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateHopAdditionInput> for super::CreateHopAdditionInput {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateHopAdditionInput,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                alpha_pct: value.alpha_pct?,
                amount_kg: value.amount_kg?,
                form: value.form?,
                hop_id: value.hop_id?,
                name: value.name?,
                time_min: value.time_min?,
                use_: value.use_?,
                hopstand_temp_c: value.hopstand_temp_c?,
            })
        }
    }
    impl ::std::convert::From<super::CreateHopAdditionInput> for CreateHopAdditionInput {
        fn from(value: super::CreateHopAdditionInput) -> Self {
            Self {
                alpha_pct: Ok(value.alpha_pct),
                amount_kg: Ok(value.amount_kg),
                form: Ok(value.form),
                hop_id: Ok(value.hop_id),
                name: Ok(value.name),
                time_min: Ok(value.time_min),
                use_: Ok(value.use_),
                hopstand_temp_c: Ok(value.hopstand_temp_c),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateMashStepInput {
        end_temp_c: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        infuse_amount_l: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        ramp_time_min: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        step_temp_c: ::std::result::Result<f64, ::std::string::String>,
        step_time_min: ::std::result::Result<i64, ::std::string::String>,
        type_: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreateMashStepInput {
        fn default() -> Self {
            Self {
                end_temp_c: Ok(Default::default()),
                infuse_amount_l: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                ramp_time_min: Ok(Default::default()),
                step_temp_c: Err("no value supplied for step_temp_c".to_string()),
                step_time_min: Err("no value supplied for step_time_min".to_string()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl CreateMashStepInput {
        pub fn end_temp_c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.end_temp_c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end_temp_c: {e}"));
            self
        }
        pub fn infuse_amount_l<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.infuse_amount_l = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for infuse_amount_l: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn ramp_time_min<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.ramp_time_min = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ramp_time_min: {e}"));
            self
        }
        pub fn step_temp_c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.step_temp_c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for step_temp_c: {e}"));
            self
        }
        pub fn step_time_min<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.step_time_min = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for step_time_min: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateMashStepInput> for super::CreateMashStepInput {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateMashStepInput,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                end_temp_c: value.end_temp_c?,
                infuse_amount_l: value.infuse_amount_l?,
                name: value.name?,
                ramp_time_min: value.ramp_time_min?,
                step_temp_c: value.step_temp_c?,
                step_time_min: value.step_time_min?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::CreateMashStepInput> for CreateMashStepInput {
        fn from(value: super::CreateMashStepInput) -> Self {
            Self {
                end_temp_c: Ok(value.end_temp_c),
                infuse_amount_l: Ok(value.infuse_amount_l),
                name: Ok(value.name),
                ramp_time_min: Ok(value.ramp_time_min),
                step_temp_c: Ok(value.step_temp_c),
                step_time_min: Ok(value.step_time_min),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateMiscAdditionInput {
        amount: ::std::result::Result<f64, ::std::string::String>,
        amount_is_weight: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        misc_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        time_min: ::std::result::Result<f64, ::std::string::String>,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
        use_: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CreateMiscAdditionInput {
        fn default() -> Self {
            Self {
                amount: Err("no value supplied for amount".to_string()),
                amount_is_weight: Ok(Default::default()),
                misc_id: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                time_min: Err("no value supplied for time_min".to_string()),
                type_: Err("no value supplied for type_".to_string()),
                use_: Err("no value supplied for use_".to_string()),
            }
        }
    }
    impl CreateMiscAdditionInput {
        pub fn amount<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.amount = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for amount: {e}"));
            self
        }
        pub fn amount_is_weight<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.amount_is_weight = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for amount_is_weight: {e}"));
            self
        }
        pub fn misc_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.misc_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for misc_id: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn time_min<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.time_min = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for time_min: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
        pub fn use_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.use_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for use_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateMiscAdditionInput> for super::CreateMiscAdditionInput {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateMiscAdditionInput,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                amount: value.amount?,
                amount_is_weight: value.amount_is_weight?,
                misc_id: value.misc_id?,
                name: value.name?,
                time_min: value.time_min?,
                type_: value.type_?,
                use_: value.use_?,
            })
        }
    }
    impl ::std::convert::From<super::CreateMiscAdditionInput> for CreateMiscAdditionInput {
        fn from(value: super::CreateMiscAdditionInput) -> Self {
            Self {
                amount: Ok(value.amount),
                amount_is_weight: Ok(value.amount_is_weight),
                misc_id: Ok(value.misc_id),
                name: Ok(value.name),
                time_min: Ok(value.time_min),
                type_: Ok(value.type_),
                use_: Ok(value.use_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateRecipeInput {
        batch_size_l: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        boil_size_l: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        boil_time_min: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        equipment_profile_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        source_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        style_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        hopstand_temp_c: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for CreateRecipeInput {
        fn default() -> Self {
            Self {
                batch_size_l: Ok(Default::default()),
                boil_size_l: Ok(Default::default()),
                boil_time_min: Ok(Default::default()),
                equipment_profile_id: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                source_id: Ok(Default::default()),
                style_id: Ok(Default::default()),
                type_: Ok(Default::default()),
                hopstand_temp_c: Ok(Default::default()),
            }
        }
    }
    impl CreateRecipeInput {
        pub fn batch_size_l<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.batch_size_l = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for batch_size_l: {e}"));
            self
        }
        pub fn boil_size_l<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.boil_size_l = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for boil_size_l: {e}"));
            self
        }
        pub fn boil_time_min<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.boil_time_min = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for boil_time_min: {e}"));
            self
        }
        pub fn equipment_profile_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.equipment_profile_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for equipment_profile_id: {e}")
            });
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn source_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.source_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source_id: {e}"));
            self
        }
        pub fn style_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.style_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for style_id: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
        pub fn hopstand_temp_c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.hopstand_temp_c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for hopstand_temp_c: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateRecipeInput> for super::CreateRecipeInput {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateRecipeInput,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                batch_size_l: value.batch_size_l?,
                boil_size_l: value.boil_size_l?,
                boil_time_min: value.boil_time_min?,
                equipment_profile_id: value.equipment_profile_id?,
                name: value.name?,
                source_id: value.source_id?,
                style_id: value.style_id?,
                type_: value.type_?,
                hopstand_temp_c: value.hopstand_temp_c?,
            })
        }
    }
    impl ::std::convert::From<super::CreateRecipeInput> for CreateRecipeInput {
        fn from(value: super::CreateRecipeInput) -> Self {
            Self {
                batch_size_l: Ok(value.batch_size_l),
                boil_size_l: Ok(value.boil_size_l),
                boil_time_min: Ok(value.boil_time_min),
                equipment_profile_id: Ok(value.equipment_profile_id),
                name: Ok(value.name),
                source_id: Ok(value.source_id),
                style_id: Ok(value.style_id),
                type_: Ok(value.type_),
                hopstand_temp_c: Ok(value.hopstand_temp_c),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateWaterAdditionInput {
        amount_l: ::std::result::Result<f64, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        water_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreateWaterAdditionInput {
        fn default() -> Self {
            Self {
                amount_l: Err("no value supplied for amount_l".to_string()),
                name: Err("no value supplied for name".to_string()),
                water_id: Ok(Default::default()),
            }
        }
    }
    impl CreateWaterAdditionInput {
        pub fn amount_l<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.amount_l = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for amount_l: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn water_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.water_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for water_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateWaterAdditionInput> for super::CreateWaterAdditionInput {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateWaterAdditionInput,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                amount_l: value.amount_l?,
                name: value.name?,
                water_id: value.water_id?,
            })
        }
    }
    impl ::std::convert::From<super::CreateWaterAdditionInput> for CreateWaterAdditionInput {
        fn from(value: super::CreateWaterAdditionInput) -> Self {
            Self {
                amount_l: Ok(value.amount_l),
                name: Ok(value.name),
                water_id: Ok(value.water_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateWaterAdjustmentInput {
        addition:
            ::std::result::Result<super::CreateWaterAdjustmentInputAddition, ::std::string::String>,
        amount: ::std::result::Result<f64, ::std::string::String>,
        target:
            ::std::result::Result<super::CreateWaterAdjustmentInputTarget, ::std::string::String>,
    }
    impl ::std::default::Default for CreateWaterAdjustmentInput {
        fn default() -> Self {
            Self {
                addition: Err("no value supplied for addition".to_string()),
                amount: Err("no value supplied for amount".to_string()),
                target: Err("no value supplied for target".to_string()),
            }
        }
    }
    impl CreateWaterAdjustmentInput {
        pub fn addition<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateWaterAdjustmentInputAddition>,
            T::Error: ::std::fmt::Display,
        {
            self.addition = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for addition: {e}"));
            self
        }
        pub fn amount<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.amount = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for amount: {e}"));
            self
        }
        pub fn target<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateWaterAdjustmentInputTarget>,
            T::Error: ::std::fmt::Display,
        {
            self.target = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for target: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateWaterAdjustmentInput> for super::CreateWaterAdjustmentInput {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateWaterAdjustmentInput,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                addition: value.addition?,
                amount: value.amount?,
                target: value.target?,
            })
        }
    }
    impl ::std::convert::From<super::CreateWaterAdjustmentInput> for CreateWaterAdjustmentInput {
        fn from(value: super::CreateWaterAdjustmentInput) -> Self {
            Self {
                addition: Ok(value.addition),
                amount: Ok(value.amount),
                target: Ok(value.target),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateYeastAdditionInput {
        add_to_secondary: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        amount: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        amount_is_weight: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        attenuation_pct: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        form: ::std::result::Result<::std::string::String, ::std::string::String>,
        laboratory: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        product_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        times_cultured: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
        yeast_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreateYeastAdditionInput {
        fn default() -> Self {
            Self {
                add_to_secondary: Ok(Default::default()),
                amount: Ok(Default::default()),
                amount_is_weight: Ok(Default::default()),
                attenuation_pct: Ok(Default::default()),
                form: Err("no value supplied for form".to_string()),
                laboratory: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                product_id: Ok(Default::default()),
                times_cultured: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
                yeast_id: Ok(Default::default()),
            }
        }
    }
    impl CreateYeastAdditionInput {
        pub fn add_to_secondary<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.add_to_secondary = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for add_to_secondary: {e}"));
            self
        }
        pub fn amount<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.amount = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for amount: {e}"));
            self
        }
        pub fn amount_is_weight<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.amount_is_weight = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for amount_is_weight: {e}"));
            self
        }
        pub fn attenuation_pct<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.attenuation_pct = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for attenuation_pct: {e}"));
            self
        }
        pub fn form<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.form = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for form: {e}"));
            self
        }
        pub fn laboratory<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.laboratory = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for laboratory: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn product_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.product_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for product_id: {e}"));
            self
        }
        pub fn times_cultured<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.times_cultured = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for times_cultured: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
        pub fn yeast_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.yeast_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for yeast_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateYeastAdditionInput> for super::CreateYeastAdditionInput {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateYeastAdditionInput,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                add_to_secondary: value.add_to_secondary?,
                amount: value.amount?,
                amount_is_weight: value.amount_is_weight?,
                attenuation_pct: value.attenuation_pct?,
                form: value.form?,
                laboratory: value.laboratory?,
                name: value.name?,
                product_id: value.product_id?,
                times_cultured: value.times_cultured?,
                type_: value.type_?,
                yeast_id: value.yeast_id?,
            })
        }
    }
    impl ::std::convert::From<super::CreateYeastAdditionInput> for CreateYeastAdditionInput {
        fn from(value: super::CreateYeastAdditionInput) -> Self {
            Self {
                add_to_secondary: Ok(value.add_to_secondary),
                amount: Ok(value.amount),
                amount_is_weight: Ok(value.amount_is_weight),
                attenuation_pct: Ok(value.attenuation_pct),
                form: Ok(value.form),
                laboratory: Ok(value.laboratory),
                name: Ok(value.name),
                product_id: Ok(value.product_id),
                times_cultured: Ok(value.times_cultured),
                type_: Ok(value.type_),
                yeast_id: Ok(value.yeast_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EquipmentProfile {
        batch_size_l: ::std::result::Result<f64, ::std::string::String>,
        boil_size_l: ::std::result::Result<f64, ::std::string::String>,
        boil_time_min: ::std::result::Result<f64, ::std::string::String>,
        calc_boil_volume: ::std::result::Result<bool, ::std::string::String>,
        created_at: ::std::result::Result<i64, ::std::string::String>,
        efficiency_pct: ::std::result::Result<f64, ::std::string::String>,
        evap_rate_pct_hr: ::std::result::Result<f64, ::std::string::String>,
        fermenter_loss_l: ::std::result::Result<f64, ::std::string::String>,
        hop_utilization_pct: ::std::result::Result<f64, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        lauter_deadspace_l: ::std::result::Result<f64, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        notes: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        top_up_kettle_l: ::std::result::Result<f64, ::std::string::String>,
        top_up_water_l: ::std::result::Result<f64, ::std::string::String>,
        trub_chiller_loss_l: ::std::result::Result<f64, ::std::string::String>,
        tun_specific_heat: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        tun_volume_l: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        tun_weight_kg: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        updated_at: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for EquipmentProfile {
        fn default() -> Self {
            Self {
                batch_size_l: Err("no value supplied for batch_size_l".to_string()),
                boil_size_l: Err("no value supplied for boil_size_l".to_string()),
                boil_time_min: Err("no value supplied for boil_time_min".to_string()),
                calc_boil_volume: Err("no value supplied for calc_boil_volume".to_string()),
                created_at: Err("no value supplied for created_at".to_string()),
                efficiency_pct: Err("no value supplied for efficiency_pct".to_string()),
                evap_rate_pct_hr: Err("no value supplied for evap_rate_pct_hr".to_string()),
                fermenter_loss_l: Err("no value supplied for fermenter_loss_l".to_string()),
                hop_utilization_pct: Err("no value supplied for hop_utilization_pct".to_string()),
                id: Err("no value supplied for id".to_string()),
                lauter_deadspace_l: Err("no value supplied for lauter_deadspace_l".to_string()),
                name: Err("no value supplied for name".to_string()),
                notes: Ok(Default::default()),
                top_up_kettle_l: Err("no value supplied for top_up_kettle_l".to_string()),
                top_up_water_l: Err("no value supplied for top_up_water_l".to_string()),
                trub_chiller_loss_l: Err("no value supplied for trub_chiller_loss_l".to_string()),
                tun_specific_heat: Ok(Default::default()),
                tun_volume_l: Ok(Default::default()),
                tun_weight_kg: Ok(Default::default()),
                updated_at: Err("no value supplied for updated_at".to_string()),
            }
        }
    }
    impl EquipmentProfile {
        pub fn batch_size_l<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.batch_size_l = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for batch_size_l: {e}"));
            self
        }
        pub fn boil_size_l<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.boil_size_l = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for boil_size_l: {e}"));
            self
        }
        pub fn boil_time_min<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.boil_time_min = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for boil_time_min: {e}"));
            self
        }
        pub fn calc_boil_volume<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.calc_boil_volume = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for calc_boil_volume: {e}"));
            self
        }
        pub fn created_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.created_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for created_at: {e}"));
            self
        }
        pub fn efficiency_pct<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.efficiency_pct = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for efficiency_pct: {e}"));
            self
        }
        pub fn evap_rate_pct_hr<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.evap_rate_pct_hr = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for evap_rate_pct_hr: {e}"));
            self
        }
        pub fn fermenter_loss_l<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.fermenter_loss_l = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for fermenter_loss_l: {e}"));
            self
        }
        pub fn hop_utilization_pct<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.hop_utilization_pct = value.try_into().map_err(|e| {
                format!("error converting supplied value for hop_utilization_pct: {e}")
            });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn lauter_deadspace_l<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.lauter_deadspace_l = value.try_into().map_err(|e| {
                format!("error converting supplied value for lauter_deadspace_l: {e}")
            });
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn notes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.notes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for notes: {e}"));
            self
        }
        pub fn top_up_kettle_l<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.top_up_kettle_l = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for top_up_kettle_l: {e}"));
            self
        }
        pub fn top_up_water_l<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.top_up_water_l = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for top_up_water_l: {e}"));
            self
        }
        pub fn trub_chiller_loss_l<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.trub_chiller_loss_l = value.try_into().map_err(|e| {
                format!("error converting supplied value for trub_chiller_loss_l: {e}")
            });
            self
        }
        pub fn tun_specific_heat<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.tun_specific_heat = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tun_specific_heat: {e}"));
            self
        }
        pub fn tun_volume_l<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.tun_volume_l = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tun_volume_l: {e}"));
            self
        }
        pub fn tun_weight_kg<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.tun_weight_kg = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tun_weight_kg: {e}"));
            self
        }
        pub fn updated_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.updated_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for updated_at: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<EquipmentProfile> for super::EquipmentProfile {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EquipmentProfile,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                batch_size_l: value.batch_size_l?,
                boil_size_l: value.boil_size_l?,
                boil_time_min: value.boil_time_min?,
                calc_boil_volume: value.calc_boil_volume?,
                created_at: value.created_at?,
                efficiency_pct: value.efficiency_pct?,
                evap_rate_pct_hr: value.evap_rate_pct_hr?,
                fermenter_loss_l: value.fermenter_loss_l?,
                hop_utilization_pct: value.hop_utilization_pct?,
                id: value.id?,
                lauter_deadspace_l: value.lauter_deadspace_l?,
                name: value.name?,
                notes: value.notes?,
                top_up_kettle_l: value.top_up_kettle_l?,
                top_up_water_l: value.top_up_water_l?,
                trub_chiller_loss_l: value.trub_chiller_loss_l?,
                tun_specific_heat: value.tun_specific_heat?,
                tun_volume_l: value.tun_volume_l?,
                tun_weight_kg: value.tun_weight_kg?,
                updated_at: value.updated_at?,
            })
        }
    }
    impl ::std::convert::From<super::EquipmentProfile> for EquipmentProfile {
        fn from(value: super::EquipmentProfile) -> Self {
            Self {
                batch_size_l: Ok(value.batch_size_l),
                boil_size_l: Ok(value.boil_size_l),
                boil_time_min: Ok(value.boil_time_min),
                calc_boil_volume: Ok(value.calc_boil_volume),
                created_at: Ok(value.created_at),
                efficiency_pct: Ok(value.efficiency_pct),
                evap_rate_pct_hr: Ok(value.evap_rate_pct_hr),
                fermenter_loss_l: Ok(value.fermenter_loss_l),
                hop_utilization_pct: Ok(value.hop_utilization_pct),
                id: Ok(value.id),
                lauter_deadspace_l: Ok(value.lauter_deadspace_l),
                name: Ok(value.name),
                notes: Ok(value.notes),
                top_up_kettle_l: Ok(value.top_up_kettle_l),
                top_up_water_l: Ok(value.top_up_water_l),
                trub_chiller_loss_l: Ok(value.trub_chiller_loss_l),
                tun_specific_heat: Ok(value.tun_specific_heat),
                tun_volume_l: Ok(value.tun_volume_l),
                tun_weight_kg: Ok(value.tun_weight_kg),
                updated_at: Ok(value.updated_at),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Fermentable {
        add_after_boil: ::std::result::Result<bool, ::std::string::String>,
        coarse_fine_diff_pct:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        color_lovibond: ::std::result::Result<f64, ::std::string::String>,
        diastatic_power_lintner:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        ibu_gal_per_lb: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        max_in_batch_pct: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        moisture_pct: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        notes: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        origin: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        protein_pct: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        recommend_mash: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        supplier: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
        yield_pct: ::std::result::Result<f64, ::std::string::String>,
    }
    impl ::std::default::Default for Fermentable {
        fn default() -> Self {
            Self {
                add_after_boil: Err("no value supplied for add_after_boil".to_string()),
                coarse_fine_diff_pct: Ok(Default::default()),
                color_lovibond: Err("no value supplied for color_lovibond".to_string()),
                diastatic_power_lintner: Ok(Default::default()),
                ibu_gal_per_lb: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                max_in_batch_pct: Ok(Default::default()),
                moisture_pct: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                notes: Ok(Default::default()),
                origin: Ok(Default::default()),
                protein_pct: Ok(Default::default()),
                recommend_mash: Ok(Default::default()),
                supplier: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
                yield_pct: Err("no value supplied for yield_pct".to_string()),
            }
        }
    }
    impl Fermentable {
        pub fn add_after_boil<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.add_after_boil = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for add_after_boil: {e}"));
            self
        }
        pub fn coarse_fine_diff_pct<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.coarse_fine_diff_pct = value.try_into().map_err(|e| {
                format!("error converting supplied value for coarse_fine_diff_pct: {e}")
            });
            self
        }
        pub fn color_lovibond<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.color_lovibond = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for color_lovibond: {e}"));
            self
        }
        pub fn diastatic_power_lintner<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.diastatic_power_lintner = value.try_into().map_err(|e| {
                format!("error converting supplied value for diastatic_power_lintner: {e}")
            });
            self
        }
        pub fn ibu_gal_per_lb<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.ibu_gal_per_lb = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ibu_gal_per_lb: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn max_in_batch_pct<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.max_in_batch_pct = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max_in_batch_pct: {e}"));
            self
        }
        pub fn moisture_pct<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.moisture_pct = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for moisture_pct: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn notes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.notes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for notes: {e}"));
            self
        }
        pub fn origin<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.origin = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for origin: {e}"));
            self
        }
        pub fn protein_pct<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.protein_pct = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for protein_pct: {e}"));
            self
        }
        pub fn recommend_mash<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.recommend_mash = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for recommend_mash: {e}"));
            self
        }
        pub fn supplier<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.supplier = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for supplier: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
        pub fn yield_pct<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.yield_pct = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for yield_pct: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Fermentable> for super::Fermentable {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Fermentable,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                add_after_boil: value.add_after_boil?,
                coarse_fine_diff_pct: value.coarse_fine_diff_pct?,
                color_lovibond: value.color_lovibond?,
                diastatic_power_lintner: value.diastatic_power_lintner?,
                ibu_gal_per_lb: value.ibu_gal_per_lb?,
                id: value.id?,
                max_in_batch_pct: value.max_in_batch_pct?,
                moisture_pct: value.moisture_pct?,
                name: value.name?,
                notes: value.notes?,
                origin: value.origin?,
                protein_pct: value.protein_pct?,
                recommend_mash: value.recommend_mash?,
                supplier: value.supplier?,
                type_: value.type_?,
                yield_pct: value.yield_pct?,
            })
        }
    }
    impl ::std::convert::From<super::Fermentable> for Fermentable {
        fn from(value: super::Fermentable) -> Self {
            Self {
                add_after_boil: Ok(value.add_after_boil),
                coarse_fine_diff_pct: Ok(value.coarse_fine_diff_pct),
                color_lovibond: Ok(value.color_lovibond),
                diastatic_power_lintner: Ok(value.diastatic_power_lintner),
                ibu_gal_per_lb: Ok(value.ibu_gal_per_lb),
                id: Ok(value.id),
                max_in_batch_pct: Ok(value.max_in_batch_pct),
                moisture_pct: Ok(value.moisture_pct),
                name: Ok(value.name),
                notes: Ok(value.notes),
                origin: Ok(value.origin),
                protein_pct: Ok(value.protein_pct),
                recommend_mash: Ok(value.recommend_mash),
                supplier: Ok(value.supplier),
                type_: Ok(value.type_),
                yield_pct: Ok(value.yield_pct),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Hop {
        alpha_pct: ::std::result::Result<f64, ::std::string::String>,
        beta_pct: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        caryophyllene_pct: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        cohumulone_pct: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        form: ::std::result::Result<::std::string::String, ::std::string::String>,
        hsi_pct: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        humulene_pct: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        myrcene_pct: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        notes: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        origin: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        substitutes: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        year: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Hop {
        fn default() -> Self {
            Self {
                alpha_pct: Err("no value supplied for alpha_pct".to_string()),
                beta_pct: Ok(Default::default()),
                caryophyllene_pct: Ok(Default::default()),
                cohumulone_pct: Ok(Default::default()),
                form: Err("no value supplied for form".to_string()),
                hsi_pct: Ok(Default::default()),
                humulene_pct: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                myrcene_pct: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                notes: Ok(Default::default()),
                origin: Ok(Default::default()),
                substitutes: Ok(Default::default()),
                type_: Ok(Default::default()),
                year: Ok(Default::default()),
            }
        }
    }
    impl Hop {
        pub fn alpha_pct<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.alpha_pct = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for alpha_pct: {e}"));
            self
        }
        pub fn beta_pct<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.beta_pct = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for beta_pct: {e}"));
            self
        }
        pub fn caryophyllene_pct<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.caryophyllene_pct = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for caryophyllene_pct: {e}"));
            self
        }
        pub fn cohumulone_pct<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.cohumulone_pct = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cohumulone_pct: {e}"));
            self
        }
        pub fn form<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.form = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for form: {e}"));
            self
        }
        pub fn hsi_pct<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.hsi_pct = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for hsi_pct: {e}"));
            self
        }
        pub fn humulene_pct<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.humulene_pct = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for humulene_pct: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn myrcene_pct<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.myrcene_pct = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for myrcene_pct: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn notes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.notes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for notes: {e}"));
            self
        }
        pub fn origin<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.origin = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for origin: {e}"));
            self
        }
        pub fn substitutes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.substitutes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for substitutes: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
        pub fn year<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.year = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for year: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Hop> for super::Hop {
        type Error = super::error::ConversionError;
        fn try_from(value: Hop) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                alpha_pct: value.alpha_pct?,
                beta_pct: value.beta_pct?,
                caryophyllene_pct: value.caryophyllene_pct?,
                cohumulone_pct: value.cohumulone_pct?,
                form: value.form?,
                hsi_pct: value.hsi_pct?,
                humulene_pct: value.humulene_pct?,
                id: value.id?,
                myrcene_pct: value.myrcene_pct?,
                name: value.name?,
                notes: value.notes?,
                origin: value.origin?,
                substitutes: value.substitutes?,
                type_: value.type_?,
                year: value.year?,
            })
        }
    }
    impl ::std::convert::From<super::Hop> for Hop {
        fn from(value: super::Hop) -> Self {
            Self {
                alpha_pct: Ok(value.alpha_pct),
                beta_pct: Ok(value.beta_pct),
                caryophyllene_pct: Ok(value.caryophyllene_pct),
                cohumulone_pct: Ok(value.cohumulone_pct),
                form: Ok(value.form),
                hsi_pct: Ok(value.hsi_pct),
                humulene_pct: Ok(value.humulene_pct),
                id: Ok(value.id),
                myrcene_pct: Ok(value.myrcene_pct),
                name: Ok(value.name),
                notes: Ok(value.notes),
                origin: Ok(value.origin),
                substitutes: Ok(value.substitutes),
                type_: Ok(value.type_),
                year: Ok(value.year),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Mash {
        equip_adjust: ::std::result::Result<bool, ::std::string::String>,
        grain_temp_c: ::std::result::Result<f64, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        notes: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        ph: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        ratio_l_per_kg: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        recipe_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        sparge_temp_c: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        steps: ::std::result::Result<::std::vec::Vec<super::MashStep>, ::std::string::String>,
        tun_specific_heat: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        tun_temp_c: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        tun_weight_kg: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for Mash {
        fn default() -> Self {
            Self {
                equip_adjust: Err("no value supplied for equip_adjust".to_string()),
                grain_temp_c: Err("no value supplied for grain_temp_c".to_string()),
                id: Err("no value supplied for id".to_string()),
                name: Err("no value supplied for name".to_string()),
                notes: Ok(Default::default()),
                ph: Ok(Default::default()),
                ratio_l_per_kg: Ok(Default::default()),
                recipe_id: Err("no value supplied for recipe_id".to_string()),
                sparge_temp_c: Ok(Default::default()),
                steps: Err("no value supplied for steps".to_string()),
                tun_specific_heat: Ok(Default::default()),
                tun_temp_c: Ok(Default::default()),
                tun_weight_kg: Ok(Default::default()),
            }
        }
    }
    impl Mash {
        pub fn equip_adjust<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.equip_adjust = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for equip_adjust: {e}"));
            self
        }
        pub fn grain_temp_c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.grain_temp_c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for grain_temp_c: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn notes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.notes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for notes: {e}"));
            self
        }
        pub fn ph<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.ph = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ph: {e}"));
            self
        }
        pub fn ratio_l_per_kg<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.ratio_l_per_kg = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ratio_l_per_kg: {e}"));
            self
        }
        pub fn recipe_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.recipe_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for recipe_id: {e}"));
            self
        }
        pub fn sparge_temp_c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.sparge_temp_c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sparge_temp_c: {e}"));
            self
        }
        pub fn steps<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::MashStep>>,
            T::Error: ::std::fmt::Display,
        {
            self.steps = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for steps: {e}"));
            self
        }
        pub fn tun_specific_heat<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.tun_specific_heat = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tun_specific_heat: {e}"));
            self
        }
        pub fn tun_temp_c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.tun_temp_c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tun_temp_c: {e}"));
            self
        }
        pub fn tun_weight_kg<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.tun_weight_kg = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tun_weight_kg: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Mash> for super::Mash {
        type Error = super::error::ConversionError;
        fn try_from(value: Mash) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                equip_adjust: value.equip_adjust?,
                grain_temp_c: value.grain_temp_c?,
                id: value.id?,
                name: value.name?,
                notes: value.notes?,
                ph: value.ph?,
                ratio_l_per_kg: value.ratio_l_per_kg?,
                recipe_id: value.recipe_id?,
                sparge_temp_c: value.sparge_temp_c?,
                steps: value.steps?,
                tun_specific_heat: value.tun_specific_heat?,
                tun_temp_c: value.tun_temp_c?,
                tun_weight_kg: value.tun_weight_kg?,
            })
        }
    }
    impl ::std::convert::From<super::Mash> for Mash {
        fn from(value: super::Mash) -> Self {
            Self {
                equip_adjust: Ok(value.equip_adjust),
                grain_temp_c: Ok(value.grain_temp_c),
                id: Ok(value.id),
                name: Ok(value.name),
                notes: Ok(value.notes),
                ph: Ok(value.ph),
                ratio_l_per_kg: Ok(value.ratio_l_per_kg),
                recipe_id: Ok(value.recipe_id),
                sparge_temp_c: Ok(value.sparge_temp_c),
                steps: Ok(value.steps),
                tun_specific_heat: Ok(value.tun_specific_heat),
                tun_temp_c: Ok(value.tun_temp_c),
                tun_weight_kg: Ok(value.tun_weight_kg),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MashStep {
        end_temp_c: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        infuse_amount_l: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        mash_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        ramp_time_min: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        step_order: ::std::result::Result<i64, ::std::string::String>,
        step_temp_c: ::std::result::Result<f64, ::std::string::String>,
        step_time_min: ::std::result::Result<i64, ::std::string::String>,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for MashStep {
        fn default() -> Self {
            Self {
                end_temp_c: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                infuse_amount_l: Ok(Default::default()),
                mash_id: Err("no value supplied for mash_id".to_string()),
                name: Err("no value supplied for name".to_string()),
                ramp_time_min: Ok(Default::default()),
                step_order: Err("no value supplied for step_order".to_string()),
                step_temp_c: Err("no value supplied for step_temp_c".to_string()),
                step_time_min: Err("no value supplied for step_time_min".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl MashStep {
        pub fn end_temp_c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.end_temp_c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end_temp_c: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn infuse_amount_l<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.infuse_amount_l = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for infuse_amount_l: {e}"));
            self
        }
        pub fn mash_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.mash_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mash_id: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn ramp_time_min<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.ramp_time_min = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ramp_time_min: {e}"));
            self
        }
        pub fn step_order<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.step_order = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for step_order: {e}"));
            self
        }
        pub fn step_temp_c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.step_temp_c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for step_temp_c: {e}"));
            self
        }
        pub fn step_time_min<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.step_time_min = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for step_time_min: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<MashStep> for super::MashStep {
        type Error = super::error::ConversionError;
        fn try_from(value: MashStep) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                end_temp_c: value.end_temp_c?,
                id: value.id?,
                infuse_amount_l: value.infuse_amount_l?,
                mash_id: value.mash_id?,
                name: value.name?,
                ramp_time_min: value.ramp_time_min?,
                step_order: value.step_order?,
                step_temp_c: value.step_temp_c?,
                step_time_min: value.step_time_min?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::MashStep> for MashStep {
        fn from(value: super::MashStep) -> Self {
            Self {
                end_temp_c: Ok(value.end_temp_c),
                id: Ok(value.id),
                infuse_amount_l: Ok(value.infuse_amount_l),
                mash_id: Ok(value.mash_id),
                name: Ok(value.name),
                ramp_time_min: Ok(value.ramp_time_min),
                step_order: Ok(value.step_order),
                step_temp_c: Ok(value.step_temp_c),
                step_time_min: Ok(value.step_time_min),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Misc {
        amount_is_weight: ::std::result::Result<bool, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        notes: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        time_min: ::std::result::Result<f64, ::std::string::String>,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
        use_: ::std::result::Result<::std::string::String, ::std::string::String>,
        use_for: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Misc {
        fn default() -> Self {
            Self {
                amount_is_weight: Err("no value supplied for amount_is_weight".to_string()),
                id: Err("no value supplied for id".to_string()),
                name: Err("no value supplied for name".to_string()),
                notes: Ok(Default::default()),
                time_min: Err("no value supplied for time_min".to_string()),
                type_: Err("no value supplied for type_".to_string()),
                use_: Err("no value supplied for use_".to_string()),
                use_for: Ok(Default::default()),
            }
        }
    }
    impl Misc {
        pub fn amount_is_weight<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.amount_is_weight = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for amount_is_weight: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn notes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.notes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for notes: {e}"));
            self
        }
        pub fn time_min<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.time_min = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for time_min: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
        pub fn use_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.use_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for use_: {e}"));
            self
        }
        pub fn use_for<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.use_for = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for use_for: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Misc> for super::Misc {
        type Error = super::error::ConversionError;
        fn try_from(value: Misc) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                amount_is_weight: value.amount_is_weight?,
                id: value.id?,
                name: value.name?,
                notes: value.notes?,
                time_min: value.time_min?,
                type_: value.type_?,
                use_: value.use_?,
                use_for: value.use_for?,
            })
        }
    }
    impl ::std::convert::From<super::Misc> for Misc {
        fn from(value: super::Misc) -> Self {
            Self {
                amount_is_weight: Ok(value.amount_is_weight),
                id: Ok(value.id),
                name: Ok(value.name),
                notes: Ok(value.notes),
                time_min: Ok(value.time_min),
                type_: Ok(value.type_),
                use_: Ok(value.use_),
                use_for: Ok(value.use_for),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Recipe {
        age_days: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        age_temp_c: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        asst_brewer: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        batch_size_l: ::std::result::Result<f64, ::std::string::String>,
        boil_size_l: ::std::result::Result<f64, ::std::string::String>,
        boil_time_min: ::std::result::Result<f64, ::std::string::String>,
        brewer: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        carbonation_temp_c:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        carbonation_vols: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        created_at: ::std::result::Result<i64, ::std::string::String>,
        date: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        efficiency_pct: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        equipment_profile: ::std::result::Result<
            ::std::option::Option<super::EquipmentProfile>,
            ::std::string::String,
        >,
        equipment_profile_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        fermentables: ::std::result::Result<
            ::std::vec::Vec<super::RecipeAdditionFermentable>,
            ::std::string::String,
        >,
        fermentation_stages: ::std::result::Result<i64, ::std::string::String>,
        fg: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        forced_carbonation: ::std::result::Result<bool, ::std::string::String>,
        hops:
            ::std::result::Result<::std::vec::Vec<super::RecipeAdditionHop>, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        keg_priming_factor:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        mash: ::std::result::Result<::std::option::Option<super::Mash>, ::std::string::String>,
        mash_water_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        miscs: ::std::result::Result<
            ::std::vec::Vec<super::RecipeAdditionMisc>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        notes: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        og: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        primary_age_days: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        primary_temp_c: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        priming_sugar_equiv:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        priming_sugar_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        secondary_age_days:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        secondary_temp_c: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        sparge_water_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        style: ::std::result::Result<::std::option::Option<super::Style>, ::std::string::String>,
        style_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        taste_notes: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        taste_rating: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        tertiary_age_days: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        tertiary_temp_c: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
        updated_at: ::std::result::Result<i64, ::std::string::String>,
        water_adjustments: ::std::result::Result<
            ::std::vec::Vec<super::RecipeWaterAdjustment>,
            ::std::string::String,
        >,
        waters: ::std::result::Result<
            ::std::vec::Vec<super::RecipeAdditionWater>,
            ::std::string::String,
        >,
        hopstand_temp_c: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        yeasts: ::std::result::Result<
            ::std::vec::Vec<super::RecipeAdditionYeast>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Recipe {
        fn default() -> Self {
            Self {
                age_days: Ok(Default::default()),
                age_temp_c: Ok(Default::default()),
                asst_brewer: Ok(Default::default()),
                batch_size_l: Err("no value supplied for batch_size_l".to_string()),
                boil_size_l: Err("no value supplied for boil_size_l".to_string()),
                boil_time_min: Err("no value supplied for boil_time_min".to_string()),
                brewer: Ok(Default::default()),
                carbonation_temp_c: Ok(Default::default()),
                carbonation_vols: Ok(Default::default()),
                created_at: Err("no value supplied for created_at".to_string()),
                date: Ok(Default::default()),
                efficiency_pct: Ok(Default::default()),
                equipment_profile: Ok(Default::default()),
                equipment_profile_id: Ok(Default::default()),
                fermentables: Err("no value supplied for fermentables".to_string()),
                fermentation_stages: Err("no value supplied for fermentation_stages".to_string()),
                fg: Ok(Default::default()),
                forced_carbonation: Err("no value supplied for forced_carbonation".to_string()),
                hops: Err("no value supplied for hops".to_string()),
                id: Err("no value supplied for id".to_string()),
                keg_priming_factor: Ok(Default::default()),
                mash: Ok(Default::default()),
                mash_water_id: Ok(Default::default()),
                miscs: Err("no value supplied for miscs".to_string()),
                name: Err("no value supplied for name".to_string()),
                notes: Ok(Default::default()),
                og: Ok(Default::default()),
                primary_age_days: Ok(Default::default()),
                primary_temp_c: Ok(Default::default()),
                priming_sugar_equiv: Ok(Default::default()),
                priming_sugar_name: Ok(Default::default()),
                secondary_age_days: Ok(Default::default()),
                secondary_temp_c: Ok(Default::default()),
                sparge_water_id: Ok(Default::default()),
                style: Ok(Default::default()),
                style_id: Ok(Default::default()),
                taste_notes: Ok(Default::default()),
                taste_rating: Ok(Default::default()),
                tertiary_age_days: Ok(Default::default()),
                tertiary_temp_c: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
                updated_at: Err("no value supplied for updated_at".to_string()),
                water_adjustments: Ok(Default::default()),
                waters: Err("no value supplied for waters".to_string()),
                hopstand_temp_c: Ok(Default::default()),
                yeasts: Err("no value supplied for yeasts".to_string()),
            }
        }
    }
    impl Recipe {
        pub fn age_days<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.age_days = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for age_days: {e}"));
            self
        }
        pub fn age_temp_c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.age_temp_c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for age_temp_c: {e}"));
            self
        }
        pub fn asst_brewer<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.asst_brewer = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for asst_brewer: {e}"));
            self
        }
        pub fn batch_size_l<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.batch_size_l = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for batch_size_l: {e}"));
            self
        }
        pub fn boil_size_l<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.boil_size_l = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for boil_size_l: {e}"));
            self
        }
        pub fn boil_time_min<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.boil_time_min = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for boil_time_min: {e}"));
            self
        }
        pub fn brewer<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.brewer = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for brewer: {e}"));
            self
        }
        pub fn carbonation_temp_c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.carbonation_temp_c = value.try_into().map_err(|e| {
                format!("error converting supplied value for carbonation_temp_c: {e}")
            });
            self
        }
        pub fn carbonation_vols<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.carbonation_vols = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for carbonation_vols: {e}"));
            self
        }
        pub fn created_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.created_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for created_at: {e}"));
            self
        }
        pub fn date<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.date = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for date: {e}"));
            self
        }
        pub fn efficiency_pct<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.efficiency_pct = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for efficiency_pct: {e}"));
            self
        }
        pub fn equipment_profile<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::EquipmentProfile>>,
            T::Error: ::std::fmt::Display,
        {
            self.equipment_profile = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for equipment_profile: {e}"));
            self
        }
        pub fn equipment_profile_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.equipment_profile_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for equipment_profile_id: {e}")
            });
            self
        }
        pub fn fermentables<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::RecipeAdditionFermentable>>,
            T::Error: ::std::fmt::Display,
        {
            self.fermentables = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for fermentables: {e}"));
            self
        }
        pub fn fermentation_stages<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.fermentation_stages = value.try_into().map_err(|e| {
                format!("error converting supplied value for fermentation_stages: {e}")
            });
            self
        }
        pub fn fg<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.fg = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for fg: {e}"));
            self
        }
        pub fn forced_carbonation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.forced_carbonation = value.try_into().map_err(|e| {
                format!("error converting supplied value for forced_carbonation: {e}")
            });
            self
        }
        pub fn hops<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::RecipeAdditionHop>>,
            T::Error: ::std::fmt::Display,
        {
            self.hops = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for hops: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn keg_priming_factor<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.keg_priming_factor = value.try_into().map_err(|e| {
                format!("error converting supplied value for keg_priming_factor: {e}")
            });
            self
        }
        pub fn mash<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Mash>>,
            T::Error: ::std::fmt::Display,
        {
            self.mash = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mash: {e}"));
            self
        }
        pub fn mash_water_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.mash_water_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mash_water_id: {e}"));
            self
        }
        pub fn miscs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::RecipeAdditionMisc>>,
            T::Error: ::std::fmt::Display,
        {
            self.miscs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for miscs: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn notes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.notes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for notes: {e}"));
            self
        }
        pub fn og<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.og = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for og: {e}"));
            self
        }
        pub fn primary_age_days<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.primary_age_days = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for primary_age_days: {e}"));
            self
        }
        pub fn primary_temp_c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.primary_temp_c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for primary_temp_c: {e}"));
            self
        }
        pub fn priming_sugar_equiv<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.priming_sugar_equiv = value.try_into().map_err(|e| {
                format!("error converting supplied value for priming_sugar_equiv: {e}")
            });
            self
        }
        pub fn priming_sugar_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.priming_sugar_name = value.try_into().map_err(|e| {
                format!("error converting supplied value for priming_sugar_name: {e}")
            });
            self
        }
        pub fn secondary_age_days<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.secondary_age_days = value.try_into().map_err(|e| {
                format!("error converting supplied value for secondary_age_days: {e}")
            });
            self
        }
        pub fn secondary_temp_c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.secondary_temp_c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for secondary_temp_c: {e}"));
            self
        }
        pub fn sparge_water_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.sparge_water_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sparge_water_id: {e}"));
            self
        }
        pub fn style<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Style>>,
            T::Error: ::std::fmt::Display,
        {
            self.style = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for style: {e}"));
            self
        }
        pub fn style_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.style_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for style_id: {e}"));
            self
        }
        pub fn taste_notes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.taste_notes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for taste_notes: {e}"));
            self
        }
        pub fn taste_rating<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.taste_rating = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for taste_rating: {e}"));
            self
        }
        pub fn tertiary_age_days<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.tertiary_age_days = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tertiary_age_days: {e}"));
            self
        }
        pub fn tertiary_temp_c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.tertiary_temp_c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tertiary_temp_c: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
        pub fn updated_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.updated_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for updated_at: {e}"));
            self
        }
        pub fn water_adjustments<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::RecipeWaterAdjustment>>,
            T::Error: ::std::fmt::Display,
        {
            self.water_adjustments = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for water_adjustments: {e}"));
            self
        }
        pub fn waters<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::RecipeAdditionWater>>,
            T::Error: ::std::fmt::Display,
        {
            self.waters = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for waters: {e}"));
            self
        }
        pub fn hopstand_temp_c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.hopstand_temp_c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for hopstand_temp_c: {e}"));
            self
        }
        pub fn yeasts<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::RecipeAdditionYeast>>,
            T::Error: ::std::fmt::Display,
        {
            self.yeasts = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for yeasts: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Recipe> for super::Recipe {
        type Error = super::error::ConversionError;
        fn try_from(value: Recipe) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                age_days: value.age_days?,
                age_temp_c: value.age_temp_c?,
                asst_brewer: value.asst_brewer?,
                batch_size_l: value.batch_size_l?,
                boil_size_l: value.boil_size_l?,
                boil_time_min: value.boil_time_min?,
                brewer: value.brewer?,
                carbonation_temp_c: value.carbonation_temp_c?,
                carbonation_vols: value.carbonation_vols?,
                created_at: value.created_at?,
                date: value.date?,
                efficiency_pct: value.efficiency_pct?,
                equipment_profile: value.equipment_profile?,
                equipment_profile_id: value.equipment_profile_id?,
                fermentables: value.fermentables?,
                fermentation_stages: value.fermentation_stages?,
                fg: value.fg?,
                forced_carbonation: value.forced_carbonation?,
                hops: value.hops?,
                id: value.id?,
                keg_priming_factor: value.keg_priming_factor?,
                mash: value.mash?,
                mash_water_id: value.mash_water_id?,
                miscs: value.miscs?,
                name: value.name?,
                notes: value.notes?,
                og: value.og?,
                primary_age_days: value.primary_age_days?,
                primary_temp_c: value.primary_temp_c?,
                priming_sugar_equiv: value.priming_sugar_equiv?,
                priming_sugar_name: value.priming_sugar_name?,
                secondary_age_days: value.secondary_age_days?,
                secondary_temp_c: value.secondary_temp_c?,
                sparge_water_id: value.sparge_water_id?,
                style: value.style?,
                style_id: value.style_id?,
                taste_notes: value.taste_notes?,
                taste_rating: value.taste_rating?,
                tertiary_age_days: value.tertiary_age_days?,
                tertiary_temp_c: value.tertiary_temp_c?,
                type_: value.type_?,
                updated_at: value.updated_at?,
                water_adjustments: value.water_adjustments?,
                waters: value.waters?,
                hopstand_temp_c: value.hopstand_temp_c?,
                yeasts: value.yeasts?,
            })
        }
    }
    impl ::std::convert::From<super::Recipe> for Recipe {
        fn from(value: super::Recipe) -> Self {
            Self {
                age_days: Ok(value.age_days),
                age_temp_c: Ok(value.age_temp_c),
                asst_brewer: Ok(value.asst_brewer),
                batch_size_l: Ok(value.batch_size_l),
                boil_size_l: Ok(value.boil_size_l),
                boil_time_min: Ok(value.boil_time_min),
                brewer: Ok(value.brewer),
                carbonation_temp_c: Ok(value.carbonation_temp_c),
                carbonation_vols: Ok(value.carbonation_vols),
                created_at: Ok(value.created_at),
                date: Ok(value.date),
                efficiency_pct: Ok(value.efficiency_pct),
                equipment_profile: Ok(value.equipment_profile),
                equipment_profile_id: Ok(value.equipment_profile_id),
                fermentables: Ok(value.fermentables),
                fermentation_stages: Ok(value.fermentation_stages),
                fg: Ok(value.fg),
                forced_carbonation: Ok(value.forced_carbonation),
                hops: Ok(value.hops),
                id: Ok(value.id),
                keg_priming_factor: Ok(value.keg_priming_factor),
                mash: Ok(value.mash),
                mash_water_id: Ok(value.mash_water_id),
                miscs: Ok(value.miscs),
                name: Ok(value.name),
                notes: Ok(value.notes),
                og: Ok(value.og),
                primary_age_days: Ok(value.primary_age_days),
                primary_temp_c: Ok(value.primary_temp_c),
                priming_sugar_equiv: Ok(value.priming_sugar_equiv),
                priming_sugar_name: Ok(value.priming_sugar_name),
                secondary_age_days: Ok(value.secondary_age_days),
                secondary_temp_c: Ok(value.secondary_temp_c),
                sparge_water_id: Ok(value.sparge_water_id),
                style: Ok(value.style),
                style_id: Ok(value.style_id),
                taste_notes: Ok(value.taste_notes),
                taste_rating: Ok(value.taste_rating),
                tertiary_age_days: Ok(value.tertiary_age_days),
                tertiary_temp_c: Ok(value.tertiary_temp_c),
                type_: Ok(value.type_),
                updated_at: Ok(value.updated_at),
                water_adjustments: Ok(value.water_adjustments),
                waters: Ok(value.waters),
                hopstand_temp_c: Ok(value.hopstand_temp_c),
                yeasts: Ok(value.yeasts),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RecipeAdditionFermentable {
        add_after_boil: ::std::result::Result<bool, ::std::string::String>,
        addition_order: ::std::result::Result<i64, ::std::string::String>,
        amount_kg: ::std::result::Result<f64, ::std::string::String>,
        color_lovibond: ::std::result::Result<f64, ::std::string::String>,
        fermentable_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        recipe_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
        yield_pct: ::std::result::Result<f64, ::std::string::String>,
    }
    impl ::std::default::Default for RecipeAdditionFermentable {
        fn default() -> Self {
            Self {
                add_after_boil: Err("no value supplied for add_after_boil".to_string()),
                addition_order: Err("no value supplied for addition_order".to_string()),
                amount_kg: Err("no value supplied for amount_kg".to_string()),
                color_lovibond: Err("no value supplied for color_lovibond".to_string()),
                fermentable_id: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                name: Err("no value supplied for name".to_string()),
                recipe_id: Err("no value supplied for recipe_id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
                yield_pct: Err("no value supplied for yield_pct".to_string()),
            }
        }
    }
    impl RecipeAdditionFermentable {
        pub fn add_after_boil<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.add_after_boil = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for add_after_boil: {e}"));
            self
        }
        pub fn addition_order<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.addition_order = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for addition_order: {e}"));
            self
        }
        pub fn amount_kg<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.amount_kg = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for amount_kg: {e}"));
            self
        }
        pub fn color_lovibond<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.color_lovibond = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for color_lovibond: {e}"));
            self
        }
        pub fn fermentable_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.fermentable_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for fermentable_id: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn recipe_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.recipe_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for recipe_id: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
        pub fn yield_pct<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.yield_pct = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for yield_pct: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RecipeAdditionFermentable> for super::RecipeAdditionFermentable {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RecipeAdditionFermentable,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                add_after_boil: value.add_after_boil?,
                addition_order: value.addition_order?,
                amount_kg: value.amount_kg?,
                color_lovibond: value.color_lovibond?,
                fermentable_id: value.fermentable_id?,
                id: value.id?,
                name: value.name?,
                recipe_id: value.recipe_id?,
                type_: value.type_?,
                yield_pct: value.yield_pct?,
            })
        }
    }
    impl ::std::convert::From<super::RecipeAdditionFermentable> for RecipeAdditionFermentable {
        fn from(value: super::RecipeAdditionFermentable) -> Self {
            Self {
                add_after_boil: Ok(value.add_after_boil),
                addition_order: Ok(value.addition_order),
                amount_kg: Ok(value.amount_kg),
                color_lovibond: Ok(value.color_lovibond),
                fermentable_id: Ok(value.fermentable_id),
                id: Ok(value.id),
                name: Ok(value.name),
                recipe_id: Ok(value.recipe_id),
                type_: Ok(value.type_),
                yield_pct: Ok(value.yield_pct),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RecipeAdditionHop {
        addition_order: ::std::result::Result<i64, ::std::string::String>,
        alpha_pct: ::std::result::Result<f64, ::std::string::String>,
        amount_kg: ::std::result::Result<f64, ::std::string::String>,
        form: ::std::result::Result<::std::string::String, ::std::string::String>,
        hop_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        recipe_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        time_min: ::std::result::Result<f64, ::std::string::String>,
        use_: ::std::result::Result<::std::string::String, ::std::string::String>,
        hopstand_temp_c: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for RecipeAdditionHop {
        fn default() -> Self {
            Self {
                addition_order: Err("no value supplied for addition_order".to_string()),
                alpha_pct: Err("no value supplied for alpha_pct".to_string()),
                amount_kg: Err("no value supplied for amount_kg".to_string()),
                form: Err("no value supplied for form".to_string()),
                hop_id: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                name: Err("no value supplied for name".to_string()),
                recipe_id: Err("no value supplied for recipe_id".to_string()),
                time_min: Err("no value supplied for time_min".to_string()),
                use_: Err("no value supplied for use_".to_string()),
                hopstand_temp_c: Ok(Default::default()),
            }
        }
    }
    impl RecipeAdditionHop {
        pub fn addition_order<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.addition_order = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for addition_order: {e}"));
            self
        }
        pub fn alpha_pct<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.alpha_pct = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for alpha_pct: {e}"));
            self
        }
        pub fn amount_kg<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.amount_kg = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for amount_kg: {e}"));
            self
        }
        pub fn form<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.form = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for form: {e}"));
            self
        }
        pub fn hop_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.hop_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for hop_id: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn recipe_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.recipe_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for recipe_id: {e}"));
            self
        }
        pub fn time_min<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.time_min = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for time_min: {e}"));
            self
        }
        pub fn use_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.use_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for use_: {e}"));
            self
        }
        pub fn hopstand_temp_c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.hopstand_temp_c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for hopstand_temp_c: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RecipeAdditionHop> for super::RecipeAdditionHop {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RecipeAdditionHop,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                addition_order: value.addition_order?,
                alpha_pct: value.alpha_pct?,
                amount_kg: value.amount_kg?,
                form: value.form?,
                hop_id: value.hop_id?,
                id: value.id?,
                name: value.name?,
                recipe_id: value.recipe_id?,
                time_min: value.time_min?,
                use_: value.use_?,
                hopstand_temp_c: value.hopstand_temp_c?,
            })
        }
    }
    impl ::std::convert::From<super::RecipeAdditionHop> for RecipeAdditionHop {
        fn from(value: super::RecipeAdditionHop) -> Self {
            Self {
                addition_order: Ok(value.addition_order),
                alpha_pct: Ok(value.alpha_pct),
                amount_kg: Ok(value.amount_kg),
                form: Ok(value.form),
                hop_id: Ok(value.hop_id),
                id: Ok(value.id),
                name: Ok(value.name),
                recipe_id: Ok(value.recipe_id),
                time_min: Ok(value.time_min),
                use_: Ok(value.use_),
                hopstand_temp_c: Ok(value.hopstand_temp_c),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RecipeAdditionMisc {
        addition_order: ::std::result::Result<i64, ::std::string::String>,
        amount: ::std::result::Result<f64, ::std::string::String>,
        amount_is_weight: ::std::result::Result<bool, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        misc_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        recipe_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        time_min: ::std::result::Result<f64, ::std::string::String>,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
        use_: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for RecipeAdditionMisc {
        fn default() -> Self {
            Self {
                addition_order: Err("no value supplied for addition_order".to_string()),
                amount: Err("no value supplied for amount".to_string()),
                amount_is_weight: Err("no value supplied for amount_is_weight".to_string()),
                id: Err("no value supplied for id".to_string()),
                misc_id: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                recipe_id: Err("no value supplied for recipe_id".to_string()),
                time_min: Err("no value supplied for time_min".to_string()),
                type_: Err("no value supplied for type_".to_string()),
                use_: Err("no value supplied for use_".to_string()),
            }
        }
    }
    impl RecipeAdditionMisc {
        pub fn addition_order<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.addition_order = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for addition_order: {e}"));
            self
        }
        pub fn amount<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.amount = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for amount: {e}"));
            self
        }
        pub fn amount_is_weight<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.amount_is_weight = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for amount_is_weight: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn misc_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.misc_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for misc_id: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn recipe_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.recipe_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for recipe_id: {e}"));
            self
        }
        pub fn time_min<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.time_min = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for time_min: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
        pub fn use_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.use_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for use_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RecipeAdditionMisc> for super::RecipeAdditionMisc {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RecipeAdditionMisc,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                addition_order: value.addition_order?,
                amount: value.amount?,
                amount_is_weight: value.amount_is_weight?,
                id: value.id?,
                misc_id: value.misc_id?,
                name: value.name?,
                recipe_id: value.recipe_id?,
                time_min: value.time_min?,
                type_: value.type_?,
                use_: value.use_?,
            })
        }
    }
    impl ::std::convert::From<super::RecipeAdditionMisc> for RecipeAdditionMisc {
        fn from(value: super::RecipeAdditionMisc) -> Self {
            Self {
                addition_order: Ok(value.addition_order),
                amount: Ok(value.amount),
                amount_is_weight: Ok(value.amount_is_weight),
                id: Ok(value.id),
                misc_id: Ok(value.misc_id),
                name: Ok(value.name),
                recipe_id: Ok(value.recipe_id),
                time_min: Ok(value.time_min),
                type_: Ok(value.type_),
                use_: Ok(value.use_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RecipeAdditionWater {
        amount_l: ::std::result::Result<f64, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        recipe_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        water_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for RecipeAdditionWater {
        fn default() -> Self {
            Self {
                amount_l: Err("no value supplied for amount_l".to_string()),
                id: Err("no value supplied for id".to_string()),
                name: Err("no value supplied for name".to_string()),
                recipe_id: Err("no value supplied for recipe_id".to_string()),
                water_id: Ok(Default::default()),
            }
        }
    }
    impl RecipeAdditionWater {
        pub fn amount_l<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.amount_l = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for amount_l: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn recipe_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.recipe_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for recipe_id: {e}"));
            self
        }
        pub fn water_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.water_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for water_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RecipeAdditionWater> for super::RecipeAdditionWater {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RecipeAdditionWater,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                amount_l: value.amount_l?,
                id: value.id?,
                name: value.name?,
                recipe_id: value.recipe_id?,
                water_id: value.water_id?,
            })
        }
    }
    impl ::std::convert::From<super::RecipeAdditionWater> for RecipeAdditionWater {
        fn from(value: super::RecipeAdditionWater) -> Self {
            Self {
                amount_l: Ok(value.amount_l),
                id: Ok(value.id),
                name: Ok(value.name),
                recipe_id: Ok(value.recipe_id),
                water_id: Ok(value.water_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RecipeAdditionYeast {
        add_to_secondary: ::std::result::Result<bool, ::std::string::String>,
        amount: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        amount_is_weight: ::std::result::Result<bool, ::std::string::String>,
        attenuation_pct: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        form: ::std::result::Result<::std::string::String, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        laboratory: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        product_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        recipe_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        times_cultured: ::std::result::Result<i64, ::std::string::String>,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
        yeast_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for RecipeAdditionYeast {
        fn default() -> Self {
            Self {
                add_to_secondary: Err("no value supplied for add_to_secondary".to_string()),
                amount: Ok(Default::default()),
                amount_is_weight: Err("no value supplied for amount_is_weight".to_string()),
                attenuation_pct: Ok(Default::default()),
                form: Err("no value supplied for form".to_string()),
                id: Err("no value supplied for id".to_string()),
                laboratory: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                product_id: Ok(Default::default()),
                recipe_id: Err("no value supplied for recipe_id".to_string()),
                times_cultured: Err("no value supplied for times_cultured".to_string()),
                type_: Err("no value supplied for type_".to_string()),
                yeast_id: Ok(Default::default()),
            }
        }
    }
    impl RecipeAdditionYeast {
        pub fn add_to_secondary<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.add_to_secondary = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for add_to_secondary: {e}"));
            self
        }
        pub fn amount<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.amount = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for amount: {e}"));
            self
        }
        pub fn amount_is_weight<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.amount_is_weight = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for amount_is_weight: {e}"));
            self
        }
        pub fn attenuation_pct<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.attenuation_pct = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for attenuation_pct: {e}"));
            self
        }
        pub fn form<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.form = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for form: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn laboratory<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.laboratory = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for laboratory: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn product_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.product_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for product_id: {e}"));
            self
        }
        pub fn recipe_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.recipe_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for recipe_id: {e}"));
            self
        }
        pub fn times_cultured<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.times_cultured = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for times_cultured: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
        pub fn yeast_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.yeast_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for yeast_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RecipeAdditionYeast> for super::RecipeAdditionYeast {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RecipeAdditionYeast,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                add_to_secondary: value.add_to_secondary?,
                amount: value.amount?,
                amount_is_weight: value.amount_is_weight?,
                attenuation_pct: value.attenuation_pct?,
                form: value.form?,
                id: value.id?,
                laboratory: value.laboratory?,
                name: value.name?,
                product_id: value.product_id?,
                recipe_id: value.recipe_id?,
                times_cultured: value.times_cultured?,
                type_: value.type_?,
                yeast_id: value.yeast_id?,
            })
        }
    }
    impl ::std::convert::From<super::RecipeAdditionYeast> for RecipeAdditionYeast {
        fn from(value: super::RecipeAdditionYeast) -> Self {
            Self {
                add_to_secondary: Ok(value.add_to_secondary),
                amount: Ok(value.amount),
                amount_is_weight: Ok(value.amount_is_weight),
                attenuation_pct: Ok(value.attenuation_pct),
                form: Ok(value.form),
                id: Ok(value.id),
                laboratory: Ok(value.laboratory),
                name: Ok(value.name),
                product_id: Ok(value.product_id),
                recipe_id: Ok(value.recipe_id),
                times_cultured: Ok(value.times_cultured),
                type_: Ok(value.type_),
                yeast_id: Ok(value.yeast_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RecipeStats {
        abv_pct: ::std::result::Result<f64, ::std::string::String>,
        bu_gu_ratio: ::std::result::Result<f64, ::std::string::String>,
        calories_per_355ml: ::std::result::Result<f64, ::std::string::String>,
        fg: ::std::result::Result<f64, ::std::string::String>,
        ibu: ::std::result::Result<f64, ::std::string::String>,
        og: ::std::result::Result<f64, ::std::string::String>,
        post_boil_volume_l: ::std::result::Result<f64, ::std::string::String>,
        pre_boil_gravity: ::std::result::Result<f64, ::std::string::String>,
        pre_boil_volume_l: ::std::result::Result<f64, ::std::string::String>,
        srm: ::std::result::Result<f64, ::std::string::String>,
        strike_temp_c: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for RecipeStats {
        fn default() -> Self {
            Self {
                abv_pct: Err("no value supplied for abv_pct".to_string()),
                bu_gu_ratio: Err("no value supplied for bu_gu_ratio".to_string()),
                calories_per_355ml: Err("no value supplied for calories_per_355ml".to_string()),
                fg: Err("no value supplied for fg".to_string()),
                ibu: Err("no value supplied for ibu".to_string()),
                og: Err("no value supplied for og".to_string()),
                post_boil_volume_l: Err("no value supplied for post_boil_volume_l".to_string()),
                pre_boil_gravity: Err("no value supplied for pre_boil_gravity".to_string()),
                pre_boil_volume_l: Err("no value supplied for pre_boil_volume_l".to_string()),
                srm: Err("no value supplied for srm".to_string()),
                strike_temp_c: Ok(Default::default()),
            }
        }
    }
    impl RecipeStats {
        pub fn abv_pct<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.abv_pct = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for abv_pct: {e}"));
            self
        }
        pub fn bu_gu_ratio<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.bu_gu_ratio = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bu_gu_ratio: {e}"));
            self
        }
        pub fn calories_per_355ml<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.calories_per_355ml = value.try_into().map_err(|e| {
                format!("error converting supplied value for calories_per_355ml: {e}")
            });
            self
        }
        pub fn fg<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.fg = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for fg: {e}"));
            self
        }
        pub fn ibu<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.ibu = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ibu: {e}"));
            self
        }
        pub fn og<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.og = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for og: {e}"));
            self
        }
        pub fn post_boil_volume_l<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.post_boil_volume_l = value.try_into().map_err(|e| {
                format!("error converting supplied value for post_boil_volume_l: {e}")
            });
            self
        }
        pub fn pre_boil_gravity<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.pre_boil_gravity = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pre_boil_gravity: {e}"));
            self
        }
        pub fn pre_boil_volume_l<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.pre_boil_volume_l = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pre_boil_volume_l: {e}"));
            self
        }
        pub fn srm<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.srm = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for srm: {e}"));
            self
        }
        pub fn strike_temp_c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.strike_temp_c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for strike_temp_c: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RecipeStats> for super::RecipeStats {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RecipeStats,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                abv_pct: value.abv_pct?,
                bu_gu_ratio: value.bu_gu_ratio?,
                calories_per_355ml: value.calories_per_355ml?,
                fg: value.fg?,
                ibu: value.ibu?,
                og: value.og?,
                post_boil_volume_l: value.post_boil_volume_l?,
                pre_boil_gravity: value.pre_boil_gravity?,
                pre_boil_volume_l: value.pre_boil_volume_l?,
                srm: value.srm?,
                strike_temp_c: value.strike_temp_c?,
            })
        }
    }
    impl ::std::convert::From<super::RecipeStats> for RecipeStats {
        fn from(value: super::RecipeStats) -> Self {
            Self {
                abv_pct: Ok(value.abv_pct),
                bu_gu_ratio: Ok(value.bu_gu_ratio),
                calories_per_355ml: Ok(value.calories_per_355ml),
                fg: Ok(value.fg),
                ibu: Ok(value.ibu),
                og: Ok(value.og),
                post_boil_volume_l: Ok(value.post_boil_volume_l),
                pre_boil_gravity: Ok(value.pre_boil_gravity),
                pre_boil_volume_l: Ok(value.pre_boil_volume_l),
                srm: Ok(value.srm),
                strike_temp_c: Ok(value.strike_temp_c),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RecipeSummary {
        batch_size_l: ::std::result::Result<f64, ::std::string::String>,
        created_at: ::std::result::Result<i64, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        style_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
        updated_at: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for RecipeSummary {
        fn default() -> Self {
            Self {
                batch_size_l: Err("no value supplied for batch_size_l".to_string()),
                created_at: Err("no value supplied for created_at".to_string()),
                id: Err("no value supplied for id".to_string()),
                name: Err("no value supplied for name".to_string()),
                style_name: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
                updated_at: Err("no value supplied for updated_at".to_string()),
            }
        }
    }
    impl RecipeSummary {
        pub fn batch_size_l<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.batch_size_l = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for batch_size_l: {e}"));
            self
        }
        pub fn created_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.created_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for created_at: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn style_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.style_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for style_name: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
        pub fn updated_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.updated_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for updated_at: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RecipeSummary> for super::RecipeSummary {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RecipeSummary,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                batch_size_l: value.batch_size_l?,
                created_at: value.created_at?,
                id: value.id?,
                name: value.name?,
                style_name: value.style_name?,
                type_: value.type_?,
                updated_at: value.updated_at?,
            })
        }
    }
    impl ::std::convert::From<super::RecipeSummary> for RecipeSummary {
        fn from(value: super::RecipeSummary) -> Self {
            Self {
                batch_size_l: Ok(value.batch_size_l),
                created_at: Ok(value.created_at),
                id: Ok(value.id),
                name: Ok(value.name),
                style_name: Ok(value.style_name),
                type_: Ok(value.type_),
                updated_at: Ok(value.updated_at),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RecipeWaterAdjustment {
        addition:
            ::std::result::Result<super::RecipeWaterAdjustmentAddition, ::std::string::String>,
        amount: ::std::result::Result<f64, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        recipe_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        target: ::std::result::Result<super::RecipeWaterAdjustmentTarget, ::std::string::String>,
    }
    impl ::std::default::Default for RecipeWaterAdjustment {
        fn default() -> Self {
            Self {
                addition: Err("no value supplied for addition".to_string()),
                amount: Err("no value supplied for amount".to_string()),
                id: Err("no value supplied for id".to_string()),
                recipe_id: Err("no value supplied for recipe_id".to_string()),
                target: Err("no value supplied for target".to_string()),
            }
        }
    }
    impl RecipeWaterAdjustment {
        pub fn addition<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RecipeWaterAdjustmentAddition>,
            T::Error: ::std::fmt::Display,
        {
            self.addition = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for addition: {e}"));
            self
        }
        pub fn amount<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.amount = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for amount: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn recipe_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.recipe_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for recipe_id: {e}"));
            self
        }
        pub fn target<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RecipeWaterAdjustmentTarget>,
            T::Error: ::std::fmt::Display,
        {
            self.target = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for target: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RecipeWaterAdjustment> for super::RecipeWaterAdjustment {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RecipeWaterAdjustment,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                addition: value.addition?,
                amount: value.amount?,
                id: value.id?,
                recipe_id: value.recipe_id?,
                target: value.target?,
            })
        }
    }
    impl ::std::convert::From<super::RecipeWaterAdjustment> for RecipeWaterAdjustment {
        fn from(value: super::RecipeWaterAdjustment) -> Self {
            Self {
                addition: Ok(value.addition),
                amount: Ok(value.amount),
                id: Ok(value.id),
                recipe_id: Ok(value.recipe_id),
                target: Ok(value.target),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Style {
        abv_max_pct: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        abv_min_pct: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        carb_max_vols: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        carb_min_vols: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        category: ::std::result::Result<::std::string::String, ::std::string::String>,
        category_number: ::std::result::Result<::std::string::String, ::std::string::String>,
        color_max_srm: ::std::result::Result<f64, ::std::string::String>,
        color_min_srm: ::std::result::Result<f64, ::std::string::String>,
        examples: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        fg_max: ::std::result::Result<f64, ::std::string::String>,
        fg_min: ::std::result::Result<f64, ::std::string::String>,
        ibu_max: ::std::result::Result<f64, ::std::string::String>,
        ibu_min: ::std::result::Result<f64, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        ingredients: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        notes: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        og_max: ::std::result::Result<f64, ::std::string::String>,
        og_min: ::std::result::Result<f64, ::std::string::String>,
        profile: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        style_guide: ::std::result::Result<::std::string::String, ::std::string::String>,
        style_letter: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for Style {
        fn default() -> Self {
            Self {
                abv_max_pct: Ok(Default::default()),
                abv_min_pct: Ok(Default::default()),
                carb_max_vols: Ok(Default::default()),
                carb_min_vols: Ok(Default::default()),
                category: Err("no value supplied for category".to_string()),
                category_number: Err("no value supplied for category_number".to_string()),
                color_max_srm: Err("no value supplied for color_max_srm".to_string()),
                color_min_srm: Err("no value supplied for color_min_srm".to_string()),
                examples: Ok(Default::default()),
                fg_max: Err("no value supplied for fg_max".to_string()),
                fg_min: Err("no value supplied for fg_min".to_string()),
                ibu_max: Err("no value supplied for ibu_max".to_string()),
                ibu_min: Err("no value supplied for ibu_min".to_string()),
                id: Err("no value supplied for id".to_string()),
                ingredients: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                notes: Ok(Default::default()),
                og_max: Err("no value supplied for og_max".to_string()),
                og_min: Err("no value supplied for og_min".to_string()),
                profile: Ok(Default::default()),
                style_guide: Err("no value supplied for style_guide".to_string()),
                style_letter: Err("no value supplied for style_letter".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl Style {
        pub fn abv_max_pct<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.abv_max_pct = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for abv_max_pct: {e}"));
            self
        }
        pub fn abv_min_pct<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.abv_min_pct = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for abv_min_pct: {e}"));
            self
        }
        pub fn carb_max_vols<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.carb_max_vols = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for carb_max_vols: {e}"));
            self
        }
        pub fn carb_min_vols<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.carb_min_vols = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for carb_min_vols: {e}"));
            self
        }
        pub fn category<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.category = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for category: {e}"));
            self
        }
        pub fn category_number<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.category_number = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for category_number: {e}"));
            self
        }
        pub fn color_max_srm<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.color_max_srm = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for color_max_srm: {e}"));
            self
        }
        pub fn color_min_srm<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.color_min_srm = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for color_min_srm: {e}"));
            self
        }
        pub fn examples<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.examples = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for examples: {e}"));
            self
        }
        pub fn fg_max<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.fg_max = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for fg_max: {e}"));
            self
        }
        pub fn fg_min<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.fg_min = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for fg_min: {e}"));
            self
        }
        pub fn ibu_max<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.ibu_max = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ibu_max: {e}"));
            self
        }
        pub fn ibu_min<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.ibu_min = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ibu_min: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn ingredients<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.ingredients = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ingredients: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn notes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.notes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for notes: {e}"));
            self
        }
        pub fn og_max<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.og_max = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for og_max: {e}"));
            self
        }
        pub fn og_min<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.og_min = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for og_min: {e}"));
            self
        }
        pub fn profile<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.profile = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for profile: {e}"));
            self
        }
        pub fn style_guide<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.style_guide = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for style_guide: {e}"));
            self
        }
        pub fn style_letter<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.style_letter = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for style_letter: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Style> for super::Style {
        type Error = super::error::ConversionError;
        fn try_from(value: Style) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                abv_max_pct: value.abv_max_pct?,
                abv_min_pct: value.abv_min_pct?,
                carb_max_vols: value.carb_max_vols?,
                carb_min_vols: value.carb_min_vols?,
                category: value.category?,
                category_number: value.category_number?,
                color_max_srm: value.color_max_srm?,
                color_min_srm: value.color_min_srm?,
                examples: value.examples?,
                fg_max: value.fg_max?,
                fg_min: value.fg_min?,
                ibu_max: value.ibu_max?,
                ibu_min: value.ibu_min?,
                id: value.id?,
                ingredients: value.ingredients?,
                name: value.name?,
                notes: value.notes?,
                og_max: value.og_max?,
                og_min: value.og_min?,
                profile: value.profile?,
                style_guide: value.style_guide?,
                style_letter: value.style_letter?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::Style> for Style {
        fn from(value: super::Style) -> Self {
            Self {
                abv_max_pct: Ok(value.abv_max_pct),
                abv_min_pct: Ok(value.abv_min_pct),
                carb_max_vols: Ok(value.carb_max_vols),
                carb_min_vols: Ok(value.carb_min_vols),
                category: Ok(value.category),
                category_number: Ok(value.category_number),
                color_max_srm: Ok(value.color_max_srm),
                color_min_srm: Ok(value.color_min_srm),
                examples: Ok(value.examples),
                fg_max: Ok(value.fg_max),
                fg_min: Ok(value.fg_min),
                ibu_max: Ok(value.ibu_max),
                ibu_min: Ok(value.ibu_min),
                id: Ok(value.id),
                ingredients: Ok(value.ingredients),
                name: Ok(value.name),
                notes: Ok(value.notes),
                og_max: Ok(value.og_max),
                og_min: Ok(value.og_min),
                profile: Ok(value.profile),
                style_guide: Ok(value.style_guide),
                style_letter: Ok(value.style_letter),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UpdateEquipmentProfileInput {
        batch_size_l: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        boil_size_l: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        boil_time_min: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        efficiency_pct: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        evap_rate_pct_hr: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        fermenter_loss_l: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        notes: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        trub_chiller_loss_l:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for UpdateEquipmentProfileInput {
        fn default() -> Self {
            Self {
                batch_size_l: Ok(Default::default()),
                boil_size_l: Ok(Default::default()),
                boil_time_min: Ok(Default::default()),
                efficiency_pct: Ok(Default::default()),
                evap_rate_pct_hr: Ok(Default::default()),
                fermenter_loss_l: Ok(Default::default()),
                name: Ok(Default::default()),
                notes: Ok(Default::default()),
                trub_chiller_loss_l: Ok(Default::default()),
            }
        }
    }
    impl UpdateEquipmentProfileInput {
        pub fn batch_size_l<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.batch_size_l = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for batch_size_l: {e}"));
            self
        }
        pub fn boil_size_l<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.boil_size_l = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for boil_size_l: {e}"));
            self
        }
        pub fn boil_time_min<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.boil_time_min = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for boil_time_min: {e}"));
            self
        }
        pub fn efficiency_pct<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.efficiency_pct = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for efficiency_pct: {e}"));
            self
        }
        pub fn evap_rate_pct_hr<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.evap_rate_pct_hr = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for evap_rate_pct_hr: {e}"));
            self
        }
        pub fn fermenter_loss_l<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.fermenter_loss_l = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for fermenter_loss_l: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn notes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.notes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for notes: {e}"));
            self
        }
        pub fn trub_chiller_loss_l<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.trub_chiller_loss_l = value.try_into().map_err(|e| {
                format!("error converting supplied value for trub_chiller_loss_l: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<UpdateEquipmentProfileInput> for super::UpdateEquipmentProfileInput {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UpdateEquipmentProfileInput,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                batch_size_l: value.batch_size_l?,
                boil_size_l: value.boil_size_l?,
                boil_time_min: value.boil_time_min?,
                efficiency_pct: value.efficiency_pct?,
                evap_rate_pct_hr: value.evap_rate_pct_hr?,
                fermenter_loss_l: value.fermenter_loss_l?,
                name: value.name?,
                notes: value.notes?,
                trub_chiller_loss_l: value.trub_chiller_loss_l?,
            })
        }
    }
    impl ::std::convert::From<super::UpdateEquipmentProfileInput> for UpdateEquipmentProfileInput {
        fn from(value: super::UpdateEquipmentProfileInput) -> Self {
            Self {
                batch_size_l: Ok(value.batch_size_l),
                boil_size_l: Ok(value.boil_size_l),
                boil_time_min: Ok(value.boil_time_min),
                efficiency_pct: Ok(value.efficiency_pct),
                evap_rate_pct_hr: Ok(value.evap_rate_pct_hr),
                fermenter_loss_l: Ok(value.fermenter_loss_l),
                name: Ok(value.name),
                notes: Ok(value.notes),
                trub_chiller_loss_l: Ok(value.trub_chiller_loss_l),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UpdateFermentableAdditionInput {
        add_after_boil: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        addition_order: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        amount_kg: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for UpdateFermentableAdditionInput {
        fn default() -> Self {
            Self {
                add_after_boil: Ok(Default::default()),
                addition_order: Ok(Default::default()),
                amount_kg: Ok(Default::default()),
            }
        }
    }
    impl UpdateFermentableAdditionInput {
        pub fn add_after_boil<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.add_after_boil = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for add_after_boil: {e}"));
            self
        }
        pub fn addition_order<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.addition_order = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for addition_order: {e}"));
            self
        }
        pub fn amount_kg<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.amount_kg = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for amount_kg: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UpdateFermentableAdditionInput>
        for super::UpdateFermentableAdditionInput
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UpdateFermentableAdditionInput,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                add_after_boil: value.add_after_boil?,
                addition_order: value.addition_order?,
                amount_kg: value.amount_kg?,
            })
        }
    }
    impl ::std::convert::From<super::UpdateFermentableAdditionInput>
        for UpdateFermentableAdditionInput
    {
        fn from(value: super::UpdateFermentableAdditionInput) -> Self {
            Self {
                add_after_boil: Ok(value.add_after_boil),
                addition_order: Ok(value.addition_order),
                amount_kg: Ok(value.amount_kg),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UpdateHopAdditionInput {
        addition_order: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        amount_kg: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        time_min: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        use_: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        hopstand_temp_c: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for UpdateHopAdditionInput {
        fn default() -> Self {
            Self {
                addition_order: Ok(Default::default()),
                amount_kg: Ok(Default::default()),
                time_min: Ok(Default::default()),
                use_: Ok(Default::default()),
                hopstand_temp_c: Ok(Default::default()),
            }
        }
    }
    impl UpdateHopAdditionInput {
        pub fn addition_order<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.addition_order = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for addition_order: {e}"));
            self
        }
        pub fn amount_kg<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.amount_kg = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for amount_kg: {e}"));
            self
        }
        pub fn time_min<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.time_min = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for time_min: {e}"));
            self
        }
        pub fn use_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.use_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for use_: {e}"));
            self
        }
        pub fn hopstand_temp_c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.hopstand_temp_c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for hopstand_temp_c: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UpdateHopAdditionInput> for super::UpdateHopAdditionInput {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UpdateHopAdditionInput,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                addition_order: value.addition_order?,
                amount_kg: value.amount_kg?,
                time_min: value.time_min?,
                use_: value.use_?,
                hopstand_temp_c: value.hopstand_temp_c?,
            })
        }
    }
    impl ::std::convert::From<super::UpdateHopAdditionInput> for UpdateHopAdditionInput {
        fn from(value: super::UpdateHopAdditionInput) -> Self {
            Self {
                addition_order: Ok(value.addition_order),
                amount_kg: Ok(value.amount_kg),
                time_min: Ok(value.time_min),
                use_: Ok(value.use_),
                hopstand_temp_c: Ok(value.hopstand_temp_c),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UpdateMashInput {
        grain_temp_c: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        notes: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        ph: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        ratio_l_per_kg: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        sparge_temp_c: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        tun_temp_c: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for UpdateMashInput {
        fn default() -> Self {
            Self {
                grain_temp_c: Ok(Default::default()),
                name: Ok(Default::default()),
                notes: Ok(Default::default()),
                ph: Ok(Default::default()),
                ratio_l_per_kg: Ok(Default::default()),
                sparge_temp_c: Ok(Default::default()),
                tun_temp_c: Ok(Default::default()),
            }
        }
    }
    impl UpdateMashInput {
        pub fn grain_temp_c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.grain_temp_c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for grain_temp_c: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn notes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.notes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for notes: {e}"));
            self
        }
        pub fn ph<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.ph = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ph: {e}"));
            self
        }
        pub fn ratio_l_per_kg<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.ratio_l_per_kg = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ratio_l_per_kg: {e}"));
            self
        }
        pub fn sparge_temp_c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.sparge_temp_c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sparge_temp_c: {e}"));
            self
        }
        pub fn tun_temp_c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.tun_temp_c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tun_temp_c: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UpdateMashInput> for super::UpdateMashInput {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UpdateMashInput,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                grain_temp_c: value.grain_temp_c?,
                name: value.name?,
                notes: value.notes?,
                ph: value.ph?,
                ratio_l_per_kg: value.ratio_l_per_kg?,
                sparge_temp_c: value.sparge_temp_c?,
                tun_temp_c: value.tun_temp_c?,
            })
        }
    }
    impl ::std::convert::From<super::UpdateMashInput> for UpdateMashInput {
        fn from(value: super::UpdateMashInput) -> Self {
            Self {
                grain_temp_c: Ok(value.grain_temp_c),
                name: Ok(value.name),
                notes: Ok(value.notes),
                ph: Ok(value.ph),
                ratio_l_per_kg: Ok(value.ratio_l_per_kg),
                sparge_temp_c: Ok(value.sparge_temp_c),
                tun_temp_c: Ok(value.tun_temp_c),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UpdateMashStepInput {
        end_temp_c: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        infuse_amount_l: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        ramp_time_min: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        step_temp_c: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        step_time_min: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        type_: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for UpdateMashStepInput {
        fn default() -> Self {
            Self {
                end_temp_c: Ok(Default::default()),
                infuse_amount_l: Ok(Default::default()),
                name: Ok(Default::default()),
                ramp_time_min: Ok(Default::default()),
                step_temp_c: Ok(Default::default()),
                step_time_min: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl UpdateMashStepInput {
        pub fn end_temp_c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.end_temp_c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end_temp_c: {e}"));
            self
        }
        pub fn infuse_amount_l<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.infuse_amount_l = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for infuse_amount_l: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn ramp_time_min<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.ramp_time_min = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ramp_time_min: {e}"));
            self
        }
        pub fn step_temp_c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.step_temp_c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for step_temp_c: {e}"));
            self
        }
        pub fn step_time_min<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.step_time_min = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for step_time_min: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UpdateMashStepInput> for super::UpdateMashStepInput {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UpdateMashStepInput,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                end_temp_c: value.end_temp_c?,
                infuse_amount_l: value.infuse_amount_l?,
                name: value.name?,
                ramp_time_min: value.ramp_time_min?,
                step_temp_c: value.step_temp_c?,
                step_time_min: value.step_time_min?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::UpdateMashStepInput> for UpdateMashStepInput {
        fn from(value: super::UpdateMashStepInput) -> Self {
            Self {
                end_temp_c: Ok(value.end_temp_c),
                infuse_amount_l: Ok(value.infuse_amount_l),
                name: Ok(value.name),
                ramp_time_min: Ok(value.ramp_time_min),
                step_temp_c: Ok(value.step_temp_c),
                step_time_min: Ok(value.step_time_min),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UpdateMiscAdditionInput {
        addition_order: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        amount: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        amount_is_weight: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        time_min: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        use_: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for UpdateMiscAdditionInput {
        fn default() -> Self {
            Self {
                addition_order: Ok(Default::default()),
                amount: Ok(Default::default()),
                amount_is_weight: Ok(Default::default()),
                time_min: Ok(Default::default()),
                use_: Ok(Default::default()),
            }
        }
    }
    impl UpdateMiscAdditionInput {
        pub fn addition_order<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.addition_order = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for addition_order: {e}"));
            self
        }
        pub fn amount<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.amount = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for amount: {e}"));
            self
        }
        pub fn amount_is_weight<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.amount_is_weight = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for amount_is_weight: {e}"));
            self
        }
        pub fn time_min<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.time_min = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for time_min: {e}"));
            self
        }
        pub fn use_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.use_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for use_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UpdateMiscAdditionInput> for super::UpdateMiscAdditionInput {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UpdateMiscAdditionInput,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                addition_order: value.addition_order?,
                amount: value.amount?,
                amount_is_weight: value.amount_is_weight?,
                time_min: value.time_min?,
                use_: value.use_?,
            })
        }
    }
    impl ::std::convert::From<super::UpdateMiscAdditionInput> for UpdateMiscAdditionInput {
        fn from(value: super::UpdateMiscAdditionInput) -> Self {
            Self {
                addition_order: Ok(value.addition_order),
                amount: Ok(value.amount),
                amount_is_weight: Ok(value.amount_is_weight),
                time_min: Ok(value.time_min),
                use_: Ok(value.use_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UpdateRecipeInput {
        age_days: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        age_temp_c: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        asst_brewer: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        batch_size_l: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        boil_size_l: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        boil_time_min: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        brewer: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        carbonation_temp_c:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        carbonation_vols: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        date: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        efficiency_pct: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        equipment_profile_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        fermentation_stages:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        forced_carbonation:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        keg_priming_factor:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        notes: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        primary_age_days: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        primary_temp_c: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        priming_sugar_equiv:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        priming_sugar_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        secondary_age_days:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        secondary_temp_c: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        style_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        taste_notes: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        taste_rating: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        tertiary_age_days: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        tertiary_temp_c: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        type_: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        hopstand_temp_c: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for UpdateRecipeInput {
        fn default() -> Self {
            Self {
                age_days: Ok(Default::default()),
                age_temp_c: Ok(Default::default()),
                asst_brewer: Ok(Default::default()),
                batch_size_l: Ok(Default::default()),
                boil_size_l: Ok(Default::default()),
                boil_time_min: Ok(Default::default()),
                brewer: Ok(Default::default()),
                carbonation_temp_c: Ok(Default::default()),
                carbonation_vols: Ok(Default::default()),
                date: Ok(Default::default()),
                efficiency_pct: Ok(Default::default()),
                equipment_profile_id: Ok(Default::default()),
                fermentation_stages: Ok(Default::default()),
                forced_carbonation: Ok(Default::default()),
                keg_priming_factor: Ok(Default::default()),
                name: Ok(Default::default()),
                notes: Ok(Default::default()),
                primary_age_days: Ok(Default::default()),
                primary_temp_c: Ok(Default::default()),
                priming_sugar_equiv: Ok(Default::default()),
                priming_sugar_name: Ok(Default::default()),
                secondary_age_days: Ok(Default::default()),
                secondary_temp_c: Ok(Default::default()),
                style_id: Ok(Default::default()),
                taste_notes: Ok(Default::default()),
                taste_rating: Ok(Default::default()),
                tertiary_age_days: Ok(Default::default()),
                tertiary_temp_c: Ok(Default::default()),
                type_: Ok(Default::default()),
                hopstand_temp_c: Ok(Default::default()),
            }
        }
    }
    impl UpdateRecipeInput {
        pub fn age_days<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.age_days = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for age_days: {e}"));
            self
        }
        pub fn age_temp_c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.age_temp_c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for age_temp_c: {e}"));
            self
        }
        pub fn asst_brewer<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.asst_brewer = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for asst_brewer: {e}"));
            self
        }
        pub fn batch_size_l<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.batch_size_l = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for batch_size_l: {e}"));
            self
        }
        pub fn boil_size_l<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.boil_size_l = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for boil_size_l: {e}"));
            self
        }
        pub fn boil_time_min<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.boil_time_min = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for boil_time_min: {e}"));
            self
        }
        pub fn brewer<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.brewer = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for brewer: {e}"));
            self
        }
        pub fn carbonation_temp_c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.carbonation_temp_c = value.try_into().map_err(|e| {
                format!("error converting supplied value for carbonation_temp_c: {e}")
            });
            self
        }
        pub fn carbonation_vols<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.carbonation_vols = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for carbonation_vols: {e}"));
            self
        }
        pub fn date<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.date = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for date: {e}"));
            self
        }
        pub fn efficiency_pct<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.efficiency_pct = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for efficiency_pct: {e}"));
            self
        }
        pub fn equipment_profile_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.equipment_profile_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for equipment_profile_id: {e}")
            });
            self
        }
        pub fn fermentation_stages<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.fermentation_stages = value.try_into().map_err(|e| {
                format!("error converting supplied value for fermentation_stages: {e}")
            });
            self
        }
        pub fn forced_carbonation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.forced_carbonation = value.try_into().map_err(|e| {
                format!("error converting supplied value for forced_carbonation: {e}")
            });
            self
        }
        pub fn keg_priming_factor<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.keg_priming_factor = value.try_into().map_err(|e| {
                format!("error converting supplied value for keg_priming_factor: {e}")
            });
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn notes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.notes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for notes: {e}"));
            self
        }
        pub fn primary_age_days<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.primary_age_days = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for primary_age_days: {e}"));
            self
        }
        pub fn primary_temp_c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.primary_temp_c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for primary_temp_c: {e}"));
            self
        }
        pub fn priming_sugar_equiv<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.priming_sugar_equiv = value.try_into().map_err(|e| {
                format!("error converting supplied value for priming_sugar_equiv: {e}")
            });
            self
        }
        pub fn priming_sugar_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.priming_sugar_name = value.try_into().map_err(|e| {
                format!("error converting supplied value for priming_sugar_name: {e}")
            });
            self
        }
        pub fn secondary_age_days<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.secondary_age_days = value.try_into().map_err(|e| {
                format!("error converting supplied value for secondary_age_days: {e}")
            });
            self
        }
        pub fn secondary_temp_c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.secondary_temp_c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for secondary_temp_c: {e}"));
            self
        }
        pub fn style_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.style_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for style_id: {e}"));
            self
        }
        pub fn taste_notes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.taste_notes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for taste_notes: {e}"));
            self
        }
        pub fn taste_rating<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.taste_rating = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for taste_rating: {e}"));
            self
        }
        pub fn tertiary_age_days<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.tertiary_age_days = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tertiary_age_days: {e}"));
            self
        }
        pub fn tertiary_temp_c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.tertiary_temp_c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tertiary_temp_c: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
        pub fn hopstand_temp_c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.hopstand_temp_c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for hopstand_temp_c: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UpdateRecipeInput> for super::UpdateRecipeInput {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UpdateRecipeInput,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                age_days: value.age_days?,
                age_temp_c: value.age_temp_c?,
                asst_brewer: value.asst_brewer?,
                batch_size_l: value.batch_size_l?,
                boil_size_l: value.boil_size_l?,
                boil_time_min: value.boil_time_min?,
                brewer: value.brewer?,
                carbonation_temp_c: value.carbonation_temp_c?,
                carbonation_vols: value.carbonation_vols?,
                date: value.date?,
                efficiency_pct: value.efficiency_pct?,
                equipment_profile_id: value.equipment_profile_id?,
                fermentation_stages: value.fermentation_stages?,
                forced_carbonation: value.forced_carbonation?,
                keg_priming_factor: value.keg_priming_factor?,
                name: value.name?,
                notes: value.notes?,
                primary_age_days: value.primary_age_days?,
                primary_temp_c: value.primary_temp_c?,
                priming_sugar_equiv: value.priming_sugar_equiv?,
                priming_sugar_name: value.priming_sugar_name?,
                secondary_age_days: value.secondary_age_days?,
                secondary_temp_c: value.secondary_temp_c?,
                style_id: value.style_id?,
                taste_notes: value.taste_notes?,
                taste_rating: value.taste_rating?,
                tertiary_age_days: value.tertiary_age_days?,
                tertiary_temp_c: value.tertiary_temp_c?,
                type_: value.type_?,
                hopstand_temp_c: value.hopstand_temp_c?,
            })
        }
    }
    impl ::std::convert::From<super::UpdateRecipeInput> for UpdateRecipeInput {
        fn from(value: super::UpdateRecipeInput) -> Self {
            Self {
                age_days: Ok(value.age_days),
                age_temp_c: Ok(value.age_temp_c),
                asst_brewer: Ok(value.asst_brewer),
                batch_size_l: Ok(value.batch_size_l),
                boil_size_l: Ok(value.boil_size_l),
                boil_time_min: Ok(value.boil_time_min),
                brewer: Ok(value.brewer),
                carbonation_temp_c: Ok(value.carbonation_temp_c),
                carbonation_vols: Ok(value.carbonation_vols),
                date: Ok(value.date),
                efficiency_pct: Ok(value.efficiency_pct),
                equipment_profile_id: Ok(value.equipment_profile_id),
                fermentation_stages: Ok(value.fermentation_stages),
                forced_carbonation: Ok(value.forced_carbonation),
                keg_priming_factor: Ok(value.keg_priming_factor),
                name: Ok(value.name),
                notes: Ok(value.notes),
                primary_age_days: Ok(value.primary_age_days),
                primary_temp_c: Ok(value.primary_temp_c),
                priming_sugar_equiv: Ok(value.priming_sugar_equiv),
                priming_sugar_name: Ok(value.priming_sugar_name),
                secondary_age_days: Ok(value.secondary_age_days),
                secondary_temp_c: Ok(value.secondary_temp_c),
                style_id: Ok(value.style_id),
                taste_notes: Ok(value.taste_notes),
                taste_rating: Ok(value.taste_rating),
                tertiary_age_days: Ok(value.tertiary_age_days),
                tertiary_temp_c: Ok(value.tertiary_temp_c),
                type_: Ok(value.type_),
                hopstand_temp_c: Ok(value.hopstand_temp_c),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UpdateWaterAdditionInput {
        amount_l: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for UpdateWaterAdditionInput {
        fn default() -> Self {
            Self {
                amount_l: Ok(Default::default()),
            }
        }
    }
    impl UpdateWaterAdditionInput {
        pub fn amount_l<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.amount_l = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for amount_l: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UpdateWaterAdditionInput> for super::UpdateWaterAdditionInput {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UpdateWaterAdditionInput,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                amount_l: value.amount_l?,
            })
        }
    }
    impl ::std::convert::From<super::UpdateWaterAdditionInput> for UpdateWaterAdditionInput {
        fn from(value: super::UpdateWaterAdditionInput) -> Self {
            Self {
                amount_l: Ok(value.amount_l),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UpdateWaterAdjustmentInput {
        addition: ::std::result::Result<
            ::std::option::Option<super::UpdateWaterAdjustmentInputAddition>,
            ::std::string::String,
        >,
        amount: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        target: ::std::result::Result<
            ::std::option::Option<super::UpdateWaterAdjustmentInputTarget>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for UpdateWaterAdjustmentInput {
        fn default() -> Self {
            Self {
                addition: Ok(Default::default()),
                amount: Ok(Default::default()),
                target: Ok(Default::default()),
            }
        }
    }
    impl UpdateWaterAdjustmentInput {
        pub fn addition<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::UpdateWaterAdjustmentInputAddition>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.addition = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for addition: {e}"));
            self
        }
        pub fn amount<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.amount = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for amount: {e}"));
            self
        }
        pub fn target<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::UpdateWaterAdjustmentInputTarget>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.target = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for target: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UpdateWaterAdjustmentInput> for super::UpdateWaterAdjustmentInput {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UpdateWaterAdjustmentInput,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                addition: value.addition?,
                amount: value.amount?,
                target: value.target?,
            })
        }
    }
    impl ::std::convert::From<super::UpdateWaterAdjustmentInput> for UpdateWaterAdjustmentInput {
        fn from(value: super::UpdateWaterAdjustmentInput) -> Self {
            Self {
                addition: Ok(value.addition),
                amount: Ok(value.amount),
                target: Ok(value.target),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UpdateYeastAdditionInput {
        add_to_secondary: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        amount: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        amount_is_weight: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        attenuation_pct: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        times_cultured: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
    }
    impl ::std::default::Default for UpdateYeastAdditionInput {
        fn default() -> Self {
            Self {
                add_to_secondary: Ok(Default::default()),
                amount: Ok(Default::default()),
                amount_is_weight: Ok(Default::default()),
                attenuation_pct: Ok(Default::default()),
                times_cultured: Ok(Default::default()),
            }
        }
    }
    impl UpdateYeastAdditionInput {
        pub fn add_to_secondary<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.add_to_secondary = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for add_to_secondary: {e}"));
            self
        }
        pub fn amount<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.amount = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for amount: {e}"));
            self
        }
        pub fn amount_is_weight<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.amount_is_weight = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for amount_is_weight: {e}"));
            self
        }
        pub fn attenuation_pct<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.attenuation_pct = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for attenuation_pct: {e}"));
            self
        }
        pub fn times_cultured<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.times_cultured = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for times_cultured: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UpdateYeastAdditionInput> for super::UpdateYeastAdditionInput {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UpdateYeastAdditionInput,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                add_to_secondary: value.add_to_secondary?,
                amount: value.amount?,
                amount_is_weight: value.amount_is_weight?,
                attenuation_pct: value.attenuation_pct?,
                times_cultured: value.times_cultured?,
            })
        }
    }
    impl ::std::convert::From<super::UpdateYeastAdditionInput> for UpdateYeastAdditionInput {
        fn from(value: super::UpdateYeastAdditionInput) -> Self {
            Self {
                add_to_secondary: Ok(value.add_to_secondary),
                amount: Ok(value.amount),
                amount_is_weight: Ok(value.amount_is_weight),
                attenuation_pct: Ok(value.attenuation_pct),
                times_cultured: Ok(value.times_cultured),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Water {
        bicarbonate_ppm: ::std::result::Result<f64, ::std::string::String>,
        calcium_ppm: ::std::result::Result<f64, ::std::string::String>,
        chloride_ppm: ::std::result::Result<f64, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        magnesium_ppm: ::std::result::Result<f64, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        notes: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        ph: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        sodium_ppm: ::std::result::Result<f64, ::std::string::String>,
        sulfate_ppm: ::std::result::Result<f64, ::std::string::String>,
    }
    impl ::std::default::Default for Water {
        fn default() -> Self {
            Self {
                bicarbonate_ppm: Err("no value supplied for bicarbonate_ppm".to_string()),
                calcium_ppm: Err("no value supplied for calcium_ppm".to_string()),
                chloride_ppm: Err("no value supplied for chloride_ppm".to_string()),
                id: Err("no value supplied for id".to_string()),
                magnesium_ppm: Err("no value supplied for magnesium_ppm".to_string()),
                name: Err("no value supplied for name".to_string()),
                notes: Ok(Default::default()),
                ph: Ok(Default::default()),
                sodium_ppm: Err("no value supplied for sodium_ppm".to_string()),
                sulfate_ppm: Err("no value supplied for sulfate_ppm".to_string()),
            }
        }
    }
    impl Water {
        pub fn bicarbonate_ppm<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.bicarbonate_ppm = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bicarbonate_ppm: {e}"));
            self
        }
        pub fn calcium_ppm<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.calcium_ppm = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for calcium_ppm: {e}"));
            self
        }
        pub fn chloride_ppm<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.chloride_ppm = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for chloride_ppm: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn magnesium_ppm<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.magnesium_ppm = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for magnesium_ppm: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn notes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.notes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for notes: {e}"));
            self
        }
        pub fn ph<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.ph = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ph: {e}"));
            self
        }
        pub fn sodium_ppm<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.sodium_ppm = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sodium_ppm: {e}"));
            self
        }
        pub fn sulfate_ppm<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.sulfate_ppm = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sulfate_ppm: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Water> for super::Water {
        type Error = super::error::ConversionError;
        fn try_from(value: Water) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                bicarbonate_ppm: value.bicarbonate_ppm?,
                calcium_ppm: value.calcium_ppm?,
                chloride_ppm: value.chloride_ppm?,
                id: value.id?,
                magnesium_ppm: value.magnesium_ppm?,
                name: value.name?,
                notes: value.notes?,
                ph: value.ph?,
                sodium_ppm: value.sodium_ppm?,
                sulfate_ppm: value.sulfate_ppm?,
            })
        }
    }
    impl ::std::convert::From<super::Water> for Water {
        fn from(value: super::Water) -> Self {
            Self {
                bicarbonate_ppm: Ok(value.bicarbonate_ppm),
                calcium_ppm: Ok(value.calcium_ppm),
                chloride_ppm: Ok(value.chloride_ppm),
                id: Ok(value.id),
                magnesium_ppm: Ok(value.magnesium_ppm),
                name: Ok(value.name),
                notes: Ok(value.notes),
                ph: Ok(value.ph),
                sodium_ppm: Ok(value.sodium_ppm),
                sulfate_ppm: Ok(value.sulfate_ppm),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WaterProfile {
        bicarbonate_ppm: ::std::result::Result<f64, ::std::string::String>,
        calcium_ppm: ::std::result::Result<f64, ::std::string::String>,
        chloride_ppm: ::std::result::Result<f64, ::std::string::String>,
        cl_so4_ratio: ::std::result::Result<f64, ::std::string::String>,
        magnesium_ppm: ::std::result::Result<f64, ::std::string::String>,
        sodium_ppm: ::std::result::Result<f64, ::std::string::String>,
        sulfate_ppm: ::std::result::Result<f64, ::std::string::String>,
    }
    impl ::std::default::Default for WaterProfile {
        fn default() -> Self {
            Self {
                bicarbonate_ppm: Err("no value supplied for bicarbonate_ppm".to_string()),
                calcium_ppm: Err("no value supplied for calcium_ppm".to_string()),
                chloride_ppm: Err("no value supplied for chloride_ppm".to_string()),
                cl_so4_ratio: Err("no value supplied for cl_so4_ratio".to_string()),
                magnesium_ppm: Err("no value supplied for magnesium_ppm".to_string()),
                sodium_ppm: Err("no value supplied for sodium_ppm".to_string()),
                sulfate_ppm: Err("no value supplied for sulfate_ppm".to_string()),
            }
        }
    }
    impl WaterProfile {
        pub fn bicarbonate_ppm<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.bicarbonate_ppm = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bicarbonate_ppm: {e}"));
            self
        }
        pub fn calcium_ppm<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.calcium_ppm = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for calcium_ppm: {e}"));
            self
        }
        pub fn chloride_ppm<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.chloride_ppm = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for chloride_ppm: {e}"));
            self
        }
        pub fn cl_so4_ratio<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.cl_so4_ratio = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cl_so4_ratio: {e}"));
            self
        }
        pub fn magnesium_ppm<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.magnesium_ppm = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for magnesium_ppm: {e}"));
            self
        }
        pub fn sodium_ppm<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.sodium_ppm = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sodium_ppm: {e}"));
            self
        }
        pub fn sulfate_ppm<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.sulfate_ppm = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sulfate_ppm: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<WaterProfile> for super::WaterProfile {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WaterProfile,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                bicarbonate_ppm: value.bicarbonate_ppm?,
                calcium_ppm: value.calcium_ppm?,
                chloride_ppm: value.chloride_ppm?,
                cl_so4_ratio: value.cl_so4_ratio?,
                magnesium_ppm: value.magnesium_ppm?,
                sodium_ppm: value.sodium_ppm?,
                sulfate_ppm: value.sulfate_ppm?,
            })
        }
    }
    impl ::std::convert::From<super::WaterProfile> for WaterProfile {
        fn from(value: super::WaterProfile) -> Self {
            Self {
                bicarbonate_ppm: Ok(value.bicarbonate_ppm),
                calcium_ppm: Ok(value.calcium_ppm),
                chloride_ppm: Ok(value.chloride_ppm),
                cl_so4_ratio: Ok(value.cl_so4_ratio),
                magnesium_ppm: Ok(value.magnesium_ppm),
                sodium_ppm: Ok(value.sodium_ppm),
                sulfate_ppm: Ok(value.sulfate_ppm),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Yeast {
        add_to_secondary: ::std::result::Result<bool, ::std::string::String>,
        alcohol_tolerance: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        attenuation_pct: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        best_for: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        flavor_profile: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        flocculation: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        form: ::std::result::Result<::std::string::String, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        laboratory: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        max_attenuation_pct:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        max_reuse: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        max_temperature_c: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        min_attenuation_pct:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        min_temperature_c: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        notes: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        pof_positive: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        product_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        species: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        sta1_positive: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        styles: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        substitutes: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for Yeast {
        fn default() -> Self {
            Self {
                add_to_secondary: Err("no value supplied for add_to_secondary".to_string()),
                alcohol_tolerance: Ok(Default::default()),
                attenuation_pct: Ok(Default::default()),
                best_for: Ok(Default::default()),
                flavor_profile: Ok(Default::default()),
                flocculation: Ok(Default::default()),
                form: Err("no value supplied for form".to_string()),
                id: Err("no value supplied for id".to_string()),
                laboratory: Ok(Default::default()),
                max_attenuation_pct: Ok(Default::default()),
                max_reuse: Ok(Default::default()),
                max_temperature_c: Ok(Default::default()),
                min_attenuation_pct: Ok(Default::default()),
                min_temperature_c: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                notes: Ok(Default::default()),
                pof_positive: Ok(Default::default()),
                product_id: Ok(Default::default()),
                species: Ok(Default::default()),
                sta1_positive: Ok(Default::default()),
                styles: Ok(Default::default()),
                substitutes: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl Yeast {
        pub fn add_to_secondary<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.add_to_secondary = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for add_to_secondary: {e}"));
            self
        }
        pub fn alcohol_tolerance<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.alcohol_tolerance = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for alcohol_tolerance: {e}"));
            self
        }
        pub fn attenuation_pct<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.attenuation_pct = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for attenuation_pct: {e}"));
            self
        }
        pub fn best_for<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.best_for = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for best_for: {e}"));
            self
        }
        pub fn flavor_profile<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.flavor_profile = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for flavor_profile: {e}"));
            self
        }
        pub fn flocculation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.flocculation = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for flocculation: {e}"));
            self
        }
        pub fn form<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.form = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for form: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn laboratory<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.laboratory = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for laboratory: {e}"));
            self
        }
        pub fn max_attenuation_pct<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.max_attenuation_pct = value.try_into().map_err(|e| {
                format!("error converting supplied value for max_attenuation_pct: {e}")
            });
            self
        }
        pub fn max_reuse<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.max_reuse = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max_reuse: {e}"));
            self
        }
        pub fn max_temperature_c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.max_temperature_c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max_temperature_c: {e}"));
            self
        }
        pub fn min_attenuation_pct<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.min_attenuation_pct = value.try_into().map_err(|e| {
                format!("error converting supplied value for min_attenuation_pct: {e}")
            });
            self
        }
        pub fn min_temperature_c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.min_temperature_c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for min_temperature_c: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn notes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.notes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for notes: {e}"));
            self
        }
        pub fn pof_positive<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.pof_positive = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pof_positive: {e}"));
            self
        }
        pub fn product_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.product_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for product_id: {e}"));
            self
        }
        pub fn species<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.species = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for species: {e}"));
            self
        }
        pub fn sta1_positive<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.sta1_positive = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sta1_positive: {e}"));
            self
        }
        pub fn styles<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.styles = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for styles: {e}"));
            self
        }
        pub fn substitutes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.substitutes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for substitutes: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Yeast> for super::Yeast {
        type Error = super::error::ConversionError;
        fn try_from(value: Yeast) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                add_to_secondary: value.add_to_secondary?,
                alcohol_tolerance: value.alcohol_tolerance?,
                attenuation_pct: value.attenuation_pct?,
                best_for: value.best_for?,
                flavor_profile: value.flavor_profile?,
                flocculation: value.flocculation?,
                form: value.form?,
                id: value.id?,
                laboratory: value.laboratory?,
                max_attenuation_pct: value.max_attenuation_pct?,
                max_reuse: value.max_reuse?,
                max_temperature_c: value.max_temperature_c?,
                min_attenuation_pct: value.min_attenuation_pct?,
                min_temperature_c: value.min_temperature_c?,
                name: value.name?,
                notes: value.notes?,
                pof_positive: value.pof_positive?,
                product_id: value.product_id?,
                species: value.species?,
                sta1_positive: value.sta1_positive?,
                styles: value.styles?,
                substitutes: value.substitutes?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::Yeast> for Yeast {
        fn from(value: super::Yeast) -> Self {
            Self {
                add_to_secondary: Ok(value.add_to_secondary),
                alcohol_tolerance: Ok(value.alcohol_tolerance),
                attenuation_pct: Ok(value.attenuation_pct),
                best_for: Ok(value.best_for),
                flavor_profile: Ok(value.flavor_profile),
                flocculation: Ok(value.flocculation),
                form: Ok(value.form),
                id: Ok(value.id),
                laboratory: Ok(value.laboratory),
                max_attenuation_pct: Ok(value.max_attenuation_pct),
                max_reuse: Ok(value.max_reuse),
                max_temperature_c: Ok(value.max_temperature_c),
                min_attenuation_pct: Ok(value.min_attenuation_pct),
                min_temperature_c: Ok(value.min_temperature_c),
                name: Ok(value.name),
                notes: Ok(value.notes),
                pof_positive: Ok(value.pof_positive),
                product_id: Ok(value.product_id),
                species: Ok(value.species),
                sta1_positive: Ok(value.sta1_positive),
                styles: Ok(value.styles),
                substitutes: Ok(value.substitutes),
                type_: Ok(value.type_),
            }
        }
    }
}
