use crate::i18n::Lang;

pub struct Release {
    pub version: &'static str,
    pub es: &'static [&'static str],
    pub en: &'static [&'static str],
}

/// Añade un bloque arriba al publicar (versión = CARGO_PKG_VERSION).
pub const RELEASES: &[Release] = &[
    Release {
        version: "0.2.1",
        es: &[
            "Pasa a llamarse EngineerCraft (mismo instalador: actualiza JavaCraft).",
            "Logo E y repo github.com/ProAle9710/engineercraft.",
            "Cierre automático de comillas y snippets if/for/while.",
            "SIMPLE / AVANZADO y tema claro/oscuro.",
        ],
        en: &[
            "Renamed EngineerCraft (same installer: upgrades JavaCraft).",
            "E logo and repo github.com/ProAle9710/engineercraft.",
            "Auto-close quotes and if/for/while snippets.",
            "SIMPLE / ADVANCED and light/dark theme.",
        ],
    },
    Release {
        version: "0.1.6",
        es: &[
            "Cierre automático de comillas, paréntesis y llaves.",
            "Asignar String con = \" deja = \"\"; y el punto y coma.",
            "Tab en if / for / while / switch: cursor entre paréntesis y bloque { }.",
            "SIMPLE / AVANZADO: archivo Java completo + consola real.",
            "Tema claro/oscuro y Novedades al actualizar.",
        ],
        en: &[
            "Auto-close quotes, parentheses, and braces.",
            "String assign = \" becomes = \"\"; with semicolon.",
            "Tab on if / for / while / switch: caret in parens and { } block.",
            "SIMPLE / ADVANCED: full Java file + real console.",
            "Light/dark theme and What's New on update.",
        ],
    },
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
