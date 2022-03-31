use serde_json::json;
use ::std::*;
use std::io::Write;

fn main(){

    let mut zbxsrv = String::new();
    let mut user = String::new();
    let mut pass = String::new();

    println!("Enter Zabbix Host IP/Name (Include virtual directory if it exists, e.g. 127.0.0.1/zabbix):");
    io::stdin().read_line(&mut zbxsrv).expect("Failed to read line");
    let zbxsrv: String = zbxsrv.trim().parse().expect("Invalid string!");
    let zbxsrv = format!("http://{}/api_jsonrpc.php", zbxsrv);

    println!("Enter Username:");
    io::stdin().read_line(&mut user).expect("Failed to read line");
    let user: String = user.trim().parse().expect("Invalid string!");

    println!("Enter Password:");
    io::stdin().read_line(&mut pass).expect("Failed to read line");
    let pass: String = pass.trim().parse().expect("Invalid string!");

    api_test(zbxsrv, user, pass).map_err(|err| println!("{:?}", err)).ok();

    println!("done");
}

#[tokio::main]
async fn api_test(s: String, u: String, p: String) -> Result<(), Box<dyn std::error::Error>> {

    //JSON Request Object
    let request = json!({
        "jsonrpc": "2.0",
        "method": "user.login",
        "params": {
            "user": u,
            "password": p
        },
        "id": 1,
        "auth": null
    });

    //println!("Your request looks like:\n{}\n", serde_json::to_string_pretty(&request).unwrap());
    println!("Your URL is {}", s);

    let client = reqwest::Client::new();

    let response = client.post(&s)
                         .json(&request)
                         .send()
                         .await?;

    let content: serde_json::Value = response.json().await?;
    
    //If you receive JSON result back, connection appears to be successful.  Error should be passed otherwise on connection failure.
    println!("{:#?}", content);

    Ok(())
}

#[tokio::main]
async fn add_hosts(s: String, u: String, p: String) -> Result<(), Box<dyn std::error::Error>> {
/* host.create JSON structure
{
    "jsonrpc": "2.0",
    "method": "host.create",
    "params": {
        "host": "192.168.81.180",
        "interfaces": [
            {
                "type": 1,
                "main": 1,
                "useip": 1,
                "ip": "192.168.81.180",
                "dns": "",
                "port": "10050"
            }
        ],
        "groups": [
            {
                "groupid": "15"
            }
        ],
        "templates": [
            {
                "templateid": "10271"
            }
        ]
    },
    "auth": "'$token'",
    "id": 1
} */

Ok(())

}