use collections::BTreeMap;
pub fn gen() -> BTreeMap<&'static [u8], (&'static [u8], bool)> {
    let mut files: BTreeMap<&'static [u8], (&'static [u8], bool)> = BTreeMap::new();
    files.insert(b"", (b"bin\netc", true));
    files.insert(b"bin", (b"ahcid\ninit\npcid\nps2d\nredoxfs\nvesad", true));
    files.insert(b"etc", (b"init.rc\npcid.toml", true));
    files.insert(b"bin/ahcid", (include_bytes!("../../initfs/bin/ahcid"), false));
    files.insert(b"bin/init", (include_bytes!("../../initfs/bin/init"), false));
    files.insert(b"bin/pcid", (include_bytes!("../../initfs/bin/pcid"), false));
    files.insert(b"bin/ps2d", (include_bytes!("../../initfs/bin/ps2d"), false));
    files.insert(b"bin/redoxfs", (include_bytes!("../../initfs/bin/redoxfs"), false));
    files.insert(b"bin/vesad", (include_bytes!("../../initfs/bin/vesad"), false));
    files.insert(b"etc/init.rc", (include_bytes!("../../initfs/etc/init.rc"), false));
    files.insert(b"etc/pcid.toml", (include_bytes!("../../initfs/etc/pcid.toml"), false));
    files
}
