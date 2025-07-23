// Vivace.cpp : Defines the entry point for the application.
// This file now serves as the main entry point for the modularized lock screen application.

// --- Standard Windows Headers and Preprocessor Definitions ---
// These preprocessor definitions MUST come before any #include <windows.h>
#define WIN32_LEAN_AND_MEAN             // Exclude rarely-used stuff from Windows headers
#define NOMINMAX                        // Exclude min/max macros to prevent conflicts with std::min/max
#include <windows.h>                    // Required for HWND, GetTickCount, GetLastInputInfo, etc.

// Required for AlphaBlend function (though its usage is in LockScreenRenderer.cpp,
// this pragma is often placed in the main app for convenience or if other parts might use it)
#pragma comment(lib, "Msimg32.lib")

#include "framework.h"          // Minimal framework.h (should only contain targetver.h)
#include "Vivace.h"             // Your project's resource header (if it defines IDs like IDI_VIVACE)
#include "LockScreenGlobals.h"  // Global variables
#include "LockScreenWindow.h"   // Window procedure and state management (declares WndProc)
#include "LockScreenUtils.h"    // Utility functions

// --- Global Variables (defined here, declared extern in LockScreenGlobals.h) ---
// These variables are DEFINED here and declared 'extern' in LockScreenGlobals.h
#define MAX_LOADSTRING 100 // Define MAX_LOADSTRING here, at the top, before its use.

HINSTANCE hInst;                                // Current instance handle
WCHAR szTitle[MAX_LOADSTRING];                  // The title bar text (from resources)
WCHAR szWindowClass[MAX_LOADSTRING];            // The main window class name (from resources)
HWND g_hWnd = nullptr;                          // Main window handle

AppState g_currentState = APP_STATE_LOCKED_SCREEN; // Initial state: Locked

DWORD g_lastInputTick = GetTickCount();         // Time of last user input
const DWORD IDLE_THRESHOLD_MS = 5 * 60 * 1000;  // 5 minutes in milliseconds for idle detection

std::wstring g_windowsUsername; // Stores the current Windows username
std::wstring g_passwordFilePath; // Full path to the dummy password file
bool g_passwordExists = false;   // Flag indicating if a password file exists
bool g_showPassword = false;     // Flag to toggle password visibility (for placeholder)

// --- Animation Global Variables Definitions ---
bool g_isAnimating = false;
DWORD g_animationStartTime = 0;
const DWORD g_animationDurationMs = 500; // 0.5 seconds for animation
AppState g_animationTargetState;
AppState g_animationSourceState;
AnimationDirection g_animationDirection = ANIM_NONE;


// --- Forward declarations of functions implemented in THIS module ---
ATOM                MyRegisterClass(HINSTANCE hInstance);
BOOL                InitInstance(HINSTANCE, int);
INT_PTR CALLBACK    About(HWND, UINT, WPARAM, LPARAM);
void MainMessageLoop();


// --- Main Entry Point ---
int APIENTRY wWinMain(_In_ HINSTANCE hInstance,
    _In_opt_ HINSTANCE hPrevInstance,
    _In_ LPWSTR    lpCmdLine,
    _In_ int       nCmdShow)
{
    UNREFERENCED_PARAMETER(hPrevInstance);
    UNREFERENCED_PARAMETER(lpCmdLine);

    // Initialize global strings from resources
    LoadStringW(hInstance, IDS_APP_TITLE, szTitle, MAX_LOADSTRING);
    LoadStringW(hInstance, IDC_VIVACE, szWindowClass, MAX_LOADSTRING);

    // Register the window class
    MyRegisterClass(hInstance);

    // Perform application initialization: create and show the window
    if (!InitInstance(hInstance, nCmdShow))
    {
        return FALSE;
    }

    // Get Windows username
    GetWindowsUsername();

    // Check for existing password file or prompt for setup
    CheckOrCreatePasswordFile();

    // Start the main message loop
    MainMessageLoop();

    return 0; // Return 0 on successful exit
}

