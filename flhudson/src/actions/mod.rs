use {
    csv::ReaderBuilder,
    serde::Deserialize,
    std::{
        collections::HashMap,
        env,
        error::Error,
        ffi::OsStr,
        fs,
        path::Path,
        process,
        thread::{spawn, JoinHandle},
    },
    xlsxwriter::Workbook,
};

pub fn route(param: crate::Param) -> Result<(), Box<dyn Error>> {
    save_xlsx(
        &(env::var("HOME").unwrap() + "/report.xlsx"),
        &match param.case.as_str() {
            "csv" => open_csv(find_csv(&param.path)),
            "siege" => Siege::build(&param.path).run(),
            "virsh" => Virsh::build(&param.path).run(),
            &_ => HashMap::new(),
        },
    );
    Ok(())
}

pub fn find_csv(path: &str) -> Vec<String> {
    if let Ok(entries) = fs::read_dir(path) {
        let files: Vec<String> = entries
            .map(|entry| entry.ok().unwrap().path())
            .filter(|path| path.extension() == Some(OsStr::new("csv")))
            .map(|path| format!("{}", path.display()))
            .collect::<Vec<String>>();
        files
    } else {
        panic!("\x1b[31mERROR\x1b[0m")
    }
}

pub fn open_csv(source: Vec<String>) -> HashMap<String, Vec<Vec<String>>> {
    let mut data: HashMap<String, Vec<Vec<String>>> = HashMap::new();
    let mut children: HashMap<String, JoinHandle<Vec<Vec<String>>>> = HashMap::new();
    for file in source {
        let label = Path::new(&file).file_stem().unwrap().to_str().unwrap();
        children.insert(
            label.to_string(),
            spawn(move || -> Vec<Vec<String>> {
                ReaderBuilder::new()
                    .has_headers(false)
                    .from_reader(fs::read_to_string(file).unwrap().as_bytes())
                    .records()
                    .map(|row| {
                        row.unwrap()
                            .iter()
                            .map(str::to_string)
                            .collect::<Vec<String>>()
                    })
                    .collect()
            }),
        );
    }
    for (label, thread) in children {
        data.insert(label, thread.join().unwrap());
    }
    data
}

pub fn save_xlsx(output: &str, source: &HashMap<String, Vec<Vec<String>>>) {
    let workbook = Workbook::new(output).unwrap();
    for list in source.keys() {
        let mut sheet = workbook.add_worksheet(Some(list)).unwrap();
        for (row_ord, row) in source[list].iter().enumerate() {
            for (col_ord, cell) in row.iter().enumerate() {
                sheet
                    .write_string(row_ord as u32, col_ord as u16, cell, None)
                    .unwrap();
            }
        }
    }
    workbook.close().unwrap();
}

pub fn once() -> bool {
    if cfg!(target_os = "linux") {
        let run = process::Command::new("lsof")
            .args(["-t", &env::current_exe().unwrap().display().to_string()])
            .output()
            .expect("failed to execute bash");
        if run.status.success() {
            String::from_utf8_lossy(&run.stdout)
                .split_whitespace()
                .count()
                == 1
        } else {
            panic!("\x1b[31m{}\x1b[0m", String::from_utf8_lossy(&run.stderr))
        }
    } else {
        true
    }
}

#[derive(Deserialize)]
pub struct Siege {
    pub proto: String,
    pub port: u16,
    pub concurrent: u8,
    pub targets: Vec<String>,
    pub endpoints: Vec<String>,
}

pub trait Action {
    fn build(path: &str) -> Self;
    fn run(&self) -> HashMap<String, Vec<Vec<String>>>;
}

impl Action for Siege {
    fn build(path: &str) -> Self {
        serde_json::from_str(fs::read_to_string(path).unwrap().as_str()).unwrap()
    }
    fn run(&self) -> HashMap<String, Vec<Vec<String>>> {
        let mut children: HashMap<String, JoinHandle<Vec<Vec<String>>>> = HashMap::new();
        for node in self.targets.clone() {
            let endpoints: Vec<String> = self.endpoints.clone();
            let proto: String = self.proto.clone();
            let port: u16 = self.port;
            let concurrent: u8 = self.concurrent;
            children.insert(
                node.clone(),
                spawn(move || -> Vec<Vec<String>> {
                    let mut report: Vec<Vec<String>> = Vec::new();
                    report.push(Vec::from([
                        "endpoint".to_string(),
                        "transactions".to_string(),
                        "availability".to_string(),
                        "elapsed_time".to_string(),
                        "data_transferred".to_string(),
                        "response_time".to_string(),
                        "transaction_rate".to_string(),
                        "throughput".to_string(),
                        "concurrency".to_string(),
                        "successful_transactions".to_string(),
                        "failed_transactions".to_string(),
                        "longest_transaction".to_string(),
                        "shortest_transaction".to_string(),
                    ]));
                    for endpoint in endpoints {
                        report.push(get_raw(
                            format!("{}://{}:{}{}", proto, node, port, endpoint),
                            concurrent,
                        ));
                    }
                    report
                }),
            );
        }
        let mut data: HashMap<String, Vec<Vec<String>>> = HashMap::new();
        for (node, thread) in children {
            data.insert(node, thread.join().unwrap());
        }
        data
    }
}

