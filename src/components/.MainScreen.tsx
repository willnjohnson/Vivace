import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import './MainScreen.css';

interface CalendarDate {
  system: string;
  date: string;
  additional_info?: string;
}

interface UserSettings {
  password: string;
  background_type: string;
  background_value: string;
  avatar_path: string;
  enabled_calendars: string[];
}

interface MainScreenProps {
  onLogout: () => void;
  settings: UserSettings;
  systemName: string;
}

const MainScreen: React.FC<MainScreenProps> = ({ onLogout, settings, systemName }) => {
  const [currentTime, setCurrentTime] = useState(new Date());
  const [calendarDates, setCalendarDates] = useState<CalendarDate[]>([]);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    // Update time every second
    const timeInterval = setInterval(() => {
      setCurrentTime(new Date());
    }, 1000);

    // Load calendar dates
    loadCalendarDates();

    // Update calendar dates every minute
    const calendarInterval = setInterval(() => {
      loadCalendarDates();
    }, 60000);

    return () => {
      clearInterval(timeInterval);
      clearInterval(calendarInterval);
    };
  }, []);

  const loadCalendarDates = async () => {
    try {
      const dates = await invoke<CalendarDate[]>('get_current_dates');
      setCalendarDates(dates);
    } catch (error) {
      console.error('Failed to load calendar dates:', error);
    } finally {
      setIsLoading(false);
    }
  };

  const formatTime = (date: Date) => {
    return date.toLocaleTimeString('en-US', {
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit',
      hour12: true
    });
  };

  const formatDate = (date: Date) => {
    return date.toLocaleDateString('en-US', {
      weekday: 'long',
      year: 'numeric',
      month: 'long',
      day: 'numeric'
    });
  };

  const handleKeyPress = (e: React.KeyboardEvent) => {
    // ESC key to logout
    if (e.key === 'Escape') {
      onLogout();
    }
  };

  return (
    <div 
      className="main-screen"
      onKeyDown={handleKeyPress}
      tabIndex={0}
    >
      <div className="main-content">
        {/* Header with user info */}
        <header className="main-header">
          <div className="user-info">
            <div className="user-avatar">
              {settings.avatar_path ? (
                <img src={settings.avatar_path} alt="User Avatar" className="avatar-small" />
              ) : (
                <div className="avatar-placeholder-small">
                  <span className="avatar-initial-small">{systemName.charAt(0).toUpperCase()}</span>
                </div>
              )}
            </div>
            <span className="user-name-small">Welcome, {systemName}</span>
          </div>
          <button className="logout-btn" onClick={onLogout} title="Press ESC to logout">
            Sign Out
          </button>
        </header>

        {/* Main time display */}
        <div className="time-display">
          <div className="current-time">
            {formatTime(currentTime)}
          </div>
          <div className="current-date">
            {formatDate(currentTime)}
          </div>
        </div>

        {/* Calendar systems */}
        <div className="calendar-section">
          <h2 className="calendar-title">Calendar Systems</h2>
          {isLoading ? (
            <div className="loading-calendars">
              <div className="spinner-large"></div>
              <p>Loading calendar systems...</p>
            </div>
          ) : (
            <div className="calendar-grid">
              {calendarDates.map((calendar, index) => (
                <div key={index} className="calendar-card">
                  <h3 className="calendar-system-name">{calendar.system}</h3>
                  <div className="calendar-date">{calendar.date}</div>
                  {calendar.additional_info && (
                    <div className="calendar-info">
                      <span className="info-label">Associated Item:</span>
                      <span className="info-value">{calendar.additional_info}</span>
                    </div>
                  )}
                </div>
              ))}
            </div>
          )}
        </div>

        {/* Footer */}
        <footer className="main-footer">
          <p>Press ESC to sign out • Settings managed via JSON configuration</p>
        </footer>
      </div>
    </div>
  );
};

export default MainScreen;