// --- FUNCTION: MyRegisterClass() ---
// PURPOSE: Registers the window class.
ATOM MyRegisterClass(HINSTANCE hInstance)
{
    WNDCLASSEXW wcex;

    wcex.cbSize = sizeof(WNDCLASSEX);
    wcex.style = CS_HREDRAW | CS_VREDRAW; // Redraw on size changes
    wcex.lpfnWndProc = WndProc;                 // Our window procedure (defined in LockScreenWindow.cpp)
    wcex.cbClsExtra = 0;
    wcex.cbWndExtra = 0;
    wcex.hInstance = hInstance;
    wcex.hIcon = LoadIcon(hInstance, MAKEINTRESOURCE(IDI_VIVACE)); // Load application icon
    wcex.hCursor = LoadCursor(nullptr, IDC_ARROW);                   // Standard arrow cursor
    wcex.hbrBackground = (HBRUSH)(COLOR_WINDOW + 1);                         // Default background brush (will be overwritten by drawing)
    wcex.lpszMenuName = nullptr;                  // CRITICAL: Set to nullptr to ensure no menu is loaded
    wcex.lpszClassName = szWindowClass;                                    // Window class name
    wcex.hIconSm = LoadIcon(wcex.hInstance, MAKEINTRESOURCE(IDI_SMALL)); // Small icon

    return RegisterClassExW(&wcex);
}

// --- FUNCTION: InitInstance(HINSTANCE, int) ---
// PURPOSE: Saves instance handle and creates main window
BOOL InitInstance(HINSTANCE hInstance, int nCmdShow)
{
    hInst = hInstance; // Store instance handle in our global variable

    // Get screen dimensions for fullscreen window
    int screenWidth = GetSystemMetrics(SM_CXSCREEN);
    int screenHeight = GetSystemMetrics(SM_CYSCREEN);

    // Create the fullscreen, borderless, always-on-top window
    g_hWnd = CreateWindowEx(
        WS_EX_TOPMOST, // Extended style: Always on top
        szWindowClass,
        szTitle, // Window title (can be empty for borderless)
        WS_POPUP | WS_VISIBLE, // Window style: Borderless popup, initially visible
        0, 0, // Position at top-left corner of the screen
        screenWidth, screenHeight, // Fullscreen dimensions
        nullptr,       // Parent window (none)
        nullptr,       // Menu (none) - This is also crucial for no menu
        hInstance,
        nullptr        // Additional application data (none)
    );

    if (!g_hWnd)
    {
        MessageBox(NULL, L"Window Creation Failed!", L"Error", MB_OK | MB_ICONERROR);
        return FALSE;
    }

    ShowWindow(g_hWnd, SW_SHOWMAXIMIZED); // Show the window maximized to fill the screen
    UpdateWindow(g_hWnd); // Ensure the window is painted

    // Set a timer to periodically update the display (e.g., clock)
    SetTimer(g_hWnd, 1, 1000, NULL); // 1-second timer (ID 1)

    return TRUE;
}

// --- Message handler for about box (kept for boilerplate compatibility) ---
INT_PTR CALLBACK About(HWND hDlg, UINT message, WPARAM wParam, LPARAM lParam)
{
    UNREFERENCED_PARAMETER(lParam);
    switch (message)
    {
    case WM_INITDIALOG:
        return (INT_PTR)TRUE;

    case WM_COMMAND:
        if (LOWORD(wParam) == IDOK || LOWORD(wParam) == IDCANCEL)
        {
            EndDialog(hDlg, LOWORD(wParam));
            return (INT_PTR)TRUE;
        }
        break;
    }
    return (INT_PTR)FALSE;
}

// Main message loop for the application.
void MainMessageLoop() {
    MSG msg = {};
    // Retrieve and dispatch messages until a WM_QUIT message is received.
    while (GetMessage(&msg, nullptr, 0, 0))
    {
        // Translate virtual-key messages into character messages.
        // This is important for keyboard input processing.
        TranslateMessage(&msg);
        // Dispatch the message to the window procedure (WndProc).
        DispatchMessage(&msg);
    }
}
