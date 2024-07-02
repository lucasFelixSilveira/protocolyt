git clone https://github.com/lucasFelixSilveira/protocolyt.git

Set-Location -Path "./protocolyt"

cargo build --release

Set-Location -Path ".."

$env:PATH += ";$(Get-Location)/protocolyt/target/release"

Write-Output "O diretório do projeto foi adicionado ao PATH:"
Write-Output $env:PATH
