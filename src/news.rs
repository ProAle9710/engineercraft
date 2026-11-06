use crate::i18n::Lang;

pub struct Release {
    pub version: &'static str,
    pub es: &'static [&'static str],
    pub en: &'static [&'static str],
}

/// Añade un bloque arriba al publicar (versión = CARGO_PKG_VERSION).
pub const RELEASES: &[Release] = &[
    Release {
        version: "0.1.5",
        es: &[
            "SIMPLE / AVANZADO: archivo Java completo + consola real.",
            "Tema claro/oscuro (icono sol/luna, esquina derecha).",
            "Pantalla Novedades al actualizar (clic en la versión).",
            "Autocomplete con explicación del método o tipo (Ctrl+Espacio).",
            "Práctica libre y copiar comentarios en el editor.",
        ],
        en: &[
            "SIMPLE / ADVANCED: full Java file + real console.",
            "Light/dark theme (sun/moon, top-right).",
            "What's New on update (click the version).",
            "Autocomplete docs for methods and types (Ctrl+Space).",
            "Free practice and copyable comments.",
        ],
    },
    Release {
        version: "0.1.4",
        es: &[
            "SIMPLE / AVANZADO: archivo Java completo + consola real.",
            "Tema claro/oscuro (icono sol/luna, esquina derecha).",
            "Autocomplete con explicación del método o tipo (Ctrl+Espacio).",
            "Práctica libre y copiar comentarios en el editor.",
            "Actualización desde GitHub Releases.",
        ],
        en: &[
            "SIMPLE / ADVANCED: full Java file + real console.",
            "Light/dark theme (sun/moon, top-right).",
            "Autocomplete docs for methods and types (Ctrl+Space).",
            "Free practice and copyable comments.",
            "Updates from GitHub Releases.",
        ],
    },
];

pub fn for_version(v: &str) -> &'static Release {
    RELEASES
        .iter()
        .find(|r| r.version == v)
        .unwrap_or(&RELEASES[0])
}

pub fn items(rel: &Release, lang: Lang) -> &'static [&'static str] {
    match lang {
        Lang::Es => rel.es,
        Lang::En => rel.en,
    }
}
