$TempFile = New-TemporaryFile
Invoke-WebRequest -useb "https://github.com/adam-bratin/changelog-rust/releases/latest/download/x86_64-pc-windows-msvc.zip" -OutFile $TempFile -Force
$outPath = "%USERPROFILE%\changelog-rust\bin"
If (!(test-path $outPath)) {
    New-Item -ItemType Directory -Force -Path $path
}
Expand-Archive -Path $TempFile -DestinationPath $outPath
[Environment]::SetEnvironmentVariable("Path", $env:Path + ";$outPath", "User")
