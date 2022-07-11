use async_std::{
  task::sleep,
  fs
};
use std::{
  borrow::Cow,
  collections::BTreeMap,
  path::Path,
  time::Duration,
  process::Command,
  os::windows::process::CommandExt
};
use sysinfo::SystemExt;
use serde::{Serialize, Deserialize};
use keyvalues_parser::{
  Value,
  Vdf
};

use winreg::{
  RegKey,
  enums::{
    HKEY_CURRENT_USER,
    KEY_ALL_ACCESS,
  },
};

use macro_tool::command;

//=====================
//  結構體
//=====================

/** Steam使用者 */
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone)]
struct User {
  Avatar: Option<String>,
  SteamID: String,
  AccountName: String,
  PersonaName: String,
  Timestamp: i32,
  MostRecent: i32,
  AllowAutoLogin:i32,
  WantsOfflineMode:i32,
  RememberPassword: i32,
  SkipOfflineModeWarning:i32,
}


//=====================
//  功能
//=====================

// /** 重啟Steam */
#[command(steamRestart)]
pub async fn restart(){
  println!("restart");
  let cur_ver = regkey();
  let exec_path: String = cur_ver.get_value("SteamExe").unwrap();
  let exec_name: &str = Path::new(&exec_path).file_name().unwrap().to_str().unwrap();
  
  let mut system = sysinfo::System::new();
  system.refresh_processes();
  
  if system.processes_by_exact_name(exec_name).count() != 0 {
    Command::new(&exec_path).args(["-shutdown"]).output().expect("steam was sleep");
    loop {
      system.refresh_processes();
      sleep(Duration::from_millis(100)).await;
      let proc = system.processes_by_exact_name(exec_name);
      if proc.count() == 0 { break };
    }
  }
  Command::new(&exec_path).creation_flags(0x00000200).spawn().expect("steam can't start");
}

/** 帳號功能 */
pub mod account {
  use async_std::fs;
  use winreg::{enums::{HKEY_CURRENT_USER, KEY_WRITE}, RegKey};
  use std::{borrow::Cow, collections::BTreeMap};
  use keyvalues_parser::{Vdf, Value};

  use super::{loginusers_path, loginusers, restart};
  use macro_tool::command;

  /**  刪除Steam帳號 */
  #[command(steamAccountDelete)]
  pub async fn delete(steamid:String){
    let path = loginusers_path();
    let data = fs::read_to_string(path.clone()).await.unwrap_or_else(|_| "".to_string());
    let mut vdf = Vdf::parse(&data).unwrap().value.unwrap_obj();
    vdf.remove(&Cow::from(&*steamid));
    fs::write(path, Vdf::new(Cow::from(&*steamid),Value::Obj(vdf)).to_string()).await.unwrap_or_default();
  }
  
  /**當前使用著
   * steamid 不為空時 嘗試登入
  */
  #[command(steamAccountCurrent)]
  pub async fn current(steamid:Option<String>) -> Result<String,String> {
    let users = loginusers().await;
    let res: String = match steamid {
      None => {
        let mut cur_user = users.iter().filter(|x|x.MostRecent == 1);
        match cur_user.next() {
          None => "".to_string(),
          Some(v) => v.SteamID.clone()
        }
      },
      Some(id) => {
        let mut userlist = BTreeMap::new();
        let mut name:String = String::new();
  
        users.iter().for_each(|x|{
          let recent = if x.SteamID == id {
            name = x.AccountName.clone();
            1
          } else { 0 };
          
          fn insert(btm:&mut BTreeMap<Cow<str>, Vec<Value>>,key:&'static str,val:&String){
            btm.insert(Cow::from(key),vec![Value::Str(Cow::from(val.clone()))]);
          }
  
          let mut btm = BTreeMap::new();
          insert(&mut btm,"AccountName",&x.AccountName);
          insert(&mut btm,"PersonaName",&x.PersonaName);
          insert(&mut btm,"RememberPassword",&x.RememberPassword.to_string());
          insert(&mut btm,"Timestamp",&x.Timestamp.to_string());
          insert(&mut btm,"AllowAutoLogin",&x.AllowAutoLogin.to_string());
          insert(&mut btm,"SkipOfflineModeWarning",&x.SkipOfflineModeWarning.to_string());
          insert(&mut btm,"WantsOfflineMode",&x.WantsOfflineMode.to_string());
          insert(&mut btm,"MostRecent",&recent.to_string());
  
          userlist.insert(Cow::from(x.SteamID.clone()),vec![Value::Obj(btm)]);
        });
  
        let vdf = Vdf::new(Cow::from("users"),Value::Obj(userlist));
        fs::write(loginusers_path(), vdf.to_string()).await.unwrap_or_default();
        let hklm = RegKey::predef(HKEY_CURRENT_USER);
        hklm
          .open_subkey_with_flags("Software\\Valve\\Steam", KEY_WRITE)
          .unwrap()
          .set_value("AutoLoginUser", &name)
          .unwrap();
        restart().await;
        id
      }
    };
    Ok(res)
  }

}

//=====================
//  小工具
//=====================

/** 取得使用者列表 */
async fn loginusers() -> Vec<User> {
  let vdf_path = loginusers_path();
  let data = fs::read_to_string(vdf_path).await.unwrap_or_else(|_| "".to_string());
  let vdf = Vdf::parse(&data).unwrap().value.unwrap_obj();

  let mut users: Vec<User> = Vec::new();

  vdf.iter().for_each(|(key, _value)|{
    let value = _value[0].get_obj().unwrap();
    fn get<'a>(name: &'a str, data: &'a BTreeMap<Cow<'a, str>, Vec<Value<'a>>>) -> String {
      match data.get(name) {
        None => String::from("undefined"),
        Some(v) => v[0].get_str().unwrap().to_string()
      }
    }

    fn int_to_string(data:String) -> i32 {
      data.parse::<i32>().unwrap_or_else(|_| -1)
    }

    let id = key.to_string();

    let avatar = path() + "/config/avatarcache/" + &id + ".png";
    let avatar = if Path::new(&avatar).exists() { Some(avatar) } else { None };

    users.push(
      User {
        AccountName: get("AccountName",value),
        PersonaName: get("PersonaName",value),
        Timestamp: int_to_string(get("Timestamp",value)),
        MostRecent: int_to_string(get("MostRecent",value)),
        AllowAutoLogin:int_to_string(get("AllowAutoLogin",value)),
        WantsOfflineMode:int_to_string(get("WantsOfflineMode",value)),
        RememberPassword: int_to_string(get("RememberPassword",value)),
        SkipOfflineModeWarning:int_to_string(get("SkipOfflineModeWarning",value)),
        Avatar: avatar,
        SteamID: id,
      }
    );
  });
  users
}

/** 登入使用者檔案位置 */
fn loginusers_path() -> String { path() + "/config/loginusers.vdf" }

/** 取得 steam 位置 */
#[command(steamPath)]
pub fn path() -> String {
  let cur_ver = regkey();
  let path: String = cur_ver.get_value("SteamPath").unwrap();
  path
}

/** 回傳Steam列表 */
#[command(steamUsers)]
pub async fn users() -> String {
  serde_json::to_string(&loginusers().await).unwrap()
}

/** 取得Steam RegKey */
fn regkey() -> RegKey {
  let hklm = RegKey::predef(HKEY_CURRENT_USER);
  hklm
    .open_subkey_with_flags("Software\\Valve\\Steam", KEY_ALL_ACCESS)
    .unwrap()
}