// TODO Handle errors
pub fn run(url: String) -> String {
  if url.starts_with("git") {
    let site: &str = url.split("@").nth(1).unwrap().split(":").nth(0).unwrap();
    let user: &str = url.split(":").nth(1).unwrap().split("/").nth(0).unwrap();
    let project: &str = url.split("/").nth(1).unwrap();

    format!("https://{}/{}/{}", site, user, project)
  } else {
    let mut site: &str = url.split("//").nth(1).unwrap().split("/").nth(0).unwrap();
    if site.contains("@") {
      site = site.split("@").nth(1).unwrap();
    }
    let user: &str = url.split("/").nth(3).unwrap();
    let project: &str = url.split("/").nth(4).unwrap();

    format!("git@{}:{}/{}", site, user, project)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn git_url() {
    let url = String::from("git@github.com:MarkNjunge/JustJava-Android.git");

    assert_eq!(
      String::from("https://github.com/MarkNjunge/JustJava-Android.git"),
      run(url)
    )
  }

  #[test]
  fn https_url() {
    let url = String::from("https://github.com/MarkNjunge/JustJava-Android.git");

    assert_eq!(
      String::from("git@github.com:MarkNjunge/JustJava-Android.git"),
      run(url)
    )
  }
}
