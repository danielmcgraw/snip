use crate::store;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "pow", about = "Text snippets on the command line!")]
pub struct Opts {
    #[structopt(short, long, default_value = "", index = 1)]
    command: String,

    #[structopt(short, long, default_value = "", index = 2)]
    list: String,

    #[structopt(short, long, default_value = "", index = 3)]
    key: String,

    #[structopt(short, long, default_value = "", index = 4)]
    value: String,
}

pub fn run_command(opts: Opts) {
    // println!("{:?}", opts);
    let mut store = store::Store::new();

    match opts.command.as_str() {
        "get" => {
            if opts.list != "" && opts.key != "" {
                store.copy_list_entry(&opts.list, &opts.key);
            }
        },
        "show" => {
            if opts.list != "" && opts.key != "" {
                store.print_list_entry(&opts.list, &opts.key);
            } else if opts.list != "" {
                if opts.list == "all" {
                    store.print_all();
                } else {
                    store.print_list(&opts.list);
                }
            } else {
                eprintln!("Not able to get", );
            }
        },
        "put" => {
            if opts.list != "" && opts.key != "" && opts.value != "" {
                store.add_list_entry(&opts.list, &opts.key, &opts.value);
            } else if opts.list != "" {
                store.add_list(&opts.list);
            } else {
                eprintln!("Not able to put", );
            }
        },
        "del" => {
            if opts.list != "" && opts.key != "" {
                store.delete_list_entry(&opts.list, &opts.key);
            } else if opts.list != "" {
                store.delete_list(&opts.list);
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
