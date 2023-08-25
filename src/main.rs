use ::std::*;
use std::io::Read;
use std::fs::File;
use std::path::Path;
extern crate csv;
use csv::Reader;
use zabbix_api::api::{ZabbixApi, Method, Query};
use serde::Deserialize;
use serde_json::{json, Value};


 #[derive(Debug, Deserialize, Clone)]
 struct Host {
    ip: String,
    hostname: String,
    gid: String,
    tid: String,
    snmp: String
}


#[tokio::main]
async fn main(){

    let config_path = Path::new("config.json");

    let connection: ZabbixApi = match File::open(&config_path) {
        Ok(mut file) => {
            let mut buff = String::new();
            if file.read_to_string(&mut buff).is_err() {
                println!("Failed to read config file");
                get_connection_info()
            } else {
                match serde_json::from_str(&buff) {
                    Ok(v) => v,
                    Err(e) => {
                        println!("Failed to parse config file: {}", e);
                        get_connection_info()
                    }
                }
            }
        }
        Err(_) => {
            println!("Config file not found");
            get_connection_info()
        }
    };    

loop {
     let mut choice: String = String::new();
        println!("Select option:");
        println!("1: Add Hosts");
        println!("2: Test API");
        println!("3: Send custom JSON request");
        println!("4: Exit");
        println!("5: Find and replace host names (EXPERIMENTAL)");
        io::stdin().read_line(&mut choice).expect("Please enter a valid option");
        //let choice: i32 = choice.trim().parse().expect("Please type a number!");
        let choice: u8 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue
            }
       };
        if choice == 1 {
            add_hosts(&connection).await.map_err(|err| println!("{:?}", err)).ok();
            continue;
        }
        else if choice == 2 { 
            api_test(&connection).await.map_err(|err| println!("{:?}", err)).ok();
            continue;
        }
        else if choice == 3 { 
            custom_request(&connection).await.map_err(|err| println!("{:?}", err)).ok();
            continue;
        }
        else if choice == 4 {
            break;
        }
        else if choice == 5 {
            hostname_find_and_replace(&connection).await.map_err(|err| println!("{:?}", err)).ok();
            continue;
        }
        else {
            println!("Please select a valid option");
            continue;
        }
    }
    println!("Goodbye");
}

fn get_user_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Please enter a valid option");
    input
}

fn get_connection_info() -> ZabbixApi {

    let mut server = String::new();
    let mut api_key = String::new();


    println!("Enter Zabbix API URL (e.g. http://127.0.0.1/zabbix/api_jsonrpc.php)):");
    io::stdin().read_line(&mut server).expect("Failed to read line");
    let server: String = server.trim().parse().expect("Invalid string!");

    println!("Enter API key:");
    io::stdin().read_line(&mut api_key).expect("Failed to read line");
    let api_key: String = api_key.trim().parse().expect("Invalid string!");

    ZabbixApi::new(&server, &api_key)

}

async fn custom_request(conn: &ZabbixApi) -> Result<(), Box<dyn std::error::Error>> {

    let mut method: String = String::new();
    println!("Enter method (e.g. host.get):");
    io::stdin().read_line(&mut method).expect("Failed to read line");
    let method: String = method.trim().parse().expect("Invalid string!");

    let mut params_input: String = String::new();
    println!("Enter params (JSON, one line):");
    io::stdin().read_line(&mut params_input).expect("Failed to read line");

    let mut requires_auth: String = String::new();
    println!("Requires auth? (y/n):");
    io::stdin().read_line(&mut requires_auth).expect("Failed to read line");
    let requires_auth: String = requires_auth.trim().parse().expect("Invalid string!");

    let requires_auth: bool = match requires_auth.as_str() {
        "y" => true,
        "n" => false,
        _ => false
    };

    match serde_json::from_str(&params_input) {
        Ok(v) => 
        {
            let result = conn.custom_request(&method, v, requires_auth).await?;
            println!("{:?}", result);

        }
        Err(e) => {
            println!("Error: {}", e);
    }
    }



    Ok(())
}

async fn api_test(conn: &ZabbixApi) -> Result<(), Box<dyn std::error::Error>> {

    let query = Query::new(Method::APIInfoVersion);

    let result = conn.request_query(query).await?;

    println!("{:?}", result);

    Ok(())
}


