pub mod model {
  use serde_derive::{Deserialize, Serialize};

  #[derive(Deserialize)]
  pub struct Report {
    // report_uuid: String,
    node_name:   String,
    profiles:    Vec<Profile>,
  }
  #[derive(Deserialize)]
  pub struct Profile {
    name:   String,
    controls: Vec<Control>,
  }

  #[derive(Deserialize)]
  pub struct Control {
    id:      String,
    title:   String,
    // impact:  f32,
    results: Option<Vec<InspecResult>>,
  }

  #[derive(Deserialize)]
  pub struct InspecResult {
    status:   String, //passed, skipped, failed
    // code_desc: String,
  }

  #[derive(Serialize)]
  pub struct WebHookMessage {
    mes:   String,
  }

  #[derive(Serialize)]
  pub struct SlackMessage {
    username:   String,
    text:        String,
    icon_url:    String,
    attachments: Vec<SlackAttachment>,
  }

  #[derive(Serialize)]
  pub struct SlackAttachment {
    text: String,
    color: String,
    fields: Vec<SlackField>,
  }

  #[derive(Serialize)]
  pub struct SlackField {
    title: String,
    value: String,
    short: bool,
  }

  impl Control {
    pub fn failed(&self) -> bool {

      if let Some(rs) = &self.results{
        return rs.iter().any(|r| r.status == "failed")
      }

      return false
    }

    pub fn number_of_failed_tests(&self) -> usize {
      if let Some(rs) = &self.results{
        return rs.iter().filter(|r| r.status == "failed").count()
      }
      0
    }
  }

  impl Profile {
    pub fn failed(&self) -> bool {
      return self.controls.iter().any(|c| c.failed())
    }

    pub fn number_of_failed_tests(&self) -> usize {
      self.controls.iter().map(|c| c.number_of_failed_tests()).sum()
    }

    pub fn failed_controls(&self) -> Vec<&Control> {
      self.controls.iter().filter(|c| c.failed()).collect::<Vec<_>>()
    }
  }

  impl Report {
    pub fn from_str(s: String) -> Report {
      serde_json::from_str(&s).expect("message was not parsable")
    }

    pub fn has_notification_to_send(&self) -> bool {
      self.num_failed_profiles() > 0
    }

    pub fn failed_profiles(&self) -> Vec<&Profile> {
      self.profiles.iter().filter(|p| p.failed()).collect::<Vec<_>>()
    }

    pub fn num_failed_profiles(&self) -> usize {
      (&self.profiles).into_iter().filter(|p| p.failed()).count()
    }

    pub fn num_failed_tests(&self) -> usize {
      self.profiles.iter().map(|c| c.number_of_failed_tests()).sum()
    }

    pub fn webhook_message(&self) -> String {
      let message = WebHookMessage {
        mes: String::from("Failed tests")
      };
  
      serde_json::to_string(&message).unwrap()
    }

    pub fn slack_message(&self) -> String {
      let mut failed_profiles_name = String::from("<None>");
      let mut failed_control_info = String::from("<None>");

      let failed_profiles = self.failed_profiles();
      if failed_profiles.len() > 0 {
        let failed_profile = failed_profiles[0];
        failed_profiles_name = format!("{}", failed_profile.name);
    
        let failed_controls = failed_profile.failed_controls();
        if failed_controls.len() > 0 {
          let failed_control = failed_controls[0];
          failed_control_info = format!("{}:{}", failed_control.id, failed_control.title)
        }
      }

      let message = SlackMessage {
        username: String::from("Notification Service"),
        text: format!("Reminder of InSpec found a critical control failure on node {}", self.node_name),
        icon_url: String::from("https://docs.chef.io/_static/chef_logo_v2.png"),
        attachments: vec![
          SlackAttachment{ 
            text: format!("{} tests failed. Rerun the test locally for full details.", self.num_failed_tests()), 
            color: String::from("warning"),
            fields: vec![
              SlackField{
                title: String::from("Control ID::Title"),
                value: failed_control_info,
                short: false
              },
              SlackField{
                title: String::from("Profile"),
                value: failed_profiles_name,
                short: false
              },
              SlackField{
                title: String::from("Node"),
                value: String::from("report.NodeName"),
                short: false
              },
              SlackField{
                title: String::from("Highest Failed Impact"),
                value: String::from("0.0"),
                short: false
              }
            ]
          }
        ]
      };
  
      serde_json::to_string(&message).unwrap()
    }
  }
}
