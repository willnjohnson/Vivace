import React, { useState, useRef, useEffect } from 'react';
import './LoginScreen.css';

interface LoginScreenProps {
  onLogin: (password: string) => Promise<boolean>;
  onTimeout: () => void;
  onEscape: () => void;
  systemName: string;
  avatarPath: string;
}

const LoginScreen: React.FC<LoginScreenProps> = ({ onLogin, onTimeout, onEscape, systemName, avatarPath }) => {
  const [password, setPassword] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState('');
  const passwordInputRef = useRef<HTMLInputElement>(null);
  const timeoutRef = useRef<NodeJS.Timeout | null>(null);

  useEffect(() => {
    // Focus on password input when component mounts
    if (passwordInputRef.current) {
      passwordInputRef.current.focus();
    }

    // Set up 1-minute timeout
    const resetTimeout = () => {
      if (timeoutRef.current) {
        clearTimeout(timeoutRef.current);
      }
      timeoutRef.current = setTimeout(() => {
        onTimeout();
      }, 60000); // 1 minute
    };

    resetTimeout();

    // Handle ESC key and activity tracking
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        onEscape();
      } else {
        resetTimeout();
      }
    };

    const handleActivity = () => {
      resetTimeout();
    };

    // Add event listeners for user activity
    document.addEventListener('keydown', handleKeyDown);
    document.addEventListener('mousemove', handleActivity);
    document.addEventListener('click', handleActivity);

    return () => {
      if (timeoutRef.current) {
        clearTimeout(timeoutRef.current);
      }
      document.removeEventListener('keydown', handleKeyDown);
      document.removeEventListener('mousemove', handleActivity);
      document.removeEventListener('click', handleActivity);
    };
  }, [onTimeout, onEscape]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!password.trim()) return;

    setIsLoading(true);
    setError('');

    try {
      const success = await onLogin(password);
      if (!success) {
        setError('Incorrect password');
        setPassword('');
        if (passwordInputRef.current) {
          passwordInputRef.current.focus();
        }
      }
    } catch (err) {
      setError('Login failed. Please try again.');
      setPassword('');
    } finally {
      setIsLoading(false);
    }
  };

  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter') {
      handleSubmit(e as any);
    } else if (e.key === 'Escape') {
      onEscape();
    }
  };

  return (
    <div className="login-screen">
      <div className="login-container">
        <div className="user-info">
          <div className="avatar-container">
            {avatarPath ? (
              <img src={avatarPath} alt="User Avatar" className="avatar" />
            ) : (
              <div className="avatar-placeholder">
                <span className="avatar-initial">{systemName}</span>
              </div>
            )}
          </div>
          <h1 className="user-name">{systemName}</h1>
        </div>

        <form onSubmit={handleSubmit} className="login-form">
          <div className="password-container">
            <input
              ref={passwordInputRef}
              type="password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              onKeyPress={handleKeyPress}
              placeholder="Password"
              className={`password-input ${error ? 'error' : ''}`}
              disabled={isLoading}
            />
          </div>
          
          {error && <div className="error-message">{error}</div>}
        </form>

        <div className="login-hint">
          <p>Press Enter to sign in • Press ESC to go back</p>
        </div>
      </div>
    </div>
  );
};

export default LoginScreen;