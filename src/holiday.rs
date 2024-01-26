use anyhow::{anyhow, Result};
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HolidayResult {
  #[serde(rename = "$schema")]
  pub schema: String,
  #[serde(rename = "$id")]
  pub id: String,
  pub year: i64,
  pub papers: Vec<String>,
  pub days: Vec<Holiday>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Holiday {
  pub name: String,
  pub date: String,
  pub is_off_day: bool,
}

async fn fetch_holidays(year: String) -> Result<Vec<Holiday>> {
  let url = format!(
    "https://raw.githubusercontent.com/NateScarlet/holiday-cn/master/{}.json",
    year
  );
  let resp = Request::get(&url)
    .send()
    .await
    .map_err(|e| anyhow!("请求日期数据出错: {:?}", e))?;
  let text = resp
    .text()
    .await
    .map_err(|e| anyhow::anyhow!("获取日期数据出错:{:?}", e))?;
  let holidays = serde_json::from_str::<HolidayResult>(&text)
    .map_err(|e| anyhow!("解析日期数据出错: {:?}", e))?;
  Ok(holidays.days)
}
