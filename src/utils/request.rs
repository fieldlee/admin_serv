use actix_web::{HttpRequest};
use std::collections::HashMap;

/// get the client ip address
pub fn get_ip(req: &HttpRequest) -> String { 
    let addr = req.peer_addr().unwrap();
    format!("{}", addr.ip())
}

/// 獲得token信息
//pub fn get_token(req: &HttpRequest) -> UserToken {
//    let headers = req.headers();
//    let auth_string = headers.get(AUTHORIZATION).unwrap().to_str().unwrap();
//    let auth_arr = auth_string.split(" ").collect::<Vec<&str>>();
//    jsonwebtoken::decode::<UserToken>(auth_arr[1], &KEY, &Validation::default()).unwrap().claims
//}
//
///// 檢測登錄信息
//pub fn check_token(req: &HttpRequest) -> Option<UserToken> { 
//    let headers = req.headers();
//    let auth_string = match headers.get(AUTHORIZATION) { 
//        Some(v) => { v.to_str().unwrap() },
//        None => { return None; }
//    };
//    match jsonwebtoken::decode::<UserToken>(auth_string, &KEY, &Validation::default()) { 
//        Ok(v) => { Some(v.claims) },
//        Err(_) => { None }
//    }
//}

/// 獲得token信息
//pub fn get_token_by<T: DeserializeOwned>(req: &HttpRequest, auth_key: &str, key: &[u8; 16]) -> T {
//    let headers = req.headers();
//    let auth_string = headers.get(auth_key).unwrap().to_str().unwrap();
//    jsonwebtoken::decode::<T>(auth_string, key, &Validation::default()).unwrap().claims
//}

/// 从url地址得到網頁內容
// pub unsafe fn get_contents(url: &str) -> String {
//     let mut data = Vec::new();
//     let mut handle = Easy::new();
//     handle.url(url).unwrap();
//     {
//         let mut transfer = handle.transfer();
//         transfer.write_function(|new_data| {
//             data.extend_from_slice(new_data);
//             Ok(new_data.len())
//         }).unwrap();
//         transfer.perform().unwrap();
//     }
//     data.shrink_to_fit();
//     String::from_utf8_unchecked(data)
// }

/// 转换查询条件
pub fn get_queries(query_string: &str) -> HashMap<&str, &str> { 
    let queries: Vec<&str> = query_string.split("&").collect();
    let mut params = HashMap::new();
    for query in &queries { 
        let parts: Vec<&str> = query.split("=").collect();
        if parts.len() > 1 { 
            params.insert(parts[0], parts[1]);
        }
    }
    params
}
