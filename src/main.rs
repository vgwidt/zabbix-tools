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
    //println!("Your URL is {}", conn.server);

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

    let mut choice: String = String::new();
    let mut ipaddress: String = String::new();
    let mut hostname: String = String::new();
    let mut groupid: String = String::new();
    let mut authtoken: String = String::new();
    println!("(Add Hosts) Select option:");
    println!("1: Add Manually");
    println!("2: Add from CSV");
    println!("3: Exit");
    io::stdin().read_line(&mut choice).expect("Please enter a valid option");
    let choice: i32 = choice.trim().parse().expect("Please type a number!");

    if choice == 1 {
        println!("Enter IP Address:");
        io::stdin().read_line(&mut ipaddress).expect("Failed to read line");
        let ipaddress: String = ipaddress.trim().parse().expect("Invalid string!");

        println!("Enter Host Name:");
        io::stdin().read_line(&mut hostname).expect("Failed to read line");
        let hostname: String = hostname.trim().parse().expect("Invalid string!");

        println!("Enter Group ID:");
        io::stdin().read_line(&mut groupid).expect("Failed to read line");
        let groupid: String = groupid.trim().parse().expect("Invalid string!");

        println!("Enter Auth Token ID:");
        io::stdin().read_line(&mut authtoken).expect("Failed to read line");
        let authtoken: String = authtoken.trim().parse().expect("Invalid string!");

    }
    else if choice == 2 { 
        println!("Enter CSV file location (not yet working, will exit program");
        return Ok(());
    }
    else if choice == 3 {
        return Ok(());
    }
    else {
        println!("Please select a valid option");
    }
    
    let request: serde_json::Value = json!({
        "jsonrpc": "2.0",
        "method": "host.create",
        "params": {
            "host": hostname.trim(),
            "name": hostname.trim(),
            "interfaces": [
                {
                    "type": "SNMP",
                    "ip": ipaddress.trim(),
                    "port": "161",
                    "details": {
                        "community": "{$SNMP_COMMUNITY}",
                    },
                "interface_ref": "if1"
                }
            ],
            "groups": [
                {
                    "groupid": groupid.trim()
                }
            ],
            "templates": [
                {
                    "templateid": "10607"
                }
            ],
            "inventory_mode": "DISABLED"
        },
        "auth": authtoken.trim(),
        "id": 1
    });

    println!("Your request looks like:\n{}\n", serde_json::to_string_pretty(&request).unwrap());

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