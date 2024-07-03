cargo build --release

Copy-Item -Recurse -Path "std" -Destination "target/release/std"

Clear-Host

$path = [System.IO.Path]::Combine((Get-Location).Path, "protocolyt/target/release")
$env:PATH += ";$path"

Write-Output "O diretório atual foi adicionado ao PATH:"
Write-Output $env:PATH
