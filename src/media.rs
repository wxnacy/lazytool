use anyhow::{Result, anyhow};
use std::path::Path;

use regex::Regex;

use crate::path::must_to_string;

#[derive(Debug, Clone)]
pub struct Episode {
    // 剧名
    pub title: Option<String>,

    // 季数
    pub season: Option<u16>,

    // 集数
    pub episode: Option<u16>,
}

impl Episode {
    /// 从地址中解析剧集信息
    ///
    /// Examples
    ///
    /// ```
    /// use lazytool::Episode;
    ///
    /// let path = "/还珠格格S01.国语中字.无台标.1080P/还珠格格S01E02.mp4";
    /// let item = Episode::from_path(path).unwrap();
    /// assert!(item.is_some());
    /// if let Some(ep) = item {
    ///     assert_eq!(ep.title, Some("还珠格格".to_string()));
    ///     assert_eq!(ep.season, Some(1));
    ///     assert_eq!(ep.episode, Some(2));
    /// }
    /// ```
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Option<Self>> {

        let mut filename = String::new();
        if let Some(name) = path.as_ref().with_extension("").file_name() {
            if let Some(name) = name.to_str() {
                filename = name.to_owned();
            }
        }
        if filename.is_empty() {
            return Err(anyhow!("{} not found filename", must_to_string(&path)));
        }

        let pattern = r"^(.*?)S(\d{2})E(\d{2})$";
        let re = Regex::new(pattern)?;

        if let Some(caps) = re.captures(&filename) {
            let title = &caps[1]; // 剧名
            let season = &caps[2]; // 季数
            let episode = &caps[3]; // 集数

            return Ok(Some(Episode {
                title: Some(title.to_string()),
                season: Some(season.parse()?),
                episode: Some(episode.parse()?),
            }))
        }
        Ok(None)
    }
}
