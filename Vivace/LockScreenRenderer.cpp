// LockScreenRenderer.cpp
#include "LockScreenRenderer.h"
#include "LockScreenGlobals.h" // For global state variables
#include <chrono>      // For time handling (std::chrono::system_clock)
#include <iomanip>     // For std::put_time
#include <sstream>     // For std::wstringstream
#include <string>      // For std::wstring

// Required for AlphaBlend function
#pragma comment(lib, "Msimg32.lib")

// Forward declarations for internal drawing functions
void DrawLockedScreenContent(HDC hdc, int width, int height, const std::tm& current_tm);
void DrawUnlockedScreenContent(HDC hdc, int width, int height);

// Renders the current frame based on the application state using basic GDI.
void RenderFrame(HDC hdc, int width, int height) {
    // Create a main memory DC for double buffering the final composite
    HDC hdcMem = CreateCompatibleDC(hdc);
    HBITMAP hbmScreen = CreateCompatibleBitmap(hdc, width, height);
    HBITMAP hbmOld = (HBITMAP)SelectObject(hdcMem, hbmScreen);

    // Set text drawing mode for transparency (for DrawText)
    SetBkMode(hdcMem, TRANSPARENT);

    // Get current time for display
    auto now = std::chrono::system_clock::now();
    std::time_t now_c = std::chrono::system_clock::to_time_t(now);
    std::tm current_tm;
    localtime_s(&current_tm, &now_c);

    if (g_isAnimating) {
        DWORD currentTime = GetTickCount();
        DWORD elapsedTime = currentTime - g_animationStartTime;
        float progress = static_cast<float>(elapsedTime) / g_animationDurationMs;

        if (progress >= 1.0f) {
            // Animation finished, set final state and stop animating
            g_isAnimating = false;
            g_currentState = g_animationTargetState;
            g_animationDirection = ANIM_NONE;
            progress = 1.0f; // Cap progress at 1.0
            InvalidateRect(g_hWnd, NULL, FALSE); // Redraw one last time in final state
        } else {
            InvalidateRect(g_hWnd, NULL, FALSE); // Keep redrawing during animation
        }

        // Create two temporary memory DCs for source and destination screens
        HDC hdcSourceScreen = CreateCompatibleDC(hdc);
        HBITMAP hbmSourceScreen = CreateCompatibleBitmap(hdc, width, height);
        HBITMAP hbmOldSourceScreen = (HBITMAP)SelectObject(hdcSourceScreen, hbmSourceScreen);

        HDC hdcTargetScreen = CreateCompatibleDC(hdc);
        HBITMAP hbmTargetScreen = CreateCompatibleBitmap(hdc, width, height);
        HBITMAP hbmOldTargetScreen = (HBITMAP)SelectObject(hdcTargetScreen, hbmTargetScreen);

        // Draw content for both source and target states onto their respective memory DCs
        if (g_animationSourceState == APP_STATE_LOCKED_SCREEN) {
            DrawLockedScreenContent(hdcSourceScreen, width, height, current_tm);
        } else { // g_animationSourceState == APP_STATE_UNLOCKED_SCREEN
            DrawUnlockedScreenContent(hdcSourceScreen, width, height);
        }

        if (g_animationTargetState == APP_STATE_LOCKED_SCREEN) {
            DrawLockedScreenContent(hdcTargetScreen, width, height, current_tm);
        } else { // g_animationTargetState == APP_STATE_UNLOCKED_SCREEN
            DrawUnlockedScreenContent(hdcTargetScreen, width, height);
        }

        // Calculate blend parameters
        BLENDFUNCTION bf;
        bf.BlendOp = AC_SRC_OVER;
        bf.BlendFlags = 0;
        bf.SourceConstantAlpha = 255; // Full alpha by default, will be adjusted
        bf.AlphaFormat = 0; // No pre-multiplied alpha

        int yOffsetSource = 0;
        int yOffsetTarget = 0;
        BYTE alphaSource = 255;
        BYTE alphaTarget = 255;

        // Determine offsets and alphas based on animation direction and progress
        if (g_animationDirection == ANIM_LOCKED_TO_UNLOCKED) {
            // Locked screen moves up and fades out (alpha goes 255 -> 0)
            // Unlocked screen moves up and fades in (alpha goes 0 -> 255)
            yOffsetSource = static_cast<int>(-height * progress);
            yOffsetTarget = static_cast<int>(height * (1.0f - progress)); // Starts below, moves to 0

            alphaSource = static_cast<BYTE>(255 * (1.0f - progress));
            alphaTarget = static_cast<BYTE>(255 * progress);

        } else if (g_animationDirection == ANIM_UNLOCKED_TO_LOCKED) {
            // Unlocked screen moves down and fades out (alpha goes 255 -> 0)
            // Locked screen moves down and fades in (alpha goes 0 -> 255)
            yOffsetSource = static_cast<int>(height * progress);
            yOffsetTarget = static_cast<int>(-height * (1.0f - progress)); // Starts above, moves to 0

            alphaSource = static_cast<BYTE>(255 * (1.0f - progress));
            alphaTarget = static_cast<BYTE>(255 * progress);
        }

        // Clear the main memory DC first
        HBRUSH hBrushBg = CreateSolidBrush(RGB(20, 20, 20)); // Base background color
        RECT fullRect = {0, 0, width, height};
        FillRect(hdcMem, &fullRect, hBrushBg);
        DeleteObject(hBrushBg);

        // Draw the source screen (fading out)
        bf.SourceConstantAlpha = alphaSource;
        AlphaBlend(hdcMem, 0, yOffsetSource, width, height, hdcSourceScreen, 0, 0, width, height, bf);

        // Draw the target screen (fading in)
        bf.SourceConstantAlpha = alphaTarget;
        AlphaBlend(hdcMem, 0, yOffsetTarget, width, height, hdcTargetScreen, 0, 0, width, height, bf);


        // Clean up temporary DCs and bitmaps
        SelectObject(hdcSourceScreen, hbmOldSourceScreen);
        DeleteObject(hbmSourceScreen);
        DeleteDC(hdcSourceScreen);

        SelectObject(hdcTargetScreen, hbmOldTargetScreen);
        DeleteObject(hbmTargetScreen);
        DeleteDC(hdcTargetScreen);

    } else {
        // Not animating, just draw the current state
        if (g_currentState == APP_STATE_LOCKED_SCREEN) {
            DrawLockedScreenContent(hdcMem, width, height, current_tm);
        } else { // APP_STATE_UNLOCKED_SCREEN
            DrawUnlockedScreenContent(hdcMem, width, height);
        }
    }

    // Copy the contents of the main memory DC to the screen DC
    BitBlt(hdc, 0, 0, width, height, hdcMem, 0, 0, SRCCOPY);

    // Clean up main memory DC and bitmap
    SelectObject(hdcMem, hbmOld);
    DeleteObject(hbmScreen);
    DeleteDC(hdcMem);
}

