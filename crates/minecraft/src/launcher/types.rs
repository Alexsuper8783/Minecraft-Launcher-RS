pub mod version_manifest {
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
    }
    
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
    pub mod rule {
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
}