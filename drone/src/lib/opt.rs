use structopt::StructOpt;
use crate::lib::append_hosts;


#[derive(Debug, StructOpt)]
pub struct Opt {
    #[structopt(long, env="PLUGIN_NAMESPACE", default_value = "default")]
    pub namespace: String,

    #[structopt(long, env="PLUGIN_KUBERNETES_PORT", default_value = "6443")]
    pub port: String,

    #[structopt(long, env="PLUGIN_KUBERNETES_PROTOCOL", default_value = "https")]
    pub protocol: String,

    #[structopt(long, env="PLUGIN_HOSTNAME")]
    pub hostname: Option<String>,

    #[structopt(long, short, env="PLUGIN_KUBERNETES_SERVER")]
    pub server: String,

    #[structopt(long, short, env="PLUGIN_KUBERNETES_TOKEN")]
    pub token: String,

    #[structopt(long, env="PLUGIN_KUBERNETES_CERT")]
    pub cert: String,

    #[structopt(long, short, env="PLUGIN_SCRIPT")]
    pub cmd: Option<String>,

    #[structopt(long)]
    pub dry_run: bool,
}

use std::io::Result as IoResult;
impl Opt {
    pub fn url<'a>(&'a self,  dry_run: &'a bool) -> IoResult<String> {
        match &self.hostname {
            Some(h) => {
                if *dry_run {
                    println!("write to file `/etc/hosts`: {} {}", &self.server, &h);
                } else {
                    append_hosts("/etc/hosts", &self.server, &h)?;
                }
                Ok(format!("{}://{}:{}", &self.protocol, &h, &self.port))
            },
            None => {
                Ok(format!("{}://{}:{}", &self.protocol, &self.server, &self.port))
            }
        }
    }
}
