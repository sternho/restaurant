use std::collections::HashMap;

#[derive(PartialEq)]
pub(crate) enum Action {
    Create,
    Delete,
    Query,
    None,
}

impl Action {
    pub fn action_parse(request:HashMap<String, String>) -> (Action, bool) {
        let url = request.get("api").unwrap();
        let table_id = request.get("table_id");
        let item_id = request.get("item_id");
        let order_id = request.get("order_id");

        if url.eq(&format!("/create")) {
            (Action::Create, table_id.is_some() && item_id.is_some())
        } else if url.eq(&format!("/delete")) {
            (Action::Delete, table_id.is_some() && order_id.is_some())
        } else if url.eq(&format!("/check")) {
            (Action::Query, table_id.is_some())
        } else {
            (Action::None, true)
        }
    }

    pub(crate) fn get_url_and_parameter(buffer:[u8;1024]) -> HashMap<String, String> {
        let url = Action::get_request_para(buffer);
        let url = url.get(0).unwrap();
        println!("url: {}", url);
        let url = url.split(" ");
        let mut info = Vec::new();
        url.for_each(|x| info.push(x.to_string()));
        let mut para = HashMap::new();
        para.insert(String::from("method"), info.get(0).unwrap().to_string());

        let link = info.get(1).unwrap();
        let index = link.find("?").unwrap_or(link.len());
        let api:String = link.chars().take(index).collect();
        para.insert(String::from("api"), api);

        let request_para:String = link.chars().skip(index+1).collect();
        let request_para = request_para.split("&");
        request_para.for_each(|param| {
            println!("request para: {}", param);
            let mut param = param.split("=");
            let key = param.next();
            let value = param.next();
            if key.is_some() && value.is_some() {
                let key = key.unwrap().to_string();
                let value = value.unwrap().to_string();
                para.insert(key, value);
            }
        });

        return para;
    }

    fn get_request_para(buffer:[u8;1024]) -> Vec<String> {
        let data = String::from_utf8(buffer.to_vec()).unwrap();
        let data = data.lines();
        let mut para = Vec::new();
        for x in data {
            para.push(x.to_string());
        }
        return para;
    }

}

// fn compare(url:&String, target:&str) -> bool {
//     let tmp = String::from(target);
//     url.eq(&tmp)
// }
