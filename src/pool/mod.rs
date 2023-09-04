use std::{path::PathBuf, fs::{self, File}, io::Write};
use crate::{config::vmids::Vmid, Error};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Index(Vec<PathBuf>);

pub(crate) fn new_pool(dir: PathBuf, pool: Vec<Vmid>) -> Result<(), Error> {
    let mut pool_paths = Vec::new();
    if dir.exists() {
        debug!("{} exists: skiping..", dir.display().to_string())
    } else {
        fs::create_dir_all(dir.clone()).map_err(Error::Io)?;
    }

    for vmid in pool {
        let mut path = dir.join(vmid.vmid_number.clone().to_string());

        if path.exists() {
            debug!("{} exists: skiping..", path.display().to_string())
        } else {
            fs::create_dir(path.clone()).map_err(Error::Io)?;
            pool_paths.push(path.clone());
            path.push("config.json");

            let serialized = serde_json::to_string_pretty(&vmid).map_err(Error::ConfigError)?;
            File::create(path).map_err(Error::Io)?.write_all(serialized.as_bytes()).map_err(Error::Io)?;
        }
    }
    File::create(dir.join("index.json")).map_err(Error::Io)?.write_all(serde_json::to_string_pretty(&Index(pool_paths)).map_err(Error::ConfigError)?.as_bytes()).map_err(Error::Io)?;

    Ok(())
}

pub(crate) fn load_pool(dir: PathBuf) -> Result<Vec<Vmid>, Error> {
    info!("{}", dir.clone().display());
    let mut pool = Vec::new();

    let index: Index = serde_json::from_str(
        &fs::read_to_string(dir.join("index.json"))
        .map_err(Error::Io)?)
        .map_err(Error::ConfigError)?;

    for path in index.0 {
        info!("{}", path.display().to_string());
        
        let mut vmid: Vmid = serde_json::from_str(
            &fs::read_to_string(path.join("config.json"))
            .map_err(Error::Io)?)
            .map_err(Error::ConfigError)?;

        vmid.path = Some(path);

        pool.push(vmid)
    }

    Ok(pool)
}

#[cfg(test)]
mod tests {
    use crate::{config::vmids::new::{self, DestinationOption}, Error};
    use super::{load_pool, new_pool};
    use std::{path::PathBuf, fs};

    #[test]
    fn new_pool_test() -> Result<(), Error> {
        let path = PathBuf::from("./pool/test/1");

        if path.clone().exists() {
            fs::remove_dir_all(path.clone()).map_err(|e| Error::Io(e))?; 
        }

        new_pool(path, new::vmid(DestinationOption::Tcp(9500), 4)?)?;

        // fs::remove_dir_all("./pool/test").map_err(|e| Error::Io(e))?;
        Ok(())
    }

    #[test]
    fn load_pool_test() -> Result<(), Error> {
        let path = PathBuf::from("./pool/test/2");

        if path.clone().exists() {
            fs::remove_dir_all(path.clone()).map_err(|e| Error::Io(e))?; 
        }

        new_pool(path.clone(), new::vmid(DestinationOption::Tcp(9500), 4)?)?;

        let pool = load_pool(path)?;

        // fs::remove_dir_all("./pool/testload").map_err(|e| Error::Io(e))?;

        assert_eq!(pool[0].vmid_number, 0);
        // assert_eq!(pool[0].port, 5900);
        assert_eq!(pool[0].name, "No Name".to_string());
        Ok(())
    }
}