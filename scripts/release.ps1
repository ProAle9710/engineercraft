# Publica GitHub Release: bump patch, MSI, gh release create + asset.
# Requiere: cargo, WiX 7, gh autenticado (gh auth login)
& "$PSScriptRoot\build-msi.ps1" -Publish
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
