use std::collections::HashMap;
use std::fs::File;
use url::Url;

use crate::error::Result;
use crate::models::{Domain, Referer, Referers};

#[derive(Default)]
pub struct RefDb {
    pub data: HashMap<Domain, Referer>,
}

impl RefDb {
    pub fn from_json(path: &str) -> Result<Self> {
        let file = File::open(path)?;
        let ref_data: Referers = serde_json::from_reader(file)?;
        let mut data = HashMap::new();

        for (medium, names) in ref_data.data {
            for (name, ref_source) in names {
                let params = match ref_source.parameters {
                    Some(p) => p,
                    None => vec![],
                };
                for domain in ref_source.domains {
                    data.insert(
                        domain,
                        Referer {
                            source: name.clone(),
                            medium: medium.clone(),
                            params: params.clone(),
                        },
                    );
                }
            }
        }

        Ok(Self { data })
    }

    pub fn lookup(&self, ref_url: &Url) -> Option<&Referer> {
        if ref_url.scheme() != "http" && ref_url.scheme() != "https" {
            return None;
        }
        let host = match ref_url.host_str() {
            None => return None,
            Some(h) => h,
        };

        self._lookup_referer(host, ref_url.path(), true)
            .or_else(|| self._lookup_referer(host, ref_url.path(), false))
    }

    fn _lookup_referer(
        &self,
        host: &str,
        path: &str,
        include_path: bool,
    ) -> Option<&Referer> {
        let mut keys = vec![host.to_string() + &path, host.to_string()];
        if include_path && path.matches('/').count() > 1 {
            let mut path_parts = path.split('/');
            path_parts.next();
            if let Some(p) = path_parts.next() {
                keys.push(host.to_string() + "/" + p);
            }
        }
        for k in keys {
            if let Some(v) = self.data.get(&k) {
                return Some(v);
            }
        }

        None
    }
}
