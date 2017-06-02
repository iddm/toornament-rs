/// Unique participant identifier
#[derive(Clone, Default, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct ParticipantId(pub String);

/// A participant type enumeration.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ParticipantType {
    /// Implies the tournament is played by teams
    Team,
    /// Means the tournament is played by players
    Single,
}

/// Logo of the participant.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct ParticipantLogo {
    /// Url to a picture of 48x48px.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_large_square: Option<String>,
    /// Url to a picture of 100x100px.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_small_square: Option<String>,
    /// Url to a picture of 200x200px.>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medium_small_square: Option<String>,
    /// Url to a picture of 400x400px.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medium_large_square: Option<String>,
}

/// A type of a participant's custom field
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum CustomFieldType {
    /// Participant's steam id
    #[serde(rename = "steam_player_id")]
    SteamId,
    /// Participant's birth date
    #[serde(rename = "birth_date")]
    Birthdate,
    /// Participant's facebook page
    #[serde(rename = "facebook")]
    Facebook,
    /// Participant's full name
    #[serde(rename = "full_name")]
    Fullname,
    /// Participant's instagram page
    #[serde(rename = "instagram")]
    Instagram,
    /// Participant's snapchat
    #[serde(rename = "snapchat")]
    Snapchat,
    /// Participant's text statement
    #[serde(rename = "text")]
    Text,
    /// Participant's twitch stream
    #[serde(rename = "twitch")]
    Twitch,
    /// Participant's twitter account
    #[serde(rename = "twitter")]
    Twitter,
    /// Participant's vimeo account
    #[serde(rename = "vimeo")]
    Vimeo,
    /// Participant's website
    #[serde(rename = "website")]
    Website,
    /// Participant's youtube channel
    #[serde(rename = "youtube")]
    Youtube,
}

/// A participant's custom fields
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct CustomField {
    /// Type of field.
    #[serde(rename = "type")]
    pub field_type: CustomFieldType,
    /// Label of field.
    pub label: String,
    /// Value informed.
    pub value: String,
}

/// A list of participant's custom fields
#[derive(Clone, Default, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct CustomFields(pub Vec<CustomField>);

/// An opponent involved in a match/tournament.
#[derive(Clone, Default, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Participant {
    /// Unique identifier for this participant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ParticipantId>,
    /// Participant name (maximum 40 characters).
    pub name: String,
    /// Logo of the participant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<ParticipantLogo>,
    /// This property is only available when the participant type is "team".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lineup: Option<Participants>,
    /// List of public custom fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<CustomFields>,
    /// Country of the participant. This property is only available when the "country"
    /// option is enabled for this tournament. This value is represented as an ISO 3166-1
    /// alpha-2 country code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Participant email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Participant check-in. This property is only available when "check-in" option is
    /// enabled for this tournament. 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_in: Option<bool>,
    /// This property is only available when the query parameter 'with_custom_fields' is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields_private: Option<CustomFields>,
}
impl Participant {
    builder_o!(id, ParticipantId);
    builder!(name, String);
    builder_o!(logo, ParticipantLogo);
    builder_o!(lineup, Participants);
    builder_o!(custom_fields, CustomFields);
    builder_o!(country, String);
    builder_o!(email, String);
    builder_o!(check_in, bool);
    builder_o!(custom_fields_private, CustomFields);
}

/// A list of participants
#[derive(Clone, Default, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Participants(pub Vec<Participant>);

#[cfg(test)]
mod tests {
    use ::serde_json;
    use ::{ Participants, CustomFieldType };

