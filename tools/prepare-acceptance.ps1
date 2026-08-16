param(
  [Parameter(Mandatory = $true)]
  [string] $SubmitterName,

  [Parameter(Mandatory = $true)]
  [string] $SubmitterEmail,

  [Parameter(Mandatory = $true)]
  [string] $Model,

  [string] $Title = "Accept secp256k1 point-add submission",
  [string] $Note = "accepted submission"
)

$ErrorActionPreference = "Stop"

function Invoke-NativeChecked {
  param(
    [Parameter(Mandatory = $true)]
    [string] $FilePath,

    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]] $Arguments
  )

  & $FilePath @Arguments
  if ($LASTEXITCODE -ne 0) {
    throw "$FilePath failed with exit code $LASTEXITCODE"
  }
}

Push-Location (Split-Path -Parent $PSScriptRoot)
try {
  Invoke-NativeChecked powershell -NoProfile -ExecutionPolicy Bypass -File .\setup.ps1
  Invoke-NativeChecked powershell -NoProfile -ExecutionPolicy Bypass -File .\benchmark.ps1 -Note $Note
  Invoke-NativeChecked powershell -ExecutionPolicy Bypass -File tools\package-submission.ps1 -NoteFile src\point_add\memory\README.md -Model $Model

  $score = Get-Content score.json | ConvertFrom-Json
  $metadata = Get-Content dist\submission-metadata.json | ConvertFrom-Json

  $message = @"
$Title

Score: $($score.score)
Score model: $($metadata.scoreModel)
Toffoli: $($score.metrics.toffoli)
Qubits: $($score.metrics.qubits)
Artifact: $($metadata.artifact)
Validation: 9024 native ECDSA Fail Fiat-Shamir point-add shots
Model: $Model

Co-authored-by: $SubmitterName <$SubmitterEmail>
"@

  Set-Content -Path ACCEPTANCE_COMMIT_MESSAGE.txt -Value $message -Encoding utf8
  Write-Host "Wrote ACCEPTANCE_COMMIT_MESSAGE.txt"
  Write-Host ""
  Write-Host $message
}
finally {
  Pop-Location
}
