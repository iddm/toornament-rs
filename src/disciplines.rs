use std::collections::HashMap;

use common::TeamSize;

/// Additional fields for `Discipline` wrap.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AdditionalFields(pub HashMap<String, HashMap<String, String>>);

/// A game discipline identity.
#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct DisciplineId(pub String);

/// A game discipline object.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Discipline {
    /// An identifier for a discipline, can be used in others APIs.
    /// Example: "counterstrike_go"
    pub id: DisciplineId,
    /// The official name of the discipline.
    /// Example: "Counter-Strike: GO"
    pub name: String,
    /// The short name of the discipline.
    /// Example: "CS:GO"
    #[serde(rename = "shortname")]
    pub short_name: String,
    /// The complete name of the discipline.
    /// Example: "Counter-Strike: Global Offensive"
    #[serde(rename = "fullname")]
    pub full_name: String,
    /// The name of the publisher of the discipline or any other right related information about the owner of the discipline.
    /// Example: "Valve Software"
    pub copyrights: String,
    /// (Optional) Sets the minimum and maximum of players in a team.
    /// Example: (4, 8), where `4` is minimal size of a team in the tournament
    /// and `8` is maximal size of a team in the tournament.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_size: Option<TeamSize>,
    /// (Optional) Additional fields concerning the discipline.
    /// Note about the special fields in this API: if the field is mentioned, you must use one of the following values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_fields: Option<AdditionalFields>,
}
impl Discipline {
    /// Creates new `Discipline` object.
    pub fn new<S: Into<String>>(
        id: DisciplineId,
        name: S,
        short_name: S,
        full_name: S,
        copyrights: S,
    ) -> Discipline {
        Discipline {
            id,
            name: name.into(),
            short_name: short_name.into(),
            full_name: full_name.into(),
            copyrights: copyrights.into(),
            team_size: None,
            additional_fields: None,
        }
    }

    builder!(id, DisciplineId);
    builder_s!(name);
    builder_s!(short_name);
    builder_s!(full_name);
    builder_s!(copyrights);
    builder!(team_size, Option<TeamSize>);
    builder!(additional_fields, Option<AdditionalFields>);
}

impl Discipline {
    /// Returns iter for the discipline
    pub fn iter<'a>(&self, client: &'a ::Toornament) -> ::DisciplineIter<'a> {
        ::DisciplineIter::new(client, self.id.clone())
    }

    /// Converts discipline into an iter
    pub fn into_iter(self, client: &::Toornament) -> ::DisciplineIter<'_> {
        ::DisciplineIter::new(client, self.id)
    }
}

/// A list of `Discipline` objects.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Disciplines(pub Vec<Discipline>);

#[cfg(test)]
mod tests {
    extern crate serde_json;
    use {Discipline, DisciplineId, Disciplines};

    #[test]
    fn test_discipline_parse() {
        let string = r#"{
            "id": "counterstrike_go",
            "name": "Counter-Strike: GO",
            "shortname": "CS:GO",
            "fullname": "Counter-Strike: Global Offensive",
            "copyrights": "Valve Software"
        }"#;
        let d: Discipline = serde_json::from_str(string).unwrap();

        assert_eq!(d.id.0, "counterstrike_go");
        assert_eq!(d.name, "Counter-Strike: GO");
        assert_eq!(d.short_name, "CS:GO");
        assert_eq!(d.full_name, "Counter-Strike: Global Offensive");
        assert_eq!(d.copyrights, "Valve Software");
    }

    #[test]
    fn test_discipline_full_parse() {
        let string = r#"{
            "id": "cod4",
            "name": "COD4:MW",
            "shortname": "COD4",
            "fullname": "Call of Duty 4 : Modern Warfare",
            "copyrights": "Infinity Ward / Activision",
            "team_size": {
                "min": 4,
                "max": 4
            },
            "additional_fields": {
                "field_name": {
                    "value": "label"
                }
            }
        }"#;
        let d: Discipline = serde_json::from_str(string).unwrap();

        assert_eq!(d.id.0, "cod4");
        assert_eq!(d.name, "COD4:MW");
        assert_eq!(d.short_name, "COD4");
        assert_eq!(d.full_name, "Call of Duty 4 : Modern Warfare");
        assert_eq!(d.copyrights, r#"Infinity Ward / Activision"#);
        assert!(d.team_size.is_some());
        let ts = d.team_size.unwrap(); // safe
        assert_eq!(ts.min, 4i64);
        assert_eq!(ts.max, 4i64);
        assert!(d.additional_fields.is_some());
        let af = d.additional_fields.unwrap(); // safe
        assert_eq!(af.0.len(), 1);
        let first = af.0.into_iter().next().unwrap(); // safe
        assert_eq!(first.0, "field_name");
        assert_eq!(first.1.len(), 1);
        let first_value = first.1.into_iter().next().unwrap(); // safe
        assert_eq!(first_value.0, "value");
        assert_eq!(first_value.1, "label");
    }

    #[test]
    fn test_disciplines_parse() {
        let string = r#"[
            {
                "id": "counterstrike_go",
                "name": "Counter-Strike: GO",
                "shortname": "CS:GO",
                "fullname": "Counter-Strike: Global Offensive",
                "copyrights": "Valve Software"
            },
            {
                "id": "quakelive",
                "name": "Quake Live",
                "shortname": "QL",
                "fullname": "Quake Live",
                "copyrights": "id Software"
            }
        ]"#;
        let ds: Disciplines = serde_json::from_str(string).unwrap();

        assert_eq!(ds.0.len(), 2);
        let correct_disciplines = vec![
            Discipline::new(
                DisciplineId("counterstrike_go".to_owned()),
                "Counter-Strike: GO",
                "CS:GO",
                "Counter-Strike: Global Offensive",
                "Valve Software",
            ),
            Discipline::new(
                DisciplineId("quakelive".to_owned()),
                "Quake Live",
                "QL",
                "Quake Live",
                "id Software",
            ),
        ];
        let mut iter = ds.0.iter().zip(correct_disciplines.iter());
        while let Some(pair) = iter.next() {
            assert_eq!(pair.0.id, pair.1.id);
            assert_eq!(pair.0.name, pair.1.name);
            assert_eq!(pair.0.short_name, pair.1.short_name);
            assert_eq!(pair.0.full_name, pair.1.full_name);
            assert_eq!(pair.0.copyrights, pair.1.copyrights);
        }
    }
}
