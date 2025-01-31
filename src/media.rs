use anyhow::{Result, anyhow};
use std::path::Path;

use regex::Regex;


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
    ///
    /// let path = "/Volumes/医馆笑传S02.37集.1080P/03.mp4";
    /// let item = Episode::from_path(path).unwrap();
    /// assert!(item.is_some());
    /// if let Some(ep) = item {
    ///     assert_eq!(ep.title, Some("医馆笑传".to_string()));
    ///     assert_eq!(ep.season, Some(2));
    ///     assert_eq!(ep.episode, Some(3));
    /// }
    /// ```
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Option<Self>> {

        let path_str = path.as_ref().to_str().ok_or_else(|| anyhow!("Invalid path"))?;

        // 尝试匹配每个模式
        if let Some(ep) = Self::match_pattern1(path_str) {
            return Ok(Some(ep));
        }
        if let Some(ep) = Self::match_pattern2(path_str) {
            return Ok(Some(ep));
        }

        // 配合文件夹名称
        Ok(None)
    }


    /// 匹配模式 1: /影片/电视剧/医馆笑传/医馆笑传S01.37集.1080P/01.mkv
    fn match_pattern1(path: &str) -> Option<Self> {
        let pattern = r"^(.*?)/([^/]+)S(\d{2})\.(\d{1,2})集\.(\d{4}P)/(\d{2})\.(\w+)$";
        let indexs = vec![2, 3, 6];
        Self::match_pattern(path, pattern, indexs)
    }

    /// 匹配模式 2: /还珠格格S01.国语中字.无台标.1080P/还珠格格S01E02.mp4
    fn match_pattern2(path: &str) -> Option<Self> {
        let pattern = r"^(.*?)/([^/]+)S(\d{2})E(\d{2})\.(\w+)$";
        let indexs = vec![2, 3, 4];
        Self::match_pattern(path, pattern, indexs)
    }

    /// 匹配模式通用
    /// /还珠格格S01.国语中字.无台标.1080P/还珠格格S01E02.mp4
    fn match_pattern(path: &str, pattern: &str, indexs: Vec<usize>) -> Option<Self> {
        let re = Regex::new(pattern).ok()?;
        if let Some(caps) = re.captures(path) {
            let title = &caps[indexs[0]]; // 剧名
            let season = &caps[indexs[1]]; // 季数
            let episode = &caps[indexs[2]]; // 集数
            Some(Episode {
                title: Some(title.to_string()),
                season: season.parse().ok(),
                episode: episode.parse().ok(),
            })
        } else {
            None
        }
    }

}

#[cfg(test)]
mod tests {
    use super::Episode;

    #[test]
    fn test_match_pattern1() {
        let path = "/Volumes/Getea/影片/电视剧/医馆笑传/医馆笑传S01.37集.1080P/01.mp4";
        let item = Episode::match_pattern1(path);
        assert!(item.is_some());
        if let Some(ep) = item {
            assert_eq!(ep.title, Some("医馆笑传".to_string()));
            assert_eq!(ep.season, Some(1));
            assert_eq!(ep.episode, Some(1));
        }
    }

    #[test]
    fn test_match_pattern2() {
        let path = "/Volumes/还珠格格S01.国语中字.无台标.1080P/还珠格格S01E02.mp4";
        let item = Episode::match_pattern2(path);
        println!("{item:?}");
        assert!(item.is_some());
        if let Some(ep) = item {
            assert_eq!(ep.title, Some("还珠格格".to_string()));
            assert_eq!(ep.season, Some(1));
            assert_eq!(ep.episode, Some(2));
        }
    }

}