// --- Internal Drawing Functions ---

void DrawLockedScreenContent(HDC hdc, int width, int height, const std::tm& current_tm) {
    // Create fonts and brushes (these should ideally be cached for performance)
    HFONT hFontLarge = CreateFont(100, 0, 0, 0, FW_NORMAL, FALSE, FALSE, FALSE,
                                  DEFAULT_CHARSET, OUT_DEFAULT_PRECIS, CLIP_DEFAULT_PRECIS,
                                  CLEARTYPE_QUALITY, DEFAULT_PITCH | FF_SWISS, L"Segoe UI");
    HFONT hFontMedium = CreateFont(50, 0, 0, 0, FW_NORMAL, FALSE, FALSE, FALSE,
                                   DEFAULT_CHARSET, OUT_DEFAULT_PRECIS, CLIP_DEFAULT_PRECIS,
                                   CLEARTYPE_QUALITY, DEFAULT_PITCH | FF_SWISS, L"Segoe UI");
    HFONT hFontSmall = CreateFont(30, 0, 0, 0, FW_NORMAL, FALSE, FALSE, FALSE,
                                  DEFAULT_CHARSET, OUT_DEFAULT_PRECIS, CLIP_DEFAULT_PRECIS,
                                  CLEARTYPE_QUALITY, DEFAULT_PITCH | FF_SWISS, L"Segoe UI");

    COLORREF textColorWhite = RGB(255, 255, 255);
    COLORREF textColorGray = RGB(180, 180, 180);
    COLORREF textColorDarkGray = RGB(50, 50, 50);
    COLORREF bgColorDark = RGB(20, 20, 20);

    // Clear background
    HBRUSH hBrushBg = CreateSolidBrush(bgColorDark);
    RECT fullRect = {0, 0, width, height};
    FillRect(hdc, &fullRect, hBrushBg);
    DeleteObject(hBrushBg);

    // --- Display Time (HH:MM:SS) ---
    std::wstringstream ssTime;
    ssTime << std::put_time(&current_tm, L"%H:%M:%S");
    std::wstring timeStr = ssTime.str();

    SelectObject(hdc, hFontLarge);
    SetTextColor(hdc, textColorWhite);
    RECT timeRect = {0, height / 2 - 100, width, height / 2 + 20}; // Adjust height for font size
    DrawText(hdc, timeStr.c_str(), -1, &timeRect, DT_CENTER | DT_VCENTER | DT_SINGLELINE);

    // --- Display Date (WEEK DAY MONTH DAY_NUMBER, YEAR) ---
    std::wstringstream ssDate;
    ssDate << std::put_time(&current_tm, L"%A, %B %d, %Y");
    std::wstring dateStr = ssDate.str();

    SelectObject(hdc, hFontMedium);
    SetTextColor(hdc, textColorGray);
    RECT dateRect = {0, height / 2 + 50, width, height / 2 + 110}; // Adjust height
    DrawText(hdc, dateStr.c_str(), -1, &dateRect, DT_CENTER | DT_VCENTER | DT_SINGLELINE);

    // --- Instruction text (positioned directly below date) ---
    SelectObject(hdc, hFontSmall);
    SetTextColor(hdc, textColorDarkGray);
    RECT instructionRect = {0, height / 2 + 120, width, height / 2 + 160}; // Position below dateRect
    DrawText(hdc, L"Click anywhere or press a key to unlock", -1, &instructionRect, DT_CENTER | DT_VCENTER | DT_SINGLELINE);

    // Clean up fonts
    DeleteObject(hFontLarge);
    DeleteObject(hFontMedium);
    DeleteObject(hFontSmall);
}