    #[test]
    fn test_participant_parse() {
        let s = r#"
[
    {
        "id": "378426939508809728",
        "name": "Evil Geniuses",
        "logo": {
            "icon_large_square": "http://api.toornament.com/id/icon_large_square",
            "extra_small_square": "http://api.toornament.com/id/extra_small_square",
            "medium_small_square": "http://api.toornament.com/id/medium_small_square",
            "medium_large_square": "http://api.toornament.com/id/medium_large_square"
        },
        "country": "US",
        "lineup": [
            {
                "name": "Storm Spirit",
                "country": "US",
                "custom_fields": [
                    {
                        "type": "steam_player_id",
                        "label": "Steam ID",
                        "value": "STEAM_0:1:1234567"
                    }
                ],
                "email": "player@oxent.net",
                "custom_fields_private": [
                    {
                        "type": "steam_player_id",
                        "label": "Steam ID",
                        "value": "STEAM_0:1:1234567"
                    }
                ]
            }
        ],
        "custom_fields": [
            {
                "type": "steam_player_id",
                "label": "Steam ID",
                "value": "STEAM_0:1:1234567"
            }
        ],
        "email": "contact@oxent.net",
        "check_in": true,
        "custom_fields_private": [
            {
                "type": "steam_player_id",
                "label": "Steam ID",
                "value": "STEAM_0:1:1234567"
            }
        ]
    }
]
        "#;

        let ps: Participants = serde_json::from_str(s).unwrap();
        assert_eq!(ps.0.len(), 1);
        let p = ps.0.iter().next().unwrap().clone();

        assert_eq!(p.id.unwrap().0, "378426939508809728");
        assert_eq!(p.name, "Evil Geniuses");
        let logo = p.logo.unwrap();
        assert_eq!(logo.icon_large_square, "http://api.toornament.com/id/icon_large_square");
        assert_eq!(logo.extra_small_square, "http://api.toornament.com/id/extra_small_square");
        assert_eq!(logo.medium_small_square, "http://api.toornament.com/id/medium_small_square");
        assert_eq!(logo.medium_large_square, "http://api.toornament.com/id/medium_large_square");
        assert_eq!(p.country, Some("US".to_owned()));
        let lineup = p.lineup.unwrap().0;
        assert_eq!(lineup.len(), 1);
        let lp = lineup.iter().next().unwrap();
        assert!(lp.id.is_none());
        assert_eq!(lp.name, "Storm Spirit");
        assert_eq!(lp.country, Some("US".to_owned()));
        {
            let lpcfs = lp.custom_fields.clone().unwrap().0;
            assert_eq!(lpcfs.len(), 1);
            let lpcf = lpcfs.iter().next().unwrap();
            assert_eq!(lpcf.field_type, CustomFieldType::SteamId);
            assert_eq!(lpcf.label, "Steam ID");
            assert_eq!(lpcf.value, "STEAM_0:1:1234567");
        }
        assert_eq!(lp.email, Some("player@oxent.net".to_owned()));
        {
            let lpcfsp = lp.custom_fields_private.clone().unwrap().0;
            assert_eq!(lpcfsp.len(), 1);
            let lpcfp = lpcfsp.iter().next().unwrap();
            assert_eq!(lpcfp.field_type, CustomFieldType::SteamId);
            assert_eq!(lpcfp.label, "Steam ID");
            assert_eq!(lpcfp.value, "STEAM_0:1:1234567");
        }
        assert_eq!(p.email, Some("contact@oxent.net".to_owned()));
        assert_eq!(p.check_in, Some(true));
        {
            let pcfs = p.custom_fields.clone().unwrap().0;
            assert_eq!(pcfs.len(), 1);
            let pcf = pcfs.iter().next().unwrap();
            assert_eq!(pcf.field_type, CustomFieldType::SteamId);
            assert_eq!(pcf.label, "Steam ID");
            assert_eq!(pcf.value, "STEAM_0:1:1234567");
        }
        {
            let pcfsp = p.custom_fields_private.clone().unwrap().0;
            assert_eq!(pcfsp.len(), 1);
            let pcfp = pcfsp.iter().next().unwrap();
            assert_eq!(pcfp.field_type, CustomFieldType::SteamId);
            assert_eq!(pcfp.label, "Steam ID");
            assert_eq!(pcfp.value, "STEAM_0:1:1234567");
        }
    }
}
