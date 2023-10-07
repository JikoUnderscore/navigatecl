# From bing chat
# I can try to rewrite the PowerShell script to Bash, but I cannot guarantee that it will work exactly the same. Here is my attempt:

# Assign the output of ./navigatecl.exe to a variable and print it
variable=$(./navigatecl.exe)
echo $variable

# Trim any leading or trailing whitespace from the variable
trimmed_string=$(echo $variable | xargs)

# Change directory to the trimmed string
cd $trimmed_string
