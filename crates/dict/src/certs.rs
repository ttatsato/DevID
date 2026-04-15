use yokogushi_core::Certification;

pub static CERTIFICATIONS: &[Certification] = &[
    Certification {
        id: "ipa_fe",
        name: "基本情報技術者試験",
        issuer: "IPA",
        aliases: &["FE", "基本情報"],
    },
    Certification {
        id: "ipa_ap",
        name: "応用情報技術者試験",
        issuer: "IPA",
        aliases: &["AP", "応用情報"],
    },
    Certification {
        id: "ipa_db",
        name: "データベーススペシャリスト試験",
        issuer: "IPA",
        aliases: &["DB", "DBスペシャリスト"],
    },
    Certification {
        id: "ipa_nw",
        name: "ネットワークスペシャリスト試験",
        issuer: "IPA",
        aliases: &["NW", "NWスペシャリスト"],
    },
    Certification {
        id: "ipa_sc",
        name: "情報処理安全確保支援士試験",
        issuer: "IPA",
        aliases: &["SC", "登録セキスペ"],
    },
    Certification {
        id: "aws_saa",
        name: "AWS Certified Solutions Architect – Associate",
        issuer: "AWS",
        aliases: &["SAA", "AWS SAA"],
    },
    Certification {
        id: "aws_sap",
        name: "AWS Certified Solutions Architect – Professional",
        issuer: "AWS",
        aliases: &["SAP", "AWS SAP"],
    },
    Certification {
        id: "aws_dva",
        name: "AWS Certified Developer – Associate",
        issuer: "AWS",
        aliases: &["DVA"],
    },
    Certification {
        id: "gcp_pca",
        name: "Google Cloud Professional Cloud Architect",
        issuer: "Google Cloud",
        aliases: &["PCA"],
    },
    Certification {
        id: "gcp_pde",
        name: "Google Cloud Professional Data Engineer",
        issuer: "Google Cloud",
        aliases: &["PDE"],
    },
    Certification {
        id: "lpic_1",
        name: "LPIC-1",
        issuer: "LPI",
        aliases: &["LPIC1"],
    },
    Certification {
        id: "lpic_2",
        name: "LPIC-2",
        issuer: "LPI",
        aliases: &["LPIC2"],
    },
    Certification {
        id: "cka",
        name: "Certified Kubernetes Administrator",
        issuer: "CNCF",
        aliases: &["CKA"],
    },
    Certification {
        id: "ckad",
        name: "Certified Kubernetes Application Developer",
        issuer: "CNCF",
        aliases: &["CKAD"],
    },
    Certification {
        id: "ocjp",
        name: "Oracle Certified Java Programmer",
        issuer: "Oracle",
        aliases: &["OCJP"],
    },
];