void DrawUnlockedScreenContent(HDC hdc, int width, int height) {
    // Create fonts and brushes (these should ideally be cached for performance)
    HFONT hFontLarge = CreateFont(100, 0, 0, 0, FW_NORMAL, FALSE, FALSE, FALSE,
                                  DEFAULT_CHARSET, OUT_DEFAULT_PRECIS, CLIP_DEFAULT_PRECIS,
                                  CLEARTYPE_QUALITY, DEFAULT_PITCH | FF_SWISS, L"Segoe UI");
    HFONT hFontMedium = CreateFont(50, 0, 0, 0, FW_NORMAL, FALSE, FALSE, FALSE,
                                   DEFAULT_CHARSET, OUT_DEFAULT_PRECIS, CLIP_DEFAULT_PRECIS,
                                   CLEARTYPE_QUALITY, DEFAULT_PITCH | FF_SWISS, L"Segoe UI");
    HFONT hFontSmall = CreateFont(30, 0, 0, 0, FW_NORMAL, FALSE, FALSE, FALSE,
                                  DEFAULT_CHARSET, OUT_DEFAULT_PRECIS, CLIP_DEFAULT_PRECIS,
                                  CLEARTYPE_QUALITY, DEFAULT_PITCH | FF_SWISS, L"Segoe UI");
    HFONT hFontIcon = CreateFont(40, 0, 0, 0, FW_NORMAL, FALSE, FALSE, FALSE,
                                 DEFAULT_CHARSET, OUT_DEFAULT_PRECIS, CLIP_DEFAULT_PRECIS,
                                 CLEARTYPE_QUALITY, DEFAULT_PITCH | FF_SWISS, L"Segoe UI Symbol"); // For Unicode icons

    COLORREF textColorWhite = RGB(255, 255, 255);
    COLORREF textColorGray = RGB(180, 180, 180);
    COLORREF textColorDarkGray = RGB(50, 50, 50);
    COLORREF bgColorLightDark = RGB(40, 40, 40);
    COLORREF accentColorBlue = RGB(0, 120, 215);

    // Clear background
    HBRUSH hBrushBg = CreateSolidBrush(bgColorLightDark);
    RECT fullRect = {0, 0, width, height};
    FillRect(hdc, &fullRect, hBrushBg);
    DeleteObject(hBrushBg);

    // --- AVATAR Placeholder ---
    int avatarSize = 150;
    int avatarX = (width - avatarSize) / 2;
    int avatarY = height / 2 - avatarSize - 100;
    HBRUSH hBrushBlue = CreateSolidBrush(accentColorBlue);
    RECT avatarRect = {avatarX, avatarY, avatarX + avatarSize, avatarY + avatarSize};
    FillRect(hdc, &avatarRect, hBrushBlue);
    DeleteObject(hBrushBlue);

    // Draw a Unicode character "👤" (person icon) inside the rectangle
    SelectObject(hdc, hFontLarge); // Use large font for the icon
    SetTextColor(hdc, textColorWhite);
    DrawText(hdc, L"👤", -1, &avatarRect, DT_CENTER | DT_VCENTER | DT_SINGLELINE);


    // --- NAME Display (moved up) ---
    SelectObject(hdc, hFontMedium);
    SetTextColor(hdc, textColorWhite);
    RECT nameRect = {0, height / 2 - 70, width, height / 2 - 10}; // Adjusted Y position
    DrawText(hdc, g_windowsUsername.c_str(), -1, &nameRect, DT_CENTER | DT_VCENTER | DT_SINGLELINE);

    // --- INPUT BOX WITH PASSWORD Placeholder ---
    int inputWidth = 400;
    int inputHeight = 60;
    int inputX = (width - inputWidth) / 2;
    int inputY = height / 2 + 50;

    // Draw the input box background
    HBRUSH hBrushDarkGray = CreateSolidBrush(textColorDarkGray);
    RECT inputRect = {inputX, inputY, inputX + inputWidth, inputY + inputHeight};
    FillRect(hdc, &inputRect, hBrushDarkGray);
    DeleteObject(hBrushDarkGray);

    // Draw masked/unmasked password text (placeholder)
    SelectObject(hdc, hFontMedium);
    SetTextColor(hdc, textColorWhite);
    std::wstring passwordDisplay = g_showPassword ? L"Password" : L"********";
    DrawText(hdc, passwordDisplay.c_str(), -1, &inputRect, DT_CENTER | DT_VCENTER | DT_SINGLELINE);

    // --- Eye Icon for Password Toggle ---
    int eyeIconSize = 40; // Approximate size of the icon
    int eyeIconPadding = 15; // Padding from right edge of input box
    int eyeIconX = inputX + inputWidth - eyeIconSize - eyeIconPadding;
    int eyeIconY = inputY + (inputHeight - eyeIconSize) / 2;
    RECT eyeIconRect = {eyeIconX, eyeIconY, eyeIconX + eyeIconSize, eyeIconY + eyeIconSize};

    SelectObject(hdc, hFontIcon); // Use the icon font
    SetTextColor(hdc, textColorWhite);
    DrawText(hdc, L"👁️", -1, &eyeIconRect, DT_CENTER | DT_VCENTER | DT_SINGLELINE);


    // Instruction text for returning to lock screen
    SelectObject(hdc, hFontSmall);
    SetTextColor(hdc, textColorGray);
    RECT instructionRect = {0, height - 100, width, height - 60};
    DrawText(hdc, L"Idle for 5 minutes to lock, or ESC to return", -1, &instructionRect, DT_CENTER | DT_VCENTER | DT_SINGLELINE);

    // Clean up fonts
    DeleteObject(hFontLarge);
    DeleteObject(hFontMedium);
    DeleteObject(hFontSmall);
    DeleteObject(hFontIcon); // Clean up the new icon font
}
