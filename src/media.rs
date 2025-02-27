use anyhow::{Result, anyhow};
use std::path::Path;
use serde::Deserialize;

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
    /// 静态匹配数据
    pub const PARSERS: [(&str, [usize; 3]); 6] = [
        // (pattern, indexs)
        // 匹配模式 1: /影片/电视剧/医馆笑传/医馆笑传S01.37集.1080P/01.mkv
        // title: 医馆笑传
        // season: 1
        // episode: 37
        (r"^(.*?)/([^/]+)S(\d{2})\.(\d{1,2})集\.(\d{4}P)/(\d{2})\.(\w+)$", [2, 3, 6]),
        // 匹配模式 2: /还珠格格S01.国语中字.无台标.1080P/还珠格格S01E02.mp4
        // title: 还珠格格
        // season: 1
        // episode: 2
        (r"^(.*?)/([^/]+)S(\d{2})E(\d{2})\.(\w+)$", [2, 3, 4]),
        // 匹配模式 3: /Volumes/ZhiTai/影片/电视剧/爱情公寓/S4 (2014) 4K/01.mp4
        // title: 爱情公寓
        // season: 4
        // episode: 1
        (r"^(.*?)/([^/]+)/S(\d{1,2})\s+\(\d{4}\)\s+\d{1,2}K/(\d{2})\.(\w+)$", [2, 3, 4]),
        // 匹配模式 4: /电视剧/龙门镖局/龙门镖局 (2013) 4K/龙门镖局.Longmen.Express.2013.E02.4K.2160p.HEVC.AAC-DHTCLUB.mp4
        // title: 龙门镖局
        // season: 固定=1 indexs 中间用 0 表示
        // episode: 2
        (r"/([^/]+)/([^/]+) \(.*\) .*E(\d{2})", [2, 0, 3]),
        // 匹配模式 5: /Volumes/Getea/影片/电影/黄渤/疯狂的赛车.2009.01201.mp4
        // title: 疯狂的赛车
        // season: 2009
        // episode: 01201
        (r"^(.*?)/([^/]+)\.(\d{4})\.(\d{5})\.\w+$", [2, 3, 4]),
        // 匹配模式 6: /Volumes/ZhiTai/影片/电视剧/约会专家.1080P/约会专家第04集.mp4
        // title: 约会专家
        // season: 固定=1 indexs 中间用 0 表示
        // episode: 4
        (r"^(.*?)/([^/]+)\.1080P/.*?第(\d{1,2})集\.\w+$", [2, 0, 3]),
    ];

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
        // 尝试匹配每个模式
        let mut parsers = vec![];
        for parser in Self::PARSERS {
            parsers.push(RegexParser::new(parser.0, parser.1.to_vec()))
        }
        Self::from_path_with_regex(path, parsers)
    }

    pub fn from_path_with_regex<P, T>(path: P, parsers: Vec<T>) -> Result<Option<Self>>
        where P: AsRef<Path>,
              T: Parser,
    {

        let path_str = path.as_ref().to_str().ok_or_else(|| anyhow!("Invalid path"))?;

        // 尝试匹配每个模式
        for parser in parsers {
            let item = parser.parse(path_str);
            if item.is_some() {
                return Ok(item);
            }
        }

        // 配合文件夹名称
        Ok(None)
    }

}

pub trait Parser {
    fn parse(&self, path: &str) -> Option<Episode>;
}

#[derive(Debug, Deserialize)]
pub struct RegexParser {
    pattern: String,
    indexes: Vec<usize>,
}

impl RegexParser {
    pub fn new<P: AsRef<str>>(pattern: P, indexes: Vec<usize>) -> Self {
        Self { pattern: pattern.as_ref().to_string(), indexes }
    }
}

impl Parser for RegexParser {
    fn parse(&self, path: &str) -> Option<Episode> {
        let re = Regex::new(&self.pattern).ok()?;
        let indexs = self.indexes.clone();
        if let Some(caps) = re.captures(path) {
            // println!("{caps:#?}");
            let title = &caps[indexs[0]]; // 剧名
            let mut season = Some(1);
            if indexs[1] != 0 {
                let season_text = &caps[indexs[1]]; // 季数
                season = season_text.parse().ok();
            }
            let episode = &caps[indexs[2]]; // 集数
            Some(Episode {
                title: Some(title.to_string()),
                season,
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
        let item = Episode::from_path(path).unwrap();
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
        let item = Episode::from_path(path).unwrap();
        println!("{item:?}");
        assert!(item.is_some());
        if let Some(ep) = item {
            assert_eq!(ep.title, Some("还珠格格".to_string()));
            assert_eq!(ep.season, Some(1));
            assert_eq!(ep.episode, Some(2));
        }
    }

    #[test]
    fn test_match_pattern3() {
        let path = "/Volumes/ZhiTai/影片/电视剧/爱情公寓/S2 (2011) 4K/02.mp4";
        let item = Episode::from_path(path).unwrap();
        assert!(item.is_some());
        if let Some(ep) = item {
            assert_eq!(ep.title, Some("爱情公寓".to_string()));
            assert_eq!(ep.season, Some(2));
            assert_eq!(ep.episode, Some(2));
        }

    }
    #[test]
    fn test_match_pattern4() {
        let path = "/Volumes/ZhiTai/影片/电视剧/龙门镖局/龙门镖局 (2013) 4K/龙门镖局.Longmen.Express.2013.E02.4K.2160p.HEVC.AAC-DHTCLUB.mp4";
        let item = Episode::from_path(path).unwrap();
        assert!(item.is_some());
        if let Some(ep) = item {
            assert_eq!(ep.title, Some("龙门镖局".to_string()));
            assert_eq!(ep.season, Some(1)); // 默认季数为1
            assert_eq!(ep.episode, Some(2));
        }
    }

    #[test]
    fn test_match_pattern5() {
        let path = "/Volumes/Getea/影/影/渤/疯狂的赛车.2009.01201.mp4";
        let item = Episode::from_path(path).unwrap();
        assert!(item.is_some());
        if let Some(ep) = item {
            assert_eq!(ep.title, Some("疯狂的赛车".to_string()));
            assert_eq!(ep.season, Some(2009));
            assert_eq!(ep.episode, Some(1201));
        }
    }

    #[test]
    fn test_match_pattern6() {
        let path = "/Volumes/ZhiTai/影片/电视剧/约会专家.1080P/约会专家第04集.mp4";
        let item = Episode::from_path(path).unwrap();
        assert!(item.is_some());
        if let Some(ep) = item {
            assert_eq!(ep.title, Some("约会专家".to_string()));
            assert_eq!(ep.season, Some(1));
            assert_eq!(ep.episode, Some(4));
        }
    }

}
