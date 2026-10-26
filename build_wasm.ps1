Write-Host "Building OniAnvil WebAssembly (WASM) Release..." -ForegroundColor Cyan
trunk build index.html --release --dist docs/app --public-url ./
if ($LASTEXITCODE -eq 0) {
    Write-Host "OniAnvil WebAssembly build successful in docs/app/" -ForegroundColor Green
} else {
    Write-Host "Trunk build failed with exit code $LASTEXITCODE" -ForegroundColor Red
}
