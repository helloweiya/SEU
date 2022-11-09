use std::collections::HashMap;
use crate::ids_encrypt;
pub use reqwest::{Client, ClientBuilder};
use reqwest::header::HeaderMap;
use soup::prelude::*;
use crate::ids_encrypt::encrypt_aes;
use serde::{Serialize, Deserialize};

// #[derive(Serialize, Deserialize)]
// struct Struct1 {
//     #[serde(rename = "desktopId")]
//     pub desktop_id: String,
//     #[serde(rename = "cardWid")]
//     pub card_wid: String,
//     #[serde(rename = "cardSeq")]
//     pub card_seq: i64,
//     #[serde(rename = "appId")]
//     pub app_id: String,
//     #[serde(rename = "cardId")]
//     pub card_id: String,
//     #[serde(rename = "cardName")]
//     pub card_name: String,
//     #[serde(rename = "cardNameEnus")]
//     pub card_name_enus: String,
//     #[serde(rename = "hasAuthLose")]
//     pub has_auth_lose: bool,
//     #[serde(rename = "appKey")]
//     pub app_key: String,
//     #[serde(rename = "shortName")]
//     pub short_name: String,
//     #[serde(rename = "cardBelongTo")]
//     pub card_belong_to: i64,
// }
//
// #[derive(Serialize, Deserialize)]
// struct Struct {
//     #[serde(rename = "desktopId")]
//     pub desktop_id: String,
//     #[serde(rename = "desktopName")]
//     pub desktop_name: String,
//     #[serde(rename = "desktopSeq")]
//     pub desktop_seq: i64,
//     pub creator: String,
//     #[serde(rename = "desktopType")]
//     pub desktop_type: i64,
//     #[serde(rename = "createTime")]
//     pub create_time: String,
//     #[serde(rename = "modifyTime")]
//     pub modify_time: String,
//     #[serde(rename = "desktopNameEnus")]
//     pub desktop_name_enus: Option<String>,
//     #[serde(rename = "isNew")]
//     pub is_new: i64,
//     #[serde(rename = "appPcCardList")]
//     pub app_pc_card_list: Vec<Struct1>,
//     #[serde(rename = "isDefaultLayout")]
//     pub is_default_layout: i64,
//     #[serde(rename = "isReset")]
//     pub is_reset: i64,
//     #[serde(rename = "isDelete")]
//     pub is_delete: i64,
//     #[serde(rename = "isDefaultDesktop")]
//     pub is_default_desktop: i64,
//     #[serde(rename = "isEditable")]
//     pub is_editable: i64,
// }
//
// #[derive(Serialize, Deserialize)]
// struct Root {
//     #[serde(rename = "appInfoAndUrl")]
//     pub app_info_and_url: String,
//     #[serde(rename = "isShowTeacherSite")]
//     pub is_show_teacher_site: bool,
//     #[serde(rename = "desktopUserList")]
//     pub desktop_user_list: Vec<Struct>,
//     #[serde(rename = "userIp")]
//     pub user_ip: String,
//     #[serde(rename = "schoolName")]
//     pub school_name: String,
//     #[serde(rename = "siteType")]
//     pub site_type: String,
//     #[serde(rename = "userTypeName")]
//     pub user_type_name: String,
//     #[serde(rename = "hasManagePermission")]
//     pub has_manage_permission: bool,
//     #[serde(rename = "userPhoto")]
//     pub user_photo: Option<String>,
//     #[serde(rename = "displayMode")]
//     pub display_mode: String,
//     #[serde(rename = "userType")]
//     pub user_type: String,
//     #[serde(rename = "flowAdmin")]
//     pub flow_admin: bool,
//     #[serde(rename = "userID")]
//     pub user_id: String,
//     #[serde(rename = "userDepartment")]
//     pub user_department: String,
//     pub roles: Vec<String>,
//     #[serde(rename = "userId")]
//     pub user_id_0: String,
//     #[serde(rename = "userSex")]
//     pub user_sex: String,
//     #[serde(rename = "hasTeacherDesktopAuth")]
//     pub has_teacher_desktop_auth: bool,
//     #[serde(rename = "userName")]
//     pub user_name: String,
//     #[serde(rename = "schoolID")]
//     pub school_id: String,
//     #[serde(rename = "hasLogin")]
//     pub has_login: bool,
//     #[serde(rename = "contextPath")]
//     pub context_path: String,
//     #[serde(rename = "appPcCardBizList")]
//     pub app_pc_card_biz_list: Vec<String>,
// }


const AUTH_URL: &str = "https://newids.seu.edu.cn/authserver/login?goto=http://my.seu.edu.cn/index.portal";
const EHALL_URL: &str = "http://ehall.seu.edu.cn/login?service=http://ehall.seu.edu.cn/new/index.html";
const USER_INFO_URL: &str = "http://ehall.seu.edu.cn/jsonp/userDesktopInfo.json";

// const BARK_URL: &str = "https://bark.hellowei.top/5Ar5gkJpLvMbRkAMMSB4UN/这里改成你自己的推送内容";


fn update_form(html: String, params: &mut HashMap<String, String> ){
    let soup = Soup::new(&html);
    let res: String;
    for item in soup.tag("input").find_all() {
        if let Some(tp) = item.get("type"){
            if tp == "hidden" {
                if let Some(name) = item.get("name") {
                    params.insert(name, item.get("value").unwrap());
                } else if let Some(id) = item.get("id"){
                    params.insert(id, item.get("value").unwrap());
                }
            }
        }
    }
}

fn set_header(header: &mut HeaderMap) {
    header.insert("Content-Type", "application/x-www-form-urlencoded".parse().unwrap());
    header.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/102.0.0.0 Safari/537.36".parse().unwrap());
}

pub async fn login(card_num: &str, passwd: &str) -> Option<Client>{
    let mut header = HeaderMap::new();
    set_header(&mut header);

    let client = ClientBuilder::new()
        .cookie_store(true)
        .default_headers(header)
        .build().unwrap();

    let content = client.get(AUTH_URL)
        .send()
        .await.unwrap()
        .text().await.unwrap();

    let mut params = HashMap::new();
    params.insert("username".to_string(),card_num.to_string());
    update_form(content,&mut params);
    let pass = encrypt_aes(passwd,&(params.get("pwdDefaultEncryptSalt").unwrap()));
    params.insert("password".to_string(), pass);
    client.post(AUTH_URL)
        .form(&params)
        .send().await.unwrap();

    client.get(EHALL_URL).send().await.unwrap();
    let mut res = client.get(USER_INFO_URL).send().await.unwrap().text().await.unwrap_or_else(
        crate::sender.send_msg("登录失败！")
    );

    // let res:Root = serde_json::from_str(&res).unwrap();
    // println!("{}",res.user_name);
    Some(client)
}
