pub mod version_manifest {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum VersionType {
        Release,
        Snapshot,
        Beta,
        Alpha
    }
    
    impl VersionType {
        pub fn new(version_type: String) -> VersionType {
            if version_type.contains("release") {
                return VersionType::Release;
            } else if version_type.contains("snapshot") {
                return VersionType::Snapshot;
            } else if version_type.contains("beta") {
                return VersionType::Beta;
            } else {
                return VersionType::Alpha;
            }
        }
        pub fn to_string(&self) -> String {
            match self {
                VersionType::Release => "release".to_string(),
                VersionType::Snapshot => "snapshot".to_string(),
                VersionType::Beta => "beta".to_string(),
                VersionType::Alpha => "alpha".to_string()
            }
        }
    }
    
    #[derive(Clone, PartialEq, Eq)]
    pub struct Version {
        pub id: String,
        pub version_type: VersionType,
        pub url: String,
        pub release_time: String
    }
    
    impl Version {
        pub fn new(
            id: String,
            version_type: VersionType,
            url: String,
            release_time: String
        ) -> Version {
            Version {
                id,
                version_type,
                url,
                release_time
            }
        }
    }
    
    #[derive(Clone, PartialEq, Eq)]
    pub struct VersionManifest {
        pub latest_release: Version,
        pub latest_snapshot: Version,
        pub versions: Vec<Version>
    }
    
    impl VersionManifest {
        pub fn new(
            latest_release: Version,
            latest_snapshot: Version,
            versions: Vec<Version>
        ) -> VersionManifest {
            VersionManifest {
                latest_release,
                latest_snapshot,
                versions
            }
        }
    }
}

pub mod version {
    pub use super::version_manifest::VersionType;
    
    pub mod rule {
        #[derive(Clone, PartialEq, Eq)]
        pub enum RuleOS {
            Name,
            Arch,
            Version
        }
        
        impl RuleOS {
            pub fn new(os: String) -> RuleOS {
                if os == "name" {
                    return RuleOS::Name;
                } else if os == "arch" {
                    return RuleOS::Arch;
                } else {
                    return RuleOS::Version;
                }
            }
        }
        
        #[derive(Clone, PartialEq, Eq)]
        pub enum Action {
            Allow,
            Disallow
        }
        
        impl Action {
            pub fn new(action: String) -> Action {
                if action == "allow" {
                    return Action::Allow;
                } else {
                    return Action::Disallow;
                }
            }
        }
        
        #[derive(Clone, PartialEq, Eq)]
        pub struct Rule {
            pub action: Action,
            pub os: Vec<(RuleOS, String)>
        }
        
        impl Rule {
            pub fn new(
                action: Action,
                os: Vec<(RuleOS, String)>
            ) -> Rule {
                Rule {
                    action,
                    os 
                }
            }
        }
    }
    
    #[derive(Clone, PartialEq, Eq)]
    pub struct Library {
        pub path: String,
        pub sha1: String,
        pub size: u64,
        pub url: String,
        pub extract: String
    }
    
    impl Library {
        pub fn new(
            path: String,
            sha1: String,
            size: u64,
            url: String,
            extract: String
        ) -> Library {
            Library {
                path,
                sha1,
                size,
                url, 
                extract
            }
        }
    }
    
    #[derive(Clone, PartialEq, Eq)]
    pub struct Asset {
        pub hash: String,
        pub size: u64,
        pub path: String
    }
    
    impl Asset {
        pub fn new(
            hash: String,
            size: u64,
            path: String 
        ) -> Asset {
            Asset { 
                hash,
                size,
                path
            }
        }
    }
    
    #[derive(Clone, PartialEq, Eq)]
    pub struct Client {
        pub sha1: String,
        pub size: u64,
        pub url: String
    }
    
    impl Client {
        pub fn new(
            sha1: String,
            size: u64,
            url: String
        ) -> Client {
            Client {
                sha1,
                size,
                url
            }
        }
    }
    
    #[derive(Clone, PartialEq, Eq)]
    pub struct Java {
        pub name: String,
        pub version: u16
    }
    
    impl Java {
        pub fn new(
            name: String,
            version: u16
        ) -> Java {
            Java {
                name,
                version
            }
        }
    }
    
    #[derive(Clone, PartialEq, Eq)]
    pub struct Version {
        pub id: String,
        pub version_type: VersionType,
        pub release_time: String,
        pub libraries: Vec<Library>,
        pub assets: Vec<Asset>,
        pub client: Client,
        pub java: Java
    }
    
    impl Version {
        pub fn new(
            id: String,
            version_type: VersionType,
            release_time: String,
            libraries: Vec<Library>,
            assets: Vec<Asset>,
            client: Client,
            java: Java
        ) -> Version {
            Version {
                id,
                version_type,
                release_time,
                libraries,
                assets,
                client,
                java
            }
        }
    }
}