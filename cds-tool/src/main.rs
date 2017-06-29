// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]
#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate log;
extern crate env_logger;

extern crate clap;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate iron;

mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain! {}
}

mod server;
mod docker;
mod run;

use std::path::PathBuf;
use std::io::Read;
use std::io::Seek;
use std::io::Write;
//use std::slice::SliceConcatExt;

use errors::*;

macro_rules! finally {
    ($contents:block) => {
        struct A<F: Fn() -> ()> {c : F};

        impl<F> Drop for A<F> where F: Fn() -> () {
            fn drop(&mut self) {
                (self.c)();
            }
        }

        #[allow(unused)]
        let a = A{c: || { $contents },};
    };
}

fn main() {
    env_logger::init().unwrap();

    if let Err(ref e) = run() {
        println!("error: {}", e);

        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:?}", backtrace);
        }

        ::std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let matches = clap::App::new("CDS multi-tool")
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .subcommand(clap::SubCommand::with_name("run")
            .setting(clap::AppSettings::ArgRequiredElseHelp)
            .about("Invoke the testing and measuring functions of the CDS tool")
            .arg_from_usage("--image=[image] 'Docker image to use'")
            .arg_from_usage("--container=[container] 'Docker container to use'")
            //.arg_from_usage("-p --port 'Private port the CDS server is listining on'")
            .arg_from_usage("-i --input=<input> 'File containing the problem (required)'")
            .arg_from_usage("-o --output=[output] 'File containing the expected output/solution'")
            .arg_from_usage("-w --write=[write] 'Write output to this file'")
            .arg_from_usage("-m --measure 'Measure-mode, run multiple times and output csv-like data (duration in micro seconds)'")
            .arg_from_usage("-c --cpus=[cpus] 'Number of cpus the application is allowed to use (can be a comma-separated list in measure-mode) [default: 1,2,4,8]'")
            .arg_from_usage("-r --runs=[runs] 'Number of runs to average the program runtime (only in measure-mode) [default: 3]'")
            .arg_from_usage("<program> 'The program to start'")
            .group(clap::ArgGroup::with_name("unit-under-test")
                .args(&["image", "container"])
                .required(true)))
        .subcommand(clap::SubCommand::with_name("server")
            .about("Start the CDS server serving requests of the test tool")
            //.arg_from_usage("-p --port=[port] 'TCP/IP port the server will listen for requests on (default: 8080)'")
            .arg_from_usage("-c --config=[config] 'Load the server configuration from this file (default: ~/.config/cds_server.json)'"))
        .get_matches();

    match matches.subcommand() {
        ("server", Some(sub_m)) => {
            let config_path = match sub_m.value_of("config") {
                Some(c) => PathBuf::from(c),
                None    => {
                    match std::env::home_dir() {
                        Some(h) => h.join(".config/cds_server.json"),
                        None    => bail!("Unable to locate home directory to load server config! You might work around this by explicly specifying the server's config file."),
                    }
                }
            };
            let port = try!(sub_m.value_of("port").unwrap_or("8080").parse().chain_err(|| "Unable to parse given port"));
            let server = try!(server::Server::new(&config_path.as_path(), port));
            let _ = try!(server.run());
        },
        ("run", Some(sub_m)) => {
            let program = sub_m.value_of("program").expect("program is strictly necessary");
            let port: u16 = try!(sub_m.value_of("port").unwrap_or("8080").parse().chain_err(|| "Unable to parse given port"));

            let input_path = sub_m.value_of("input").expect("input is strictly necessary");
            let mut input_file = try!(std::fs::File::open(input_path).chain_err(|| format!("unable to open input file {} containing the problem to be solved", input_path)))    ;
            let mut input: Vec<u8> = Vec::new();
            try!(input_file.read_to_end(&mut input).chain_err(|| format!("unable to read content of input file {}", input_path)));

            let expected_output = if let Some(output_path) = sub_m.value_of("output") {
                let mut output_file = try!(std::fs::File::open(output_path).chain_err(|| format!("unable to open the file containing the expected output (solution) {}", output_path)));
                let mut output = String::new();
                try!(output_file.read_to_string(&mut output).chain_err(|| format!("unable to read content of output file {}", output_path)));
                Some(output)
            } else {
                None
            };

            let mut write_to = if let Some(write_to_path) = sub_m.value_of("write") {
                Some(try!(std::fs::File::create(write_to_path).chain_err(|| format!("unable to open output file {} for writing", write_to_path))))
            } else {
                None
            };

            if sub_m.is_present("measure") {
                let image_id = sub_m.value_of("image").expect("image is strictly necessary in measure-mode!");

                println!("program; run; cpus; duration;");

                for r in 0..try!(sub_m.value_of("runs").unwrap_or("3").parse().chain_err(|| "Given runs is not a number")) {
                    for cpus in sub_m.value_of("cpus").unwrap_or("1,2,4,8").split(",") {
                        let cpus = try!(cpus.parse::<u32>().chain_err(|| "Given cpu count is not a positive number"));
                        let cpuset = (0..cpus).map(|x| format!("{}", x)).collect::<Vec<String>>().join(",");
                        let cid = try!(docker::start_container(image_id,
                                                               &["--cpuset-cpus", cpuset.as_str(),
                                                                 "-p", port.to_string().as_str(),
                                                                 "-e", format!("MAX_CPUS={}", cpus).as_str()
                                                                ])
                            .chain_err(|| "starting of measurement container failed"));
                        finally! {{
                            docker::stop_container(cid.as_str(), true);
                        }}
                        let (stdout, stderr, exit_status, duration) = try!(run::run(cid.as_str(), port, program, input.as_slice()).chain_err(|| "measuring failed"));
                        if exit_status != 0 {
                            bail!("measurment run terminated with non-zero exit status! exit_status: {}\nstdout:\n--------------\n{}--------------\nstderr:\n--------------\n{}--------------",
                                exit_status,
                                stdout,
                                stderr);
                        }
                        if let Some(ref eout) = expected_output {
                            if eout != &stdout {
                                bail!("measurement run terminated with unexpected output. actual stdout:\n--------------\n{}--------------\nexpected stdout:\n--------------\n{}--------------",
                                    stdout,
                                    eout);
                            }
                        }

                        if let Some(ref mut write_to) = write_to {
                            try!(write_to.seek(std::io::SeekFrom::Start(0)).chain_err(|| "unable to seek in output file"));
                            try!(write_to.write_all(stdout.as_bytes()).chain_err(|| "unable to write to output file"));
                        }

                        println!("{}; {}; {}; {}", program, r, cpus, duration);
                    }
                }
            } else {
                let (cid, created) = if let Some(image_id) = sub_m.value_of("image") {
                    let cpus = sub_m.value_of("cpus").unwrap_or("8");
                    if cpus.find(",").is_some() {
                        bail!("multiple, comma separated --cpus argument are only allowed in measurement-mode");
                    }
                    let cpus = try!(cpus.parse::<u32>().chain_err(|| "Given cpu count is not a positive number"));
                    let cpuset = (0..cpus).map(|x| format!("{}", x)).collect::<Vec<String>>().join(",");
                    let cid = try!(docker::start_container(image_id,
                                                           &["--cpuset-cpus", cpuset.as_str(),
                                                             "-p", port.to_string().as_str(),
                                                             "-e", format!("MAX_CPUS={}", cpus).as_str()
                                                            ])
                        .chain_err(|| "starting of measurement container failed"));
                    (cid, true)
                } else {
                    (sub_m.value_of("container").unwrap().to_string(), false)
                };

                    finally! {{
                        if created {
                            docker::stop_container(cid.as_str(), true);
                            println!("stopping container");
                        }
                    }}

                let (stdout, stderr, exit_status, duration) = try!(run::run(cid.as_str(), port, program, input.as_slice()).chain_err(|| "measuring failed"));
                println!("ran program {}\nexit status: {}\nduration: {} micro seconds\nstdout:\n--------------\n{}--------------\nstderr:\n--------------\n{}--------------",
                    program,
                    exit_status,
                    duration,
                    stdout,
                    stderr);

                if let Some(ref eout) = expected_output {
                    if eout != &stdout {
                        bail!("actual output differs from expected output:\n--------------\n{}--------------",
                            eout);
                    }
                }

                if let Some(ref mut write_to) = write_to {
                    try!(write_to.seek(std::io::SeekFrom::Start(0)).chain_err(|| "unable to seek in output file"));
                    try!(write_to.write_all(stdout.as_bytes()).chain_err(|| "unable to write to output file"));
                }
            }
        },
        _ => bail!("Application started without subcommand!"),
    };

    Ok(())
}
