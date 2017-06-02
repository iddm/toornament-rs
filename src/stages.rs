/// A stage number
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct StageNumber(pub i64);

/// Tournament stage type
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StageType {
    /// Group type
    Group,
    /// League type
    League,
    /// Swiss type
    Swiss,
    /// Single-elimination type
    SingleElimination,
    /// Double-elimination type
    DoubleElimination,
    /// Bracket group type
    BracketGroup,
}

/// A tournament stage
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Stage {
    /// Stage number.
    pub number: StageNumber,
    /// Name of this stage.
    pub name: String,
    /// Stage type.
    #[serde(rename = "type")]
    pub stage_type: StageType,
    /// Number of participants of this stage.
    pub size: i64,
}

/// A list of tournament stages
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Stages(pub Vec<Stage>);


#[cfg(test)]
mod tests {
    use ::*;

    #[test]
    fn test_stages_parse() {
        let string = r#"
        [
            {
                "number": 1,
                "name": "Playoffs",
                "type": "single_elimination",
                "size": 8
            }
        ]
        "#;

        let stages: Stages = serde_json::from_str(string).unwrap();

        assert_eq!(stages.0.len(), 1);
        let s = stages.0.first().unwrap().clone();
        assert_eq!(s.number, StageNumber(1i64));
        assert_eq!(s.name, "Playoffs".to_owned());
        assert_eq!(s.stage_type, StageType::SingleElimination);
        assert_eq!(s.size, 8i64);
    }
}
