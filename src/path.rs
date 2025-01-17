use std::{env, path::{Path, PathBuf}};

/// 解析 `~` 家地址
///
/// Examples
///
/// ```
///
/// use lazytool::expand_user;
/// use std::path::PathBuf;
///
/// let path = expand_user("~/.bash_profile");
/// assert_eq!(path, PathBuf::from("/Users/wxnacy/.bash_profile"));
///
/// let path = expand_user("/tmp/foo");
/// assert_eq!(path, PathBuf::from("/tmp/foo"));
/// ```
pub fn expand_user<P: AsRef<Path>>(path: P) -> PathBuf {
    let mut path_buf = PathBuf::from(path.as_ref());

    // 检查路径是否以 '~' 开头
    if path_buf.starts_with("~") {
        // 获取用户的主目录
        if let Some(home_dir) = env::var_os("HOME") {
            // 替换 '~' 为主目录路径
            let home = Path::new(&home_dir);
            path_buf = home.join(path_buf.strip_prefix("~").unwrap());
        }
    }

    path_buf
}

/// 地址强行转为 `String`
///
/// Examples
///
/// ```
///
/// use lazytool::path;
///
/// let s = path::must_to_string("/tmp/filter");
/// assert_eq!(s, String::from("/tmp/filter"));
/// ```
pub fn must_to_string<P: AsRef<Path>>(path: P) -> String {
    if let Some(s) = path.as_ref().to_str() {
        return s.to_string();
    }
    String::new()
}

/// 强行获取地址文件名
///
/// Examples
///
/// ```
///
/// use lazytool::path;
///
/// let s = path::must_get_filename("/tmp/filter.json");
/// assert_eq!(s, String::from("filter.json"));
/// ```
pub fn must_get_filename<P: AsRef<Path>>(path: P) -> String {
    path.as_ref().file_name().expect("Failed get filename").to_string_lossy().into_owned()
}
