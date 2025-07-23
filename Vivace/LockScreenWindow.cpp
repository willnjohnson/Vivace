// LockScreenWindow.cpp
#include "LockScreenWindow.h"
#include "LockScreenRenderer.h" // To call RenderFrame
#include "LockScreenUtils.h"    // To call UpdateLastInputTime
#include "Vivace.h"             // Required to access resource IDs like IDM_EXIT, IDM_ABOUT, IDD_ABOUTBOX

// --- Window Procedure ---
LRESULT CALLBACK WndProc(HWND hWnd, UINT message, WPARAM wParam, LPARAM lParam) {
    switch (message)
    {
    case WM_PAINT:
    {
        PAINTSTRUCT ps;
        HDC hdc = BeginPaint(hWnd, &ps); // Get device context for painting
        // Render content based on current state (handles animation if g_isAnimating is true)
        RenderFrame(hdc, ps.rcPaint.right - ps.rcPaint.left, ps.rcPaint.bottom - ps.rcPaint.top);
        EndPaint(hWnd, &ps); // Release device context
        break;
    }

    case WM_SIZE:
        // Force a repaint when window size changes to redraw content
        InvalidateRect(hWnd, NULL, FALSE);
        break;

    case WM_LBUTTONDOWN: // Mouse click detected
        UpdateLastInputTime(); // Reset idle timer on any user interaction

        if (g_currentState == APP_STATE_LOCKED_SCREEN) {
            SetAppState(APP_STATE_UNLOCKED_SCREEN); // Unlock on explicit click
        }
        else if (g_currentState == APP_STATE_UNLOCKED_SCREEN) {
            // Check if click is on the eye icon to toggle password visibility
            int mouseX = LOWORD(lParam);
            int mouseY = HIWORD(lParam);

            // Define the approximate region for the eye icon (adjust as needed)
            // These values are hardcoded for simplicity, ideally derived from RenderFrame's calculations
            RECT clientRect;
            GetClientRect(g_hWnd, &clientRect);
            int width = clientRect.right - clientRect.left;
            int height = clientRect.bottom - clientRect.top;

            int inputWidth = 400;
            int inputHeight = 60;
            int inputX = (width - inputWidth) / 2;
            int inputY = height / 2 + 50;

            int eyeIconSize = 40; // Approximate size of the icon
            int eyeIconPadding = 15; // Padding from right edge of input box
            int eyeIconX = inputX + inputWidth - eyeIconSize - eyeIconPadding;
            int eyeIconY = inputY + (inputHeight - eyeIconSize) / 2;

            RECT eyeIconRect = { eyeIconX, eyeIconY, eyeIconX + eyeIconSize, eyeIconY + eyeIconSize };

            if (PtInRect(&eyeIconRect, { mouseX, mouseY })) {
                g_showPassword = !g_showPassword; // Toggle visibility
                InvalidateRect(hWnd, NULL, FALSE); // Redraw to reflect change
            }
        }
        break;

    case WM_KEYDOWN:     // Keyboard key press detected
        UpdateLastInputTime(); // Reset idle timer on any user interaction

        if (g_currentState == APP_STATE_LOCKED_SCREEN) {
            SetAppState(APP_STATE_UNLOCKED_SCREEN); // Unlock on explicit key press
        }
        else if (g_currentState == APP_STATE_UNLOCKED_SCREEN && wParam == VK_ESCAPE) {
            // If in unlocked state and ESC is pressed, return to locked state
            SetAppState(APP_STATE_LOCKED_SCREEN);
        }
        break;

    case WM_MOUSEMOVE:   // Mouse movement detected (only update idle, don't unlock from locked state)
        UpdateLastInputTime();
        break;

    case WM_TIMER: // Timer message
        if (wParam == 1) { // Our 1-second timer
            DWORD currentTick = GetTickCount();
            if (g_isAnimating) {
                // If animating, force redraw to update animation frame
                InvalidateRect(hWnd, NULL, FALSE);
            }
            else if (currentTick - g_lastInputTick >= IDLE_THRESHOLD_MS) {
                if (g_currentState == APP_STATE_UNLOCKED_SCREEN) {
                    SetAppState(APP_STATE_LOCKED_SCREEN); // Lock if idle in unlocked view
                }
            }
            InvalidateRect(hWnd, NULL, FALSE); // Force repaint to update time/date (even if not animating)
        }
        break;

    case WM_COMMAND: // Handle menu commands (e.g., About, Exit)
    {
        // Placeholder for About box, will need to be implemented if desired
        // For now, only Exit is handled directly.
        int wmId = LOWORD(wParam);
        switch (wmId)
        {
            // Note: DialogBox and About function are typically in Vivace.cpp or a separate dialog module.
            // If you want the About dialog to work, ensure About() is accessible and hInst is passed.
            // For now, IDM_ABOUT is commented out to avoid dependency issues if About() is not defined.
            // case IDM_ABOUT:
            //     DialogBox(hInst, MAKEINTRESOURCE(IDD_ABOUTBOX), hWnd, About);
            //     break;
        case IDM_EXIT: // Assuming IDM_EXIT is defined in Vivace.h
            DestroyWindow(hWnd);
            break;
        default:
            return DefWindowProc(hWnd, message, wParam, lParam);
        }
    }
    break;

    case WM_DESTROY:
        KillTimer(hWnd, 1); // Stop the timer
        PostQuitMessage(0); // Terminate the application
        break;

    default:
        return DefWindowProc(hWnd, message, wParam, lParam);
    }
    return 0;
}

// --- Helper Function: SetAppState ---
void SetAppState(AppState newState) {
    if (g_currentState == newState) return; // No state change, no animation

    // Start animation
    g_isAnimating = true;
    g_animationStartTime = GetTickCount();
    g_animationSourceState = g_currentState; // Current state is the source
    g_animationTargetState = newState;       // New state is the target

    if (g_animationSourceState == APP_STATE_LOCKED_SCREEN && g_animationTargetState == APP_STATE_UNLOCKED_SCREEN) {
        g_animationDirection = ANIM_LOCKED_TO_UNLOCKED;
    }
    else if (g_animationSourceState == APP_STATE_UNLOCKED_SCREEN && g_animationTargetState == APP_STATE_LOCKED_SCREEN) {
        g_animationDirection = ANIM_UNLOCKED_TO_LOCKED;
    }
    else {
        // Fallback for unexpected transitions or direct state changes without animation
        g_isAnimating = false;
        g_animationDirection = ANIM_NONE;
        g_currentState = newState; // Directly set state if no animation
    }

    // Force repaint to start the animation frames
    InvalidateRect(g_hWnd, NULL, FALSE);
}
