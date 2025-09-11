import React, { useState, useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';
import './CalendarScreen.css';

interface CalendarDate {
  system: string;
  date: string;
  additional_info?: string;
}

// Re-declare UserSettings on the frontend if you need to load individual settings
// Or create a minimal interface for just the relevant settings
interface UserSettings {
  show_seconds: boolean | null;
  date_format: string | null;
  // Add other settings if needed for frontend logic
}

interface CalendarScreenProps {
  onProceed: () => void;
}

const CalendarScreen: React.FC<CalendarScreenProps> = ({ onProceed }) => {
  const [currentTime, setCurrentTime] = useState(new Date());
  const [calendarDates, setCalendarDates] = useState<CalendarDate[]>([]);
  const [userSettings, setUserSettings] = useState<UserSettings | null>(null); // State to hold user settings
  const [isLoading, setIsLoading] = useState(true);

  // Function to load settings
  const loadUserSettings = useCallback(async () => {
    try {
      // Assuming 'load_settings' command returns the full UserSettings struct
      const settings = await invoke<UserSettings>('load_settings');
      console.log('Loaded user settings:', settings); // Debug log
      setUserSettings(settings);
    } catch (error) {
      console.error('Failed to load user settings:', error);
      // Fallback or handle error
      setUserSettings({ show_seconds: true, date_format: 'military' }); // Default fallback
    }
  }, []);

  // Function to load other calendar dates (non-Gregorian primarily, or just for verification)
  const loadOtherCalendarDates = useCallback(async () => {
    try {
      const dates = await invoke<CalendarDate[]>('get_current_dates');
      console.log('Received ALL calendar dates from Rust (for other calendars):', dates); // Debug log
      setCalendarDates(dates);
    } catch (error) {
      console.error('Failed to load ALL calendar dates:', error);
    } finally {
      setIsLoading(false);
    }
  }, []);


  useEffect(() => {
    // Load user settings on component mount
    loadUserSettings();

    // Update client-side time every second
    const timeInterval = setInterval(() => {
      setCurrentTime(new Date());
    }, 1000);

    // Load ALL calendar dates initially (for non-Gregorian displays)
    loadOtherCalendarDates();

    // Re-load other calendar dates every minute (as they change less frequently)
    const calendarInterval = setInterval(() => {
      loadOtherCalendarDates();
    }, 60000);

    // Add event listeners for any key press or click
    const handleInteraction = () => {
      onProceed();
    };

    document.addEventListener('keydown', handleInteraction);
    document.addEventListener('click', handleInteraction);

    return () => {
      clearInterval(timeInterval);
      clearInterval(calendarInterval);
      document.removeEventListener('keydown', handleInteraction);
      document.removeEventListener('click', handleInteraction);
    };
  }, [onProceed, loadUserSettings, loadOtherCalendarDates]); // Dependencies updated

  // --- Formatting for the main Gregorian display ---
  let mainTimeDisplay = '';
  let mainDateDisplay = '';

  if (userSettings) {
    const showSeconds = userSettings.show_seconds ?? false; // Default to false if null
    const dateFormat = userSettings.date_format ?? 'standard'; // Default to standard if null

    // Determine time format options
    let timeOptions: Intl.DateTimeFormatOptions = {
      hour: '2-digit',
      minute: '2-digit',
    };
    if (showSeconds) {
      timeOptions.second = '2-digit';
    }

    if (dateFormat === 'military') {
      timeOptions.hourCycle = 'h23'; // 24-hour format
      timeOptions.hour12 = false; // Ensure 12-hour is off
    } else if (dateFormat === 'standard') {
      timeOptions.hour12 = true; // 12-hour format with AM/PM
    } else if (dateFormat.startsWith('custom:')) {
      // For custom, we rely on the Rust backend for the full string or more complex parsing.
      // For now, if custom is chosen, we might still fall back to standard/military logic for live ticking.
      // A more robust solution for 'custom:' might require parsing the custom format string in JS.
      // For simplicity here, we'll make a decision: let custom use military/standard depending on its content.
      // Or simply, we might just display the raw time part if it's "custom" and we expect backend to send it.
      // For ticking, the frontend must format it. Let's assume custom formats are handled by Rust for static,
      // and live ticking falls back to military/standard.
      timeOptions.hourCycle = 'h23'; // Assume 24-hour for custom unless specified
      timeOptions.hour12 = false;
    }


    mainTimeDisplay = currentTime.toLocaleTimeString('en-US', timeOptions);

    // Determine date format options
    let dateOptions: Intl.DateTimeFormatOptions = {
      weekday: 'long',
      month: 'long',
      day: 'numeric',
      year: 'numeric',
    };

    // For simplicity, we'll keep date formatting using toLocaleDateString,
    // as the `dateFormat` setting primarily influences time.
    // If you have "custom:..." formats for DATE that need live updates,
    // that would require more complex parsing of the chrono format string in JS.
    mainDateDisplay = currentTime.toLocaleDateString('en-US', dateOptions);

  } else {
    // Fallback if settings are not loaded yet
    mainDateDisplay = currentTime.toLocaleDateString('en-US', {
      weekday: 'long',
      month: 'long',
      day: 'numeric',
      year: 'numeric',
    });
    mainTimeDisplay = currentTime.toLocaleTimeString('en-US', {
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit',
      hour12: false, // Default to 24-hour for fallback
    });
  }


  // Find French Revolutionary calendar info
  const frenchRevolutionaryInfo = calendarDates.find(
    (cal) =>
      cal.system.toLowerCase().includes('french') ||
      cal.system.toLowerCase().includes('revolutionary') ||
      cal.system.toLowerCase().includes('republican'),
  );

  return (
    <div className="calendar-screen">
      <div className="calendar-content">
        {/* Large time display */}
        <div className="time-display-large">{mainTimeDisplay}</div>

        {/* Date display */}
        <div className="date-display">{mainDateDisplay}</div>

        {/* French Revolutionary calendar info if available */}
        {frenchRevolutionaryInfo && (
          <div className="revolutionary-calendar">
            <div className="revolutionary-date">
              {frenchRevolutionaryInfo.date}
            </div>
            {frenchRevolutionaryInfo.additional_info && (
              <div className="revolutionary-info">
                {frenchRevolutionaryInfo.additional_info}
              </div>
            )}
          </div>
        )}

        {/* Other calendar systems (excluding Gregorian and French Revolutionary) */}
        {!isLoading && calendarDates.length > 0 && (
          <div className="additional-calendars">
            {calendarDates
              .filter(
                (cal) =>
                  !cal.system.toLowerCase().includes('french') &&
                  !cal.system.toLowerCase().includes('revolutionary') &&
                  !cal.system.toLowerCase().includes('republican') &&
                  !cal.system.toLowerCase().includes('gregorian'), // Exclude Gregorian here
              )
              .map((calendar, index) => (
                <div key={index} className="calendar-item">
                  <span className="calendar-system">{calendar.system}:</span>
                  <span className="calendar-date-small">{calendar.date}</span>
                  {calendar.additional_info && (
                    <span className="calendar-additional">
                      {calendar.additional_info}
                    </span>
                  )}
                </div>
              ))}
          </div>
        )}

        {/* Interaction hint */}
        <div className="interaction-hint">
          <p>Press any key or click to continue</p>
        </div>
      </div>
    </div>
  );
};

export default CalendarScreen;