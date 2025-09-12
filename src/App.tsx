import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { register, unregister } from '@tauri-apps/plugin-global-shortcut';
import CalendarScreen from './components/CalendarScreen';
import LoginScreen from './components/LoginScreen';
import './App.css';

interface UserSettings {
  password: string;
  background_type: string;
  background_value: string;
  avatar_path?: string | null;
  enabled_calendars: string[];
  timeout_minutes?: number | null;
  hotkey_combination?: string | null;
  auto_lock_enabled?: boolean | null;
  auto_lock_minutes?: number | null;
  show_seconds?: boolean | null;
  date_format?: string | null;
  theme?: string | null;
  sound_enabled?: boolean | null;
  sound_file?: string | null;
}

type AppScreen = 'calendar' | 'login' | 'hidden';

function App() {
  const [currentScreen, setCurrentScreen] = useState<AppScreen>('calendar');
  const [settings, setSettings] = useState<UserSettings | null>(null);
  const [systemName, setSystemName] = useState('');

  useEffect(() => {
    loadInitialData();
    
    // Cleanup function for hotkeys
    return () => {
      cleanupGlobalHotkeys();
    };
  }, []);

  // Set up global hotkeys when settings are loaded
  useEffect(() => {
    if (settings) {
      setupGlobalHotkeys();
    }
    
    return () => {
      cleanupGlobalHotkeys();
    };
  }, [settings]);

  const loadInitialData = async () => {
    try {
      const [settingsData, realname, username] = await Promise.all([
        invoke<UserSettings>('load_settings'),
        invoke<string>('get_system_realname'),
        invoke<string>('get_system_username')
      ]);
      
      setSettings(settingsData);
      setSystemName(realname || username || 'User');
    } catch (error) {
      console.error('Failed to load initial data:', error);
      // Set default settings if loading fails
      setSettings({
        password: 'password',
        background_type: 'gradient',
        background_value: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)',
        avatar_path: null,
        enabled_calendars: ['gregorian', 'french_revolutionary'],
        timeout_minutes: 1,
        hotkey_combination: 'Alt+L',
        auto_lock_enabled: null,
        auto_lock_minutes: null,
        show_seconds: null,
        date_format: null,
        theme: null,
        sound_enabled: null,
        sound_file: null
      });
      setSystemName('User');
    }
  };

  const setupGlobalHotkeys = async () => {
    try {
      // Clean up any existing hotkeys first
      await cleanupGlobalHotkeys();
      
      // Get the hotkey combination from settings or use default
      const hotkeyCombo = settings?.hotkey_combination || 'Alt+L';
      
      console.log(`Setting up global hotkey: ${hotkeyCombo}`);
      
      // Register the global hotkey
      await register(hotkeyCombo, () => {
        console.log('Global hotkey triggered - showing window');
        setCurrentScreen('calendar');
        showWindow();
      });
      
      console.log(`Global hotkey ${hotkeyCombo} registered successfully`);
      
    } catch (error) {
      console.error('Failed to register global hotkey:', error);
      
      // Fallback to document listener if global hotkey fails
      setupFallbackHotkeys();
    }
  };

  const cleanupGlobalHotkeys = async () => {
    try {
      const hotkeyCombo = settings?.hotkey_combination || 'Alt+L';
      await unregister(hotkeyCombo);
      console.log(`Global hotkey ${hotkeyCombo} unregistered`);
    } catch (error) {
      // It's okay if unregister fails - the hotkey might not have been registered
      console.log('No global hotkey to unregister or unregister failed:', error);
    }
  };

  // Fallback hotkey implementation (only works when app is focused)
  const setupFallbackHotkeys = () => {
    const handleKeyDown = (e: KeyboardEvent) => {
      // ALT + L to show lock screen
      if (e.altKey && e.key.toLowerCase() === 'l') {
        e.preventDefault();
        console.log('Fallback hotkey triggered - showing window');
        setCurrentScreen('calendar');
        showWindow();
      }
    };

    document.addEventListener('keydown', handleKeyDown);
    
    // Store cleanup function
    return () => {
      document.removeEventListener('keydown', handleKeyDown);
    };
  };

  const showWindow = async () => {
    try {
      await invoke('show_window');
    } catch (error) {
      console.error('Failed to show window:', error);
    }
  };

  const hideWindow = async () => {
    try {
      await invoke('hide_window');
    } catch (error) {
      console.error('Failed to hide window:', error);
    }
  };

  const handleProceedToLogin = () => {
    setCurrentScreen('login');
  };

  const handleBackToCalendar = () => {
    setCurrentScreen('calendar');
  };

  const handleLogin = async (password: string): Promise<boolean> => {
    try {
      const isValid = await invoke<boolean>('verify_password', { password });
      if (isValid) {
        await hideWindow();
      }
      return isValid;
    } catch (error) {
      console.error('Login failed:', error);
      return false;
    }
  };

  if (!settings) {
    return (
      <div className="app loading">
        <div className="loading-spinner">Loading...</div>
      </div>
    );
  }

  return (
    <div
      className="app"
      style={{
        background: settings.background_type === 'gradient'
          ? settings.background_value
          : `url(${settings.background_value}) center/cover no-repeat`
      }}
    >
      {currentScreen === 'calendar' ? (
        <CalendarScreen onProceed={handleProceedToLogin} />
      ) : (
        <LoginScreen
          onLogin={handleLogin}
          onTimeout={handleBackToCalendar}
          onEscape={handleBackToCalendar}
          systemName={systemName}
          avatarPath={settings.avatar_path || ''}
        />
      )}
    </div>
  );
}

export default App;