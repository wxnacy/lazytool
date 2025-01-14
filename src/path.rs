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

