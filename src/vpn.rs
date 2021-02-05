use log::{debug, trace};

use crate::actions::{Executable, Identifiable};

pub(crate) const VPN_CONNECT_CMD: &'static str = "vpn connect";
pub(crate) const VPN_TOKEN: &'static str = "token";

pub struct ConnectVPN {
    pub msg: String
}

impl Identifiable for ConnectVPN {
    fn id(&self) -> String {
        return "vpn connect".to_string();
    }
}

impl Executable for ConnectVPN {
    fn execute(&self) -> String {
        let message_parts: Vec<&str> = self.msg.split(VPN_TOKEN).collect();
        let encoded_vpn_token = message_parts[1].trim();

        debug!("Encoded VPN token {}", encoded_vpn_token);

        return match base64::decode(encoded_vpn_token) {
            Ok(vpn_token) => {
                trace!("Token {} decrypted to {}", encoded_vpn_token, vpn_token);
                let result_message = format!("Token {} successfully decrypted", encoded_vpn_token);
                debug!("{}", &result_message);
                result_message
            }
            Err(err) => {
                let result_message = format!("Could not decode vpn token: {}", err);
                debug!("{}", &result_message);
                result_message
            }
        };
    }
}


