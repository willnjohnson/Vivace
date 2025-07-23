#pragma once
// LockScreenUtils.h
#pragma once

#include <windows.h> // For DWORD
#include <string>    // For std::wstring

// Function to update the last input tick count
void UpdateLastInputTime();

// Function to retrieve the Windows username
void GetWindowsUsername();

// Function to check for and potentially create a dummy password file
void CheckOrCreatePasswordFile();
