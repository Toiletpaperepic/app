use crate::{config::vmids::Vmid, Error};
use std::{path::PathBuf, fs::{self, File}, io::Write};

pub(crate) fn new_pool(dir: PathBuf, pool: Vec<Vmid>) -> Result<(), Error> {
    if dir.exists() {
        debug!("{} exists: skiping..", dir.display().to_string())
    } else {
        fs::create_dir(dir.clone()).map_err(Error::Io)?;
    }

    let mut look = Vec::new();

    for vmid in pool {
        let mut path = dir.clone();
        path.push(vmid.vmid_number.clone().to_string());

        if path.exists() {
            debug!("{} exists: skiping..", path.display().to_string())
        } else {
            fs::create_dir(path.clone()).map_err(Error::Io)?;
            look.push(path.clone());
            path.push("config.json");

            let serialized = toml::to_string_pretty(&vmid).map_err(Error::ConfigErrorSer)?;
            File::create(path).map_err(Error::Io)?.write_all(serialized.as_bytes()).map_err(Error::Io)?;
        }
    }
    println!("{:?}", look);
    Ok(())
}

pub(crate) fn load_pool(dir: PathBuf) -> Result<Vec<Vmid>, Error> {
    info!("{}", dir.clone().display());
    let paths = fs::read_dir(dir).map_err(Error::Io)?;
    let mut pool = Vec::new();

    for path in paths {
        let mut path = path.map_err(Error::Io)?.path();
        path.push("config.json");

        info!("{}", path.clone().display());

        let vmid: Vmid = toml::from_str(
            &fs::read_to_string(path)
            .map_err(Error::Io)?)
            .map_err(Error::ConfigErrorDe)?;

        pool.push(vmid);
    }

    Ok(pool)
}

#[cfg(test)]
mod tests {
    use crate::{config::vmids::new, Error};
    use super::{load_pool, new_pool};
    use std::{path::PathBuf, fs};
    
    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn pool_test() -> Result<(), Error> {
        init();

        if PathBuf::from("./pool/test").exists() {
            fs::remove_dir_all("./pool/test").map_err(|e| Error::Io(e))?; 
        }

        new_pool(PathBuf::from("./pool/test"), new::vmid(5900,5))?;

        // fs::remove_dir_all("./pool/test").map_err(|e| Error::Io(e))?;
        Ok(())
    }

    #[test]
    fn load_pool_test() -> Result<(), Error> {
        init();

        if PathBuf::from("./pool/testload").exists() {
            fs::remove_dir_all("./pool/testload").map_err(|e| Error::Io(e))?; 
        }

        new_pool(PathBuf::from("./pool/testload"), new::vmid(5900,5))?;

        let pool = load_pool(PathBuf::from("./pool/testload"))?;

        // fs::remove_dir_all("./pool/testload").map_err(|e| Error::Io(e))?;

        assert_eq!(pool[0].vmid_number, 0);
        assert_eq!(pool[0].port, 5900);
        assert_eq!(pool[0].name, "No Name".to_string());
        Ok(())
    }
}