pub mod model {
  use serde_derive::Deserialize;

  #[derive(Deserialize)]
  pub struct Report {
    // report_uuid: String,
    // node_name:   String,
    profiles:    Vec<Profile>,
  }
  #[derive(Deserialize)]
  pub struct Profile {
    // name:   String,
    controls: Vec<Control>,
  }

  #[derive(Deserialize)]
  pub struct Control {
    // id:      String,
    // title:   String,
    // impact:  f32,
    results: Option<Vec<InspecResult>>,
  }

  #[derive(Deserialize)]
  pub struct InspecResult {
    status:   String, //passed, skipped, failed
    // code_desc: String,
  }

  impl Control {
    pub fn failed(&self) -> bool {

      if let Some(rs) = &self.results{
        return rs.iter().any(|r| r.status == "failed")
      }

      return false
    }
  }

  impl Profile {
    pub fn failed(&self) -> bool {
      return self.controls.iter().any(|c| c.failed())
    }
  }

  impl Report {
    pub fn from_str(s: String) -> Report {
      serde_json::from_str(&s).expect("message was not parsable")
    }

    pub fn has_notification_to_send(self) -> bool {
      return self.num_failed_profiles() > 0
    }

    pub fn num_failed_profiles(self) -> usize {
      return self.profiles.into_iter().filter(|p| p.failed()).count();
    }
  }
}
