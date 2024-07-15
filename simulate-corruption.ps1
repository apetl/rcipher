# Function to corrupt a file by writing random data
function Corrupt-File {
    param (
        [int]$BytesToCorrupt = 10
    )

    $FilePath = "X:\Projects\rcipher\corrupt.rcp"

    # Check if the file exists
    if (-Not (Test-Path $FilePath)) {
        Write-Host "File not found: $FilePath"
        return
    }

    # Open the file in binary mode
    $fileStream = [System.IO.File]::Open($FilePath, [System.IO.FileMode]::Open, [System.IO.FileAccess]::ReadWrite)
    $random = New-Object System.Random

    try {
        for ($i = 0; $i -lt $BytesToCorrupt; $i++) {
            # Generate a random position within the file
            $position = $random.Next(0, $fileStream.Length)
            $fileStream.Seek($position, [System.IO.SeekOrigin]::Begin) | Out-Null

            # Generate a random byte
            $randomByte = [byte]($random.Next(0, 256))

            # Write the random byte to the file
            $fileStream.WriteByte($randomByte)
        }

        Write-Host "File corrupted successfully: $FilePath"
    }
    finally {
        # Close the file stream
        $fileStream.Close()
    }
}

# Example usage
Corrupt-File -BytesToCorrupt 5