async fn add_hosts(conn: &ZabbixApi) -> Result<(), Box<dyn std::error::Error>> {

    let mut choice: String = String::new();
    let mut visiblename: String = String::new();
    let mut ipaddress: String = String::new();
    let mut groupid: String = String::new();
    let mut templateid: String = String::new();
    //let mut snmpstring: String = String::new();

    println!("(Add Hosts) Select option:");
    println!("1: Add Manually");
    println!("2: Add from CSV");
    println!("3: Exit");
    io::stdin().read_line(&mut choice).expect("Please enter a valid option");
    let choice: i32 = choice.trim().parse().expect("Please type a number!");

    if choice == 1 {
        //println!("Enter Host Name:");
        //io::stdin().read_line(&mut hostname).expect("Failed to read line");
        //let hostname: String = hostname.trim().parse().expect("Invalid string!");
        let hostname:String = get_user_input("Enter Host Name:").trim().parse().expect("Invalid string!");

        println!("Enter Visible Name:");
        io::stdin().read_line(&mut visiblename).expect("Failed to read line");
        let hostname: String = hostname.trim().parse().expect("Invalid string!");

        println!("Enter IP Address:");
        io::stdin().read_line(&mut ipaddress).expect("Failed to read line");
        let ipaddress: String = ipaddress.trim().parse().expect("Invalid string!");

        println!("Enter Group ID:");
        io::stdin().read_line(&mut groupid).expect("Failed to read line");
        let groupid: String = groupid.trim().parse().expect("Invalid string!");

        println!("Enter Template ID:");
        io::stdin().read_line(&mut templateid).expect("Failed to read line");
        let templateid: String = groupid.trim().parse().expect("Invalid string!");

        let mut query = Query::new(Method::HostCreate);
        let params = json!({
            "host": hostname,
            "name": visiblename,
            "interfaces": [
                {
                    "type": 2,
                    "ip": ipaddress,
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
                    "templateid": templateid
                }
            ],
            "inventory_mode": 0
        });

        query.params = params;
    
       let result = conn.request_query(query).await?;

       println!("{:?}", result);

    }
    else if choice == 2 { 
        let mut csvloc = String::new();
        
        println!("Enter CSV location/file name (e.g. <hosts.csv> (if in current dir):");
        io::stdin().read_line(&mut csvloc).expect("Failed to read line");
        let csvloc: String = csvloc.trim().parse().expect("Invalid string!");

        let mut rdr = Reader::from_path(csvloc)?;
        //let mut rows: Vec<Host> = Vec::new();
        for result in rdr.records() {
            let record = result?;
            println!("{:?}", record);
            //Build struct, then send to JSON in this loop?
            let host: Host = record.deserialize(None)?;
            println!("{}", host.ip);

            let mut query = Query::new(Method::HostCreate);
            query.params = json!({
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
            });


            let result = conn.request_query(query).await?;

            println!("{:?}", result);
            
        }

    }
    else if choice == 3 {
        return Ok(());
    }
    else {
        println!("Please select a valid option");
    }

  Ok(())
}

async fn hostname_find_and_replace(conn: &ZabbixApi) -> Result<(), Box<dyn std::error::Error>> {
    //Ask user for input string for host name search
    let mut find: String = String::new();
    println!("Enter host name find string (case-sensitive):");
    io::stdin().read_line(&mut find).expect("Failed to read line");
    let find: String = find.trim().parse().expect("Invalid string!");

    //Ask user for input string for host name replacement (to)
    let mut replace: String = String::new();
    println!("Enter host name replace string:");
    io::stdin().read_line(&mut replace).expect("Failed to read line");
    let replace: String = replace.trim().parse().expect("Invalid string!");

    let query = Query::new(Method::HostGet).add_search("name", &find).set_output(vec!["hostid", "name"]);
    let result = conn.request_query(query).await?;
    println!("Result: {:?}", result);
    
    let result = result.result;
    let pairs = result.as_array().unwrap();

    let mut params_list: Vec<Value> = Vec::new();

    for host in pairs {
        //Zabbix search is case insensitive, so only pick out hosts where case matches
        let hostname = host["name"].as_str().unwrap();
        if hostname.contains(&find) {
            let hostid = host["hostid"].as_str().unwrap();
            let hostname = host["name"].as_str().unwrap();
            let hostname = hostname.replace(&find, &replace);
            //let method = Method::HostUpdate;
            let params = json!({
                "hostid": hostid,
                "name": hostname
            });
            params_list.push(params);
            //println!("{:?}", params);
            //let result = conn.request(method, params).await?;
            //println!("{:?}", result);
        }
    }

    //If params is not empty, show the user the list of hosts that will change and ask for confirmation
    if !params_list.is_empty() {
        println!("The following hosts will be changed:");
        for host in &params_list {
            println!("{}", host["name"]);
        }
        let mut confirm: String = String::new();
        println!("Are you sure you want to continue? (y/n):");
        io::stdin().read_line(&mut confirm).expect("Failed to read line");
        let confirm: String = confirm.trim().parse().expect("Invalid string!");
        if confirm == "y" {
            for item in &params_list {
                let mut query = Query::new(Method::HostUpdate);
                let params = item.clone();
                query.params = params;
                let result = conn.request_query(query).await?;
                println!("{:?}", result);
            }
        }
        else {
            println!("Host names will not be changed");
        }
    }
    else {
        println!("No hosts found");
    }

    Ok(())

}