use serde_json::json;
use ::std::*;
use std::io::Write;

#[derive(Debug, Clone)]
 struct Connection {
        server: String,
        username: String,
        password: String
 }

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

        let conn_string = Connection {
            server: zbxsrv,
            username: user,
            password: pass
        };

        let cloned_string = conn_string.clone();

        api_test(conn_string).map_err(|err| println!("{:?}", err)).ok();

loop {
    let mut choice: String = String::new();
        let conn_string = cloned_string.clone();
        println!("Select option:");
        println!("1: Add Hosts");
        println!("2: Test API");
        println!("3: Exit");
        io::stdin().read_line(&mut choice).expect("Please enter a valid option");
        let choice: i32 = choice.trim().parse().expect("Please type a number!");
        
        if choice == 1 {
            add_hosts(conn_string).map_err(|err| println!("{:?}", err)).ok();
            continue;
        }
        else if choice == 2 { 
            api_test(conn_string).map_err(|err| println!("{:?}", err)).ok();
            continue;
        }
        else if choice == 3 {
            break;
        }
        else {
            println!("Please select a valid option");
            continue;
        }
    }
    println!("Goodbye");
}

#[tokio::main]
async fn api_test(conn: Connection) -> Result<(), Box<dyn std::error::Error>> {

    //JSON Request Object
    let request = json!({
        "jsonrpc": "2.0",
        "method": "user.login",
        "params": {
            "user": conn.username,
            "password": conn.password
        },
        "id": 1,
        "auth": null
    });

    //println!("Your request looks like:\n{}\n", serde_json::to_string_pretty(&request).unwrap());
    println!("Your URL is {}", conn.server);

    let client = reqwest::Client::new();

    let response = client.post(&conn.server)
                         .json(&request)
                         .send()
                         .await?;

    let content: serde_json::Value = response.json().await?;
    
    //If you receive JSON result back, connection appears to be successful.  Error should be passed otherwise on connection failure.
    println!("{:#?}", content);

    Ok(())
}

#[tokio::main]
async fn add_hosts(conn: Connection) -> Result<(), Box<dyn std::error::Error>> {
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