$variable = ./navigatecl.exe | Tee-Object -Variable variable

$trimmed_string = $variable.Trim()
cd $trimmed_string
