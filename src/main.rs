use serde_json::json;
use serde::Deserialize;
use ::std::*;
use std::io::Read;
use std::fs::File;
extern crate csv;
use csv::Reader;

#[derive(Debug, Clone, Deserialize)]
 struct Connection {
        server: String,
        username: String,
        password: String,
        token: String
 }

 #[derive(Debug, Deserialize, Clone)]
 struct Host {
    ip: String,
    hostname: String,
    gid: String,
    tid: String,
    snmp: String
}

fn main(){

    //Try to load config
    let mut file = File::open("config.json").unwrap();
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();
 
    let mut conn_string: Connection = serde_json::from_str(&buff).unwrap();
    //check if contents are OK, otherwise prompt (get_server)

    let cloned_string = conn_string.clone();
    //api_test(&conn_string).map_err(|err| println!("{:?}", err)).ok();

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
            add_hosts(&conn_string).map_err(|err| println!("{:?}", err)).ok();
            continue;
        }
        else if choice == 2 { 
            api_test(&conn_string).map_err(|err| println!("{:?}", err)).ok();
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

fn get_server() -> Connection {

    let mut zbxsrv = String::new();
    let mut user = String::new();
    let mut pass = String::new();
    let mut authtoken = String::new();

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

    println!("Enter Auth Token:");
    io::stdin().read_line(&mut authtoken).expect("Failed to read line");
    let authtoken: String = authtoken.trim().parse().expect("Invalid string!");

let conn_string = Connection {
    server: zbxsrv,
    username: user,
    password: pass,
    token: authtoken
};

return conn_string;
}

fn api_test(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {

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

    send_request(&conn, request);

    Ok(())
}

#[tokio::main]
async fn send_request(conn: &Connection, req: serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
    
    //println!("Your request looks like:\n{}\n", serde_json::to_string_pretty(&req).unwrap());
    let client = reqwest::Client::new();
    let response = client.post(&conn.server)
                         .json(&req)
                         .send()
                         .await?;

    let content: serde_json::Value = response.json().await?;
    println!("{:#?}", content);
    Ok(())
}

fn add_hosts(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {

    let mut choice: String = String::new();
    let mut ipaddress: String = String::new();
    let mut hostname: String = String::new();
    let mut groupid: String = String::new();
    let mut templateid: String = String::new();
    let mut snmpstring: String = String::new();

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

        let request: serde_json::Value = json!({
            "jsonrpc": "2.0",
            "method": "host.create",
            "params": {
                "host": hostname.trim(),
                "name": hostname.trim(),
                "interfaces": [
                    {
                        "type": 2,
                        "ip": ipaddress.trim(),
                        "dns": "",
                        "useip": 1,
                        "main": 1,
                        "port": "161",
                        "details": {
                            "version": 2,
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
                "inventory_mode": 0
            },
            "auth": conn.token,
            "id": 1
        });
    
       send_request(conn, request);

    }
    else if choice == 2 { 
        let mut csvloc = String::new();
        
        println!("Enter CSV location/file name (e.g. <hosts.csv> (if in current dir):");
        io::stdin().read_line(&mut csvloc).expect("Failed to read line");
        let csvloc: String = csvloc.trim().parse().expect("Invalid string!");

        let mut rdr = Reader::from_path(csvloc)?;
        let mut rows: Vec<Host> = Vec::new();
        for result in rdr.records() {
            let record = result?;
            println!("{:?}", record);
            //Build struct, then send to JSON in this loop?
            let host: Host = record.deserialize(None)?;
            println!("{}", host.ip);

            let request: serde_json::Value = json!({
                "jsonrpc": "2.0",
                "method": "host.create",
                "params": {
                    "host": host.ip.trim(),
                    "name": host.hostname.trim(),
                    "interfaces": [
                        {
                            "type": 2,
                            "ip": host.ip.trim(),
                            "dns": "",
                            "useip": 1,
                            "main": 1,
                            "port": "161",
                            "details": {
                                "version": 2,
                                "community": host.snmp.trim(),
                            },
                        "interface_ref": "if1"
                        }
                    ],
                    "groups": [
                        {
                            "groupid": host.gid.trim()
                        }
                    ],
                    "templates": [
                        {
                            "templateid": host.tid.trim()
                        }
                    ],
                    "inventory_mode": 0
                },
                "auth": conn.token,
                "id": 1
            });

            //println!("Your request looks like:\n{}\n", serde_json::to_string_pretty(&request).unwrap());
            send_request(&conn, request);
        }

    }
    else if choice == 3 {
        return Ok(());
    }
    else {
        println!("Please select a valid option");
    }
        
    //If you receive JSON result back, connection appears to be successful.  Error should be passed otherwise on connection failure.
  Ok(())
}