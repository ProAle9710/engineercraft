# Publica GitHub Release: bump patch, MSI, gh release create + asset.
# Antes: añade un Release { version } al tope de src/news.rs (What's new).
# Requiere: cargo, WiX 7, gh autenticado (gh auth login)
& "$PSScriptRoot\build-msi.ps1" -Publish
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
