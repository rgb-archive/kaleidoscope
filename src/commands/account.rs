// Kaleidoscope: RGB command-line wallet utility
// Written in 2019-2020 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
//     Alekos Filini <alekos.filini@gmail.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

use clap::Clap;
use lnpbp::bitcoin::util::bip32::DerivationPath;
use regex::Regex;

fn name_validator(name: &str) -> Result<(), String> {
    let re = Regex::new(r"^\w[\w\d_\-]{0,23}$").expect("Regex parse failure");
    if !re.is_match(&name) {
        Err(
            "Account name must be <24 chars, contain no spaces, consist only of \
            alphanumeric characters, dashes and underscores and start with \
            a letter\
            "
            .to_string(),
        )
    } else {
        Ok(())
    }
}

#[derive(Clap, Clone, Debug, Display)]
#[display_from(Debug)]
pub enum Command {
    /// Lists all known accounts
    List,

    /// Creates a new account under current keyring
    Create {
        /// Account tag name (must not contain spaces)
        #[clap(validator=name_validator)]
        name: String,

        /// Derivation path
        derivation_path: DerivationPath,

        /// Additional account information, like purpose
        description: Option<String>,
    },

    /// Lists detailed information about account-controlled "deposit boxes":
    /// items that may be managed with account private keys
    DepositBoxes {
        /// Amount of deposit boxes to list
        #[clap(short = "N", long, default_value = "10")]
        no: u8,

        /// Offset for the first deposit box
        #[clap(short = "O", long, default_value = "0")]
        offset: u32,

        /// Tag name of the account to list deposit boxes
        account: String,
    },
}
