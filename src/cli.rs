/*!
# LockNote CLI
Uses `clap` for argument parsing. Commands:
- `init` - initialize lockfile
- `add` - add note
- `list` - list all notes
- `view <id>` - view note
- `delete <id>` - delete note
- `search <keyword>` - search notes
*/

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "LockNote")]
#[command(about = "Secure encrypted notes CLI", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}


#[derive(Subcommand, Debug)]
pub enum Commands {
    Init { file: Option<String> },
    Add { file: Option<String> },
    List { file: Option<String> },
    View { id: String, file: Option<String> },
    Delete { id: String, file: Option<String> },
    Search { keyword: String, file: Option<String> },
}
