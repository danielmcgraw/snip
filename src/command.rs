use crate::store;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "snip", about = "Text snippets on the command line!")]
pub struct Opts {
    #[structopt(short = "c", long = "command", default_value = "", index = 1)]
    command: String,

    #[structopt(short = "t", long = "topic", default_value = "", index = 2)]
    topic: String,

    #[structopt(short = "k", long = "key", default_value = "", index = 3)]
    key: String,

    #[structopt(short = "v", long = "value", default_value = "", index = 4)]
    value: String,
}

pub fn run_command(opts: Opts) {
    // println!("{:?}", opts);
    let mut store = store::Store::new();

    match opts.command.as_str() {
        "get" => {
            if opts.topic != "" && opts.key != "" {
                store.copy_list_entry(&opts.topic, &opts.key);
            }
        },
        "show" | "echo" => {
            if opts.topic != "" && opts.key != "" {
                store.print_list_entry(&opts.topic, &opts.key);
            } else if opts.topic != "" {
                if opts.topic == "all" {
                    store.print_all();
                } else {
                    store.print_list(&opts.topic);
                }
            } else {
                eprintln!("Not able to get", );
            }
        },
        "open" => {
            if opts.topic != "" && opts.key != "" {
                store.open_list_entry(&opts.topic, &opts.key);
            }
        }
        "put" => {
            if opts.topic != "" && opts.key != "" && opts.value != "" {
                store.add_list_entry(&opts.topic, &opts.key, &opts.value);
            } else if opts.topic != "" {
                store.add_list(&opts.topic);
            } else {
                eprintln!("Not able to put", );
            }
        },
        "del" => {
            if opts.topic != "" && opts.key != "" {
                store.delete_list_entry(&opts.topic, &opts.key);
            } else if opts.topic != "" {
                store.delete_list(&opts.topic);
            } else {
                eprintln!("Not able to put", );
            }
        },
        "nuke" => {
            store.nuke();
        }
        _ => println!("unrecognized command"),
    }

    // println!("{:?}", store);
    store.write_out();
}
