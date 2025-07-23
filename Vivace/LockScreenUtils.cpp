// LockScreenUtils.cpp
#include "LockScreenUtils.h"
#include "LockScreenGlobals.h" // For global variables
#include <Shlobj.h>             // Corrected include path for SHGetFolderPathW
#include <Lmcons.h>             // For UNLEN
#include <iostream>             // For std::wcerr, std::wcout
#include <fstream>              // For std::ifstream, std::ofstream
#include <sstream>              // For std::wstringstream

// Retrieves the current Windows username and stores it globally.
void GetWindowsUsername() {
    WCHAR username[UNLEN + 1]; // UNLEN is max username length
    DWORD username_len = UNLEN + 1;
    if (GetUserNameW(username, &username_len)) {
        g_windowsUsername = username;
    }
    else {
        g_windowsUsername = L"Guest"; // Fallback if username cannot be retrieved
        std::wcerr << L"Failed to get Windows username. Error: " << GetLastError() << std::endl;
    }
}

// Checks for the existence of a dummy password file and sets g_passwordExists.
// If the file doesn't exist, it creates a dummy one for demonstration.
void CheckOrCreatePasswordFile() {
    WCHAR appDataPath[MAX_PATH];
    if (SHGetFolderPathW(NULL, CSIDL_APPDATA, NULL, 0, appDataPath) == S_OK) {
        std::wstringstream ss;
        ss << appDataPath << L"\\VivaceLockScreen\\password.txt";
        g_passwordFilePath = ss.str();

        // Create the directory if it doesn't exist
        std::wstring dirPath = g_passwordFilePath.substr(0, g_passwordFilePath.find_last_of(L"\\/"));
        CreateDirectoryW(dirPath.c_str(), NULL); // Create directory, ignore if exists

        std::ifstream file(g_passwordFilePath);
        if (file.is_open()) {
            g_passwordExists = true;
            file.close();
            std::wcout << L"Password file found at: " << g_passwordFilePath << std::endl;
        }
        else {
            g_passwordExists = false;
            // For this iteration, we'll create a dummy file if it doesn't exist
            // In a real app, this would trigger a "set password" flow.
            std::ofstream outFile(g_passwordFilePath);
            if (outFile.is_open()) {
                outFile << "dummy_hashed_password"; // Store a dummy hash
                outFile.close();
                g_passwordExists = true; // Now it exists
                std::wcout << L"Dummy password file created at: " << g_passwordFilePath << std::endl;
            }
            else {
                std::wcerr << L"Failed to create dummy password file at: " << g_passwordFilePath << std::endl;
            }
        }
    }
    else {
        std::wcerr << L"Failed to get APPDATA path. Password management disabled." << std::endl;
        g_passwordExists = false; // Cannot manage password if path not found
    }
}

// Updates the last input tick count from the system.
void UpdateLastInputTime() {
    LASTINPUTINFO lii = { sizeof(LASTINPUTINFO) };
    if (GetLastInputInfo(&lii)) {
        g_lastInputTick = lii.dwTime;
    }
}
