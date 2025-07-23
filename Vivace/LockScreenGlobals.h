// LockScreenGlobals.h
#pragma once

// --- Standard Windows Headers and Preprocessor Definitions ---
// These preprocessor definitions MUST come before any #include <windows.h>
#define WIN32_LEAN_AND_MEAN
#define NOMINMAX
#include <windows.h>

// --- Standard C++ Library Includes for Types ---
#include <string>

// --- Global Variables (declared extern here, defined in Vivace.cpp) ---
extern HWND g_hWnd;

// Application States for the lock screen
enum AppState {
    APP_STATE_LOCKED_SCREEN,
    APP_STATE_UNLOCKED_SCREEN
};
extern AppState g_currentState;

extern DWORD g_lastInputTick;
extern const DWORD IDLE_THRESHOLD_MS;

// --- User and Password Management Global Variables ---
extern std::wstring g_windowsUsername;
extern std::wstring g_passwordFilePath;
extern bool g_passwordExists;
extern bool g_showPassword; // Flag to toggle password visibility (for placeholder)

// --- Animation Global Variables ---
enum AnimationDirection {
    ANIM_NONE,
    ANIM_LOCKED_TO_UNLOCKED, // Locked screen moves up and fades out, Unlocked moves up and fades in
    ANIM_UNLOCKED_TO_LOCKED  // Unlocked screen moves down and fades out, Locked moves down and fades in
};

extern bool g_isAnimating;
extern DWORD g_animationStartTime;
extern const DWORD g_animationDurationMs;
extern AppState g_animationTargetState;
extern AppState g_animationSourceState;
extern AnimationDirection g_animationDirection;