fn get_raw(connect: String, concurrent: u8) -> Vec<String> {
    let run = process::Command::new("bash")
        .arg("-xc")
        .arg(format!(
            "siege --json-output --concurrent {} --time 1m  '{}'",
            concurrent, &connect
        ))
        .output()
        .expect("\x1b[31mFailed to execute siege!\x1b[0m");
    let result: HashMap<String, f32> = if run.status.success() {
        eprintln!("\x1b[32m{}\x1b[0m", String::from_utf8_lossy(&run.stderr));
        serde_json::from_str(&String::from_utf8_lossy(&run.stdout)).unwrap()
    } else {
        panic!("\x1b[31m{}\x1b[0m", String::from_utf8_lossy(&run.stderr))
    };
    vec![
        connect,
        result["transactions"].to_string(),
        result["availability"].to_string(),
        result["elapsed_time"].to_string(),
        result["data_transferred"].to_string(),
        result["response_time"].to_string(),
        result["transaction_rate"].to_string(),
        result["throughput"].to_string(),
        result["concurrency"].to_string(),
        result["successful_transactions"].to_string(),
        result["failed_transactions"].to_string(),
        result["longest_transaction"].to_string(),
        result["shortest_transaction"].to_string(),
    ]
}

#[derive(Deserialize)]
pub struct Virsh {
    pub nodes: Vec<usize>,
}

impl Action for Virsh {
    fn build(path: &str) -> Self {
        serde_json::from_str(fs::read_to_string(path).unwrap().as_str()).unwrap()
    }
    fn run(&self) -> HashMap<String, Vec<Vec<String>>> {
        let mut children: HashMap<String, JoinHandle<Vec<Vec<String>>>> = HashMap::new();
        for node in self.nodes.clone() {
            children.insert(
                node.to_string(),
                spawn(move || -> Vec<Vec<String>> { vm_list(node) }),
            );
        }
        let mut data: HashMap<String, Vec<Vec<String>>> = HashMap::new();
        for (node, thread) in children {
            data.insert(node, thread.join().unwrap());
        }
        data
    }
}
fn vm_list(node: usize) -> Vec<Vec<String>> {
    let run = process::Command::new("bash")
        .arg("-xc")
        .arg(format!("sshpass -e virsh --connect qemu+ssh://{}@192.168.25.{node}/system list --name --state-running", env::var("SSHUSER").unwrap()))
        .output()
        .expect("failed to execute bash");
    let mut list_raw: Vec<Vec<String>> = Vec::new();
    if run.status.success() {
        eprintln!("\x1b[32m{}\x1b[0m", String::from_utf8_lossy(&run.stderr));
        for pool in String::from_utf8_lossy(&run.stdout)
            .split_whitespace()
            .collect::<Vec<&str>>()
            .chunks(5)
        {
            let mut children: Vec<JoinHandle<Vec<String>>> = Vec::new();
            for line in pool {
                let vm: String = line.to_string();
                children.push(spawn(move || -> Vec<String> { vm_dump(node, vm) }));
            }
            for thread in children {
                list_raw.push(thread.join().unwrap())
            }
        }
    } else {
        panic!("\x1b[31m{}\x1b[0m", String::from_utf8_lossy(&run.stderr))
    };
    list_raw
}

#[derive(Deserialize)]
struct VM {
    name: String,
    memory: String,
    vcpu: String,
}

fn vm_dump(node: usize, name: String) -> Vec<String> {
    let run = process::Command::new("bash")
        .arg("-xc")
        .arg(format!(
            "sshpass -e virsh --connect qemu+ssh://admin@192.168.25.{}/system dumpxml {}",
            node, name
        ))
        .output()
        .expect("failed to execute bash");
    let vm: VM = if run.status.success() {
        eprintln!("\x1b[32m{}\x1b[0m", String::from_utf8_lossy(&run.stderr));
        serde_xml_rs::from_str(String::from_utf8_lossy(&run.stdout).to_string().as_str()).unwrap()
    } else {
        panic!("\x1b[31m{}\x1b[0m", String::from_utf8_lossy(&run.stderr))
    };
    vec![vm.name, vm.memory, vm.vcpu]
}
