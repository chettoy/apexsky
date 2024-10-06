use std::path::PathBuf;

/// Get the home directory of the user.
/// 1. ```SUDO_HOME if (unix, uid == 0)```
/// 2. ```homedir::home(SUDO_UID) if (unix, uid == 0)```
/// 3. ```homedir::my_home() // process' current user```
pub fn get_runner_home_dir() -> Option<PathBuf> {
    // Checking sudoer's home on unix systems
    #[cfg(unix)]
    {
        use nix::unistd::Uid;
        use obfstr::obfstr as s;
        use uzers::{get_current_uid, get_user_by_uid};
        if get_current_uid() == 0 {
            if let Some(from_env) = std::env::var(s!("SUDO_HOME")).ok().map(PathBuf::from) {
                return Some(from_env);
            }
            if let Some(original_user_uid) = std::env::var(s!("SUDO_UID")).ok() {
                let original_user_uid: Uid = original_user_uid
                    .parse::<u32>()
                    .expect(s!("Invalid SUDO_UID"))
                    .into();
                let original_user = get_user_by_uid(original_user_uid.into())
                    .expect(s!("Failed to get original user"));
                let original_user_username = original_user.name().to_str().expect(&format!(
                    "{}{:?}{}",
                    s!("Invalid username `"),
                    original_user.name(),
                    s!("` -> str")
                ));
                return homedir::home(original_user_username).ok().flatten();
            }
        }
    }

    // use homedir::myhome()
    homedir::my_home().ok().flatten()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_runner_home() {
        #[cfg(unix)]
        {
            use uzers::get_current_uid;
            if get_current_uid() == 0 {
                get_runner_home_dir().unwrap();
                return;
            }
        }
        assert_eq!(get_runner_home_dir(), homedir::my_home().ok().flatten())
    }
}
