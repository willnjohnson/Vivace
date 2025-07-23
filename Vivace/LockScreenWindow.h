// LockScreenWindow.h
#pragma once

#include <windows.h> // For HWND, UINT, WPARAM, LPARAM
#include "LockScreenGlobals.h" // For AppState enum and global variables

// Function to handle window messages
LRESULT CALLBACK WndProc(HWND hWnd, UINT message, WPARAM wParam, LPARAM lParam);

// Function to set the application state
void SetAppState(AppState newState);
