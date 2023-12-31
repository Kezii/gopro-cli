/*
 * gopro API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MediaItem {
    /// Media filename
    #[serde(rename = "n")]
    pub n: String,
    /// Created time (seconds since epoch)
    #[serde(rename = "cre")]
    pub cre: String,
    /// Last modified time (seconds since epoch)
    #[serde(rename = "mod")]
    pub r#mod: String,
    /// Size of (group) media in bytes
    #[serde(rename = "s")]
    pub s: String,
    /// Low resolution video file size
    #[serde(rename = "glrv", skip_serializing_if = "Option::is_none")]
    pub glrv: Option<String>,
    /// ID of last member of a group (for grouped media items)
    #[serde(rename = "ls", skip_serializing_if = "Option::is_none")]
    pub ls: Option<String>,
    /// Group ID (if grouped media item)
    #[serde(rename = "g", skip_serializing_if = "Option::is_none")]
    pub g: Option<String>,
    /// ID of first member of a group (for grouped media items)
    #[serde(rename = "b", skip_serializing_if = "Option::is_none")]
    pub b: Option<String>,
    /// ID of last member of a group (for grouped media items)
    #[serde(rename = "l", skip_serializing_if = "Option::is_none")]
    pub l: Option<String>,
    /// Group type (for grouped media items) (b -> burst, c -> continuous shot, n -> night lapse, t -> time lapse)
    #[serde(rename = "t", skip_serializing_if = "Option::is_none")]
    pub t: Option<String>,
    /// List of missing/deleted group member IDs (for grouped media items)
    #[serde(rename = "m", skip_serializing_if = "Option::is_none")]
    pub m: Option<Vec<String>>,
}

impl MediaItem {
    pub fn new(n: String, cre: String, r#mod: String, s: String) -> MediaItem {
        MediaItem {
            n,
            cre,
            r#mod,
            s,
            glrv: None,
            ls: None,
            g: None,
            b: None,
            l: None,
            t: None,
            m: None,
        }
    }
}